use errno::{set_errno, Errno};
use log::debug;
use std::mem::size_of;

use super::protocol::{get_protocol, get_protocol_next, Data, OptLevels, Opt, Protocol};
use super::socket::{socket_set_error, socket_set_sockopt, Socket};
use super::{get_long, get_short, set_long, set_short, DLPErrorDefinitions, Error};

const CMP_OFFSET_TYPE: usize = 0;
const CMP_OFFSET_FLGS: usize = 1;
const CMP_OFFSET_VERS: usize = 2;
const CMP_OFFSET_RESV: usize = 4;
const CMP_OFFSET_BAUD: usize = 8;

const CMP_HEADER_LEN: usize = 10;

const CMP_VERS_1_0: u16 = 0x0100;
const CMP_VERS_1_1: u16 = 0x0101;
const CMP_VERS_1_2: u16 = 0x0102;
const CMP_VERS_1_3: u16 = 0x0103;
const CMP_VERSION: u16 = CMP_VERS_1_2;

const PAD_DATA: i32 = 0x01;
const PAD_WAKE: i32 = 0x101;
const PAD_ACK: i32 = 0x02;
const PAD_TICKLE: i32 = 0x04;
const PAD_ABORT: i32 = 0x08;

pub fn new_cmp_protocol() -> Protocol {
    return Protocol {
        level: OptLevels::LevelDev,
        data: Data {
            cmp_data_type: 0,
            cmp_flags: 0,
            cmp_version: 0,
            cmp_baudrate: 0,
            padp_data_type: 0,
        },
        read: cmp_rx,
        write: cmp_tx,
        flush: cmp_flush,
        get_sock_opt: cmp_getsockopt,
        set_sock_opt: cmp_setsockopt,
    };
}

fn cmp_rx(ps: &Socket, msg: &mut Vec<u8>, len: usize, flags: i32) -> DLPErrorDefinitions {
    let error: DLPErrorDefinitions;
    let prot: Protocol;
    let next: Protocol;
    let mut data: Data;

    debug!("CMP RX len={} flags=0x{:#02x}\n", len, flags);

    prot = match get_protocol(ps.socket_descriptor, OptLevels::LevelCMP) {
        Some(x) => x,
        None => {
            return socket_set_error(ps.socket_descriptor, DLPErrorDefinitions::ErrSockInvalid)
        }
    };

    data = prot.data;
    next = match get_protocol_next(ps.socket_descriptor, OptLevels::LevelCMP) {
        Some(x) => x,
        None => {
            return socket_set_error(ps.socket_descriptor, DLPErrorDefinitions::ErrSockInvalid)
        }
    };

    error = (next.read)(ps, msg, len, flags);
    if (error as i64) < 10 {
        let err: DLPErrorDefinitions;
        if (error as i64) < 0 {
            err = error;
        } else {
            err = DLPErrorDefinitions::ErrProtAborted;
        }
        return socket_set_error(ps.socket_descriptor, err);
    }
    data.cmp_data_type = msg[CMP_OFFSET_TYPE];
    data.cmp_flags = msg[CMP_OFFSET_FLGS];
    data.cmp_version = get_short(&msg[CMP_OFFSET_VERS..CMP_OFFSET_VERS + 2]);
    data.cmp_baudrate = get_long(&msg[CMP_OFFSET_BAUD..CMP_OFFSET_BAUD + 4]);

    DLPErrorDefinitions::ErrNoErr
}

fn cmp_tx(sock: &mut Socket, _: &Vec<u8>, _: usize, flags: i32) -> DLPErrorDefinitions {
    let cmp_type: i32;
    let error: DLPErrorDefinitions;
    let mut size: usize;
    let prot: Protocol;
    let next: Protocol;
    let data: Data;
    let mut cmp_buf: Vec<u8>;

    cmp_buf = Vec::with_capacity(CMP_HEADER_LEN);

    prot = match get_protocol(sock.socket_descriptor, OptLevels::LevelCMP) {
        Some(x) => x,
        None => {
            return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrSockInvalid)
        }
    };

    data = prot.data;
    next = match get_protocol_next(sock.socket_descriptor, OptLevels::LevelCMP) {
        Some(x) => x,
        None => {
            return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrSockInvalid)
        }
    };

    cmp_type = PAD_DATA;
    size = size_of::<i32>();
    socket_set_sockopt(sock.socket_descriptor, OptLevels::LevelPADP, Opt::PADPType, &(cmp_type as i64), &mut size);

    cmp_buf[CMP_OFFSET_TYPE] = data.cmp_data_type;
    cmp_buf[CMP_OFFSET_FLGS] = data.cmp_flags;
    if data.cmp_version > CMP_VERSION {
        set_short(&mut cmp_buf, CMP_OFFSET_VERS, CMP_VERSION)
    } else {
        set_short(&mut cmp_buf, CMP_OFFSET_VERS, data.cmp_version)
    }
    set_short(&mut cmp_buf, CMP_OFFSET_RESV, 0);
    set_long(&mut cmp_buf, CMP_OFFSET_BAUD, data.cmp_baudrate);

    error = (next.write)(sock, &cmp_buf, CMP_HEADER_LEN, flags);
    if (error as i64) < 10 {
        if (error as i64) < 0 {
            return error
        } else {
            return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrProtAborted);
        }
    }

    DLPErrorDefinitions::ErrNoErr
}

fn cmp_flush(sock: &Socket, flags: i32) -> DLPErrorDefinitions {
	let next: Protocol;

    match get_protocol(sock.socket_descriptor, OptLevels::LevelCMP) {
        Some(x) => x,
        None => {
            return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrSockInvalid)
        }
    };

    next = match get_protocol_next(sock.socket_descriptor, OptLevels::LevelCMP) {
        Some(x) => x,
        None => {
            return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrSockInvalid)
        }
    };

	return (next.flush)(sock, flags);
}

fn cmp_getsockopt(
    sock: &Socket,
    level: OptLevels,
    option_name: Opt,
    option_value: &mut i64,
    option_len: &mut usize,
) -> DLPErrorDefinitions {
    let prot: Protocol;
    let mut data: Data;

    prot = match get_protocol(sock.socket_descriptor, OptLevels::LevelCMP) {
        Some(x) => x,
        None => {
            return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrSockInvalid)
        }
    };
    data = prot.data;

    match option_name {
        CMPType => {
            if *option_len != size_of::<u8>() {
                set_errno(Errno(Error::EINVAL as i32));
                return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrGenericArgument);
            }
            data.cmp_data_type = *option_value as u8;
            *option_len = size_of::<u8>();
        },
        CMPFlags => {
            if *option_len != size_of::<u8>() {
                set_errno(Errno(Error::EINVAL as i32));
                return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrGenericArgument);
            }
            data.cmp_flags = *option_value as u8;
            *option_len = size_of::<u8>();
        },
        CMPVers => {
            if *option_len != size_of::<u16>() {
                set_errno(Errno(Error::EINVAL as i32));
                return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrGenericArgument);
            }
            data.cmp_version = *option_value as u16;
            *option_len = size_of::<u16>();
        },
        CMPBaud => {
            if *option_len != size_of::<u64>() {
                set_errno(Errno(Error::EINVAL as i32));
                return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrGenericArgument);
            }
            data.cmp_baudrate = *option_value as u64;
            *option_len = size_of::<u64>();
        }
    };

	DLPErrorDefinitions::ErrNoErr
}

fn cmp_setsockopt(
    sock: &Socket,
    level: OptLevels,
    option_name: Opt,
    option_value: &i64,
    option_len: &mut usize,
) -> DLPErrorDefinitions {
    let prot: Protocol;
    let mut data: Data;
    
    prot = match get_protocol(sock.socket_descriptor, OptLevels::LevelCMP) {
        Some(x) => x,
        None => {
            return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrSockInvalid)
        }
    };
    data = prot.data;

	if option_name == Opt::PADPType {
        if *option_len != size_of::<u8>() {
            set_errno(Errno(Error::EINVAL as i32));
            return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrGenericArgument);
        }
        data.padp_data_type = *option_value as u8;
        *option_len = size_of::<u8>();
	}

    DLPErrorDefinitions::ErrNoErr
}

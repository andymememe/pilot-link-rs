use errno::{set_errno, Errno};
use log::debug;
use std::mem::size_of;

use super::padp::{PADP_DATA};
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

const CMP_TYPE_WAKE: u8 = 0x01;
const CMP_TYPE_INIT: u8 = 0x02;
const CMP_TYPE_ABRT: u8 = 0x03;
const CMP_TYPE_EXTN: u8 = 0x04;

/* CMP packet flag values */
const CMP_FL_CHANGE_BAUD_RATE: u8 = 0x80; // < Want to switch speeds
const CMP_FL_ONE_MINUTE_TIMEOUT: u8 = 0x40; // < Use a 1 minute timeout before dropping link
const CMP_FL_TWO_MINUTE_TIMEOUT: u8 = 0x20;	// < Use a 2 minute timeout before dropping ling
const CMP_FL_LONG_PACKET_SUPPORT: u8 = 0x10; // < long PADP packet format is supported

pub fn new_cmp_protocol() -> Protocol {
    return Protocol {
        level: OptLevels::LevelDev,
        data: Data {
            data_type: 0,
            cmp_flags: 0,
            cmp_version: 0,
            cmp_baudrate: 0,
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
    data.data_type = msg[CMP_OFFSET_TYPE];
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

    cmp_type = PADP_DATA;
    size = size_of::<i32>();
    socket_set_sockopt(sock.socket_descriptor, OptLevels::LevelPADP, Opt::PADPType, &(cmp_type as i64), &mut size);

    cmp_buf[CMP_OFFSET_TYPE] = data.data_type;
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
    _: OptLevels,
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
        Opt::CMPType => {
            if *option_len != size_of::<u8>() {
                set_errno(Errno(Error::EINVAL as i32));
                return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrGenericArgument);
            }
            data.data_type = *option_value as u8;
            *option_len = size_of::<u8>();
        },
        Opt::CMPFlags => {
            if *option_len != size_of::<u8>() {
                set_errno(Errno(Error::EINVAL as i32));
                return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrGenericArgument);
            }
            data.cmp_flags = *option_value as u8;
            *option_len = size_of::<u8>();
        },
        Opt::CMPVers => {
            if *option_len != size_of::<u16>() {
                set_errno(Errno(Error::EINVAL as i32));
                return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrGenericArgument);
            }
            data.cmp_version = *option_value as u16;
            *option_len = size_of::<u16>();
        },
        Opt::CMPBaud => {
            if *option_len != size_of::<u64>() {
                set_errno(Errno(Error::EINVAL as i32));
                return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrGenericArgument);
            }
            data.cmp_baudrate = *option_value as u64;
            *option_len = size_of::<u64>();
        },
        _ => {}
    };

	DLPErrorDefinitions::ErrNoErr
}

fn cmp_setsockopt(
    sock: &Socket,
    _: OptLevels,
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
        data.data_type = *option_value as u8;
        *option_len = size_of::<u8>();
	}

    DLPErrorDefinitions::ErrNoErr
}

fn cmp_rx_handshake(sock: &mut Socket, establish_rate: i64, establish_high_rate: i64) -> DLPErrorDefinitions {
    let prot: Protocol;
	let mut buf: Vec<u8>;
    let mut data: Data;
	let mut error: DLPErrorDefinitions;
	
	prot = match get_protocol(sock.socket_descriptor, OptLevels::LevelCMP) {
        Some(x) => x,
        None => {
            return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrSockInvalid)
        }
    };
    data = prot.data;

    /* Read the cmp packet */
    buf = Vec::with_capacity(CMP_HEADER_LEN);
    error = cmp_rx(sock, &mut buf, CMP_HEADER_LEN, 0);
    
	if (error as i8) < 0 {
        return error;
    }

	if (data.cmp_version & 0xFF00) == 0x0100 {
		if establish_rate != -1 {
			if (establish_rate as u64) > data.cmp_baudrate {
				if establish_high_rate != 0 {
					data.cmp_baudrate = establish_rate as u64;
				}
			} else {
				data.cmp_baudrate = establish_rate as u64;
			}
		}
        
        error = cmp_init(sock, data.cmp_baudrate);
		if (error as i8) < 0 {
            return error;
        }
	} else {
		/* 0x80 means the comm version wasn't compatible */
		cmp_abort(sock, 0x80);
        set_errno(Errno(Error::ECONNREFUSED as i32));
        return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrProtIncompatible);
	}

	return DLPErrorDefinitions::ErrNoErr;
}

fn cmp_tx_handshake(sock: &mut Socket) -> DLPErrorDefinitions {
    let prot: Protocol;
    let data: Data;
	let mut result: DLPErrorDefinitions;
	
	prot = match get_protocol(sock.socket_descriptor, OptLevels::LevelCMP) {
        Some(x) => x,
        None => {
            return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrSockInvalid)
        }
    };
    data = prot.data;

    result = cmp_wakeup(sock, 38400);
	if (result as i8) < 0 {	/* Assume box can't go over 38400 */
        return result;
    }

    result = cmp_rx(sock, &mut vec![], 0, 0);
	if (result as i8) < 0 {
		return result /* failed to read, errno already set */
    }

    match data.data_type {
        CMP_TYPE_INIT => return DLPErrorDefinitions::ErrNoErr,
        CMP_TYPE_ABRT => {
            set_errno(Errno(Error::EIO as i32));
            return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrProtAborted)
        },
        _ => {}
    }

	return DLPErrorDefinitions::ErrProtIncompatible;
}

fn cmp_init(sock: &mut Socket, baudrate: u64) -> DLPErrorDefinitions {
    let prot: Protocol;
    let mut data: Data;
	
	prot = match get_protocol(sock.socket_descriptor, OptLevels::LevelCMP) {
        Some(x) => x,
        None => {
            return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrSockInvalid)
        }
    };
    data = prot.data;

	data.data_type = CMP_TYPE_INIT;
	data.cmp_flags = CMP_FL_LONG_PACKET_SUPPORT;
	data.cmp_version = CMP_VERSION;
	data.cmp_baudrate = baudrate;

	return cmp_tx(sock, &vec![], 0, 0);
}

fn cmp_abort(sock: &mut Socket, reason: u8) -> DLPErrorDefinitions {
    let prot: Protocol;
    let mut data: Data;
	
	prot = match get_protocol(sock.socket_descriptor, OptLevels::LevelCMP) {
        Some(x) => x,
        None => {
            return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrSockInvalid)
        }
    };
    data = prot.data;

	data.data_type = CMP_TYPE_ABRT;
	data.cmp_flags = reason;

	return cmp_tx(sock, &vec![], 0, 0);
}

fn cmp_wakeup(sock: &mut Socket, max_baud: u64) -> DLPErrorDefinitions {
    let prot: Protocol;
    let mut data: Data;
	
	prot = match get_protocol(sock.socket_descriptor, OptLevels::LevelCMP) {
        Some(x) => x,
        None => {
            return socket_set_error(sock.socket_descriptor, DLPErrorDefinitions::ErrSockInvalid)
        }
    };
    data = prot.data;

	data.data_type = CMP_TYPE_WAKE;
	data.cmp_flags = 0;
	data.cmp_version = CMP_VERSION;
	data.cmp_baudrate = max_baud;

	return cmp_tx(sock, &vec![], 0, 0);
}
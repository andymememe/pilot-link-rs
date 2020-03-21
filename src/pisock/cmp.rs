use log::debug;

use super::protocol::{get_protocol, get_protocol_next, Data, OptLevels, Protocol};
use super::socket::{socket_set_error, Socket};
use super::{get_long, get_short, DLPErrorDefinitions};

const PI_CMP_OFFSET_TYPE: usize = 0;
const PI_CMP_OFFSET_FLGS: usize = 1;
const PI_CMP_OFFSET_VERS: usize = 2;
const PI_CMP_OFFSET_RESV: usize = 4;
const PI_CMP_OFFSET_BAUD: usize = 6;

pub fn new_cmp_protocol() -> Protocol {
    return Protocol {
        level: OptLevels::LevelDev,
        data: Data {
            cmp_data_type: 0,
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

fn cmp_rx(ps: &Socket, msg: &mut Vec<u8>, len: usize, flags: i32) -> i64 {
    let bytes: i64;
    let prot: Protocol;
    let next: Protocol;
    let mut data: Data;

    debug!("CMP RX len={} flags=0x{:#02x}\n", len, flags);

    prot = match get_protocol(ps.socket_descriptor, OptLevels::LevelCMP) {
        Some(x) => x,
        None => {
            return socket_set_error(ps.socket_descriptor, DLPErrorDefinitions::ErrSockInvalid)
                as i64
        }
    };

    data = prot.data;
    next = match get_protocol_next(ps.socket_descriptor, OptLevels::LevelCMP) {
        Some(x) => x,
        None => {
            return socket_set_error(ps.socket_descriptor, DLPErrorDefinitions::ErrSockInvalid)
                as i64
        }
    };

    bytes = (next.read)(ps, msg, len, flags);
    if bytes < 10 {
        let err: DLPErrorDefinitions;
        if bytes < 0 {
            err = match num::FromPrimitive::from_i64(bytes) {
                Some(x) => x,
                None => DLPErrorDefinitions::ErrProtAborted,
            };
        } else {
            err = DLPErrorDefinitions::ErrProtAborted;
        }
        return socket_set_error(ps.socket_descriptor, err) as i64;
    }
    data.cmp_data_type = msg[PI_CMP_OFFSET_TYPE];
    data.cmp_flags = msg[PI_CMP_OFFSET_FLGS];
    data.cmp_version = get_short(&msg[PI_CMP_OFFSET_VERS..PI_CMP_OFFSET_VERS + 2]);
    data.cmp_baudrate = get_long(&msg[PI_CMP_OFFSET_BAUD..PI_CMP_OFFSET_BAUD + 4]);

    0
}

fn cmp_tx(ps: &mut Socket, buf: &Vec<u8>, len: usize, flags: i32) -> i64 {
    let bytes: i128;
    let prot: Protocol;
    let next: Protocol;
    let mut data: Data;

    0
}

fn cmp_flush(ps: &Socket, flags: i32) -> i32 {
    0
}

fn cmp_getsockopt(
    ps: &Socket,
    level: OptLevels,
    option_name: i32,
    option_value: &mut Data,
    option_len: usize,
) -> i32 {
    0
}

fn cmp_setsockopt(
    ps: &Socket,
    level: OptLevels,
    option_name: i32,
    option_value: &Data,
    option_len: usize,
) -> i32 {
    0
}

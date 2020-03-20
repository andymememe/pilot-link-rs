use log::debug;

use super::Data;
use super::protocol::{
    Protocol,
    OptLevels,
    get_protocol
};
use super::socket::{
    Socket
};

#[derive(Clone, Copy)]
pub struct CMPData {
    pub cmp_data_type: u8,
    pub flags: u8,
    pub version: u64,
    pub baudrate: i64,
}

impl Data for CMPData {

}

pub fn new_cmp_protocol() -> Protocol<CMPData>
{
	return Protocol::<CMPData> {
        level: 0,
        data: CMPData {
            cmp_data_type: 0,
            flags: 0,
            version: 0,
            baudrate: 0
        },
        read: cmp_rx,
        write: cmp_tx,
        flush: cmp_flush,
        get_sock_opt: cmp_getsockopt,
        set_sock_opt: cmp_setsockopt
    }
}

fn cmp_rx(ps: &Socket<CMPData>, buf: &mut Vec<u8>, len: usize, flags: i64) -> i128 {
    let byte: i64;
    let prot: Protocol<CMPData>;
    let next: Protocol<CMPData>;
    let data: CMPData;

    debug!("CMP RX len={} flags=0x{:#02x}\n", len, flags);

    prot = get_protocol(ps.socket_descriptor, OptLevels::PI_LEVEL_CMP).unwrap();

	0
}

fn cmp_tx(ps: &mut Socket<CMPData>, buf: &Vec<u8>, len: usize, flags: i64) -> i128 {
    0
}

fn cmp_flush(ps: &Socket<CMPData>, flags: i64) -> i64 {
    0
}

fn cmp_getsockopt(ps: &Socket<CMPData>, level: i64, option_name: i64, option_value: &mut CMPData, option_len: usize) -> i64 {
    0
}

fn cmp_setsockopt(ps: &Socket<CMPData>, level: i64, option_name: i64, option_value: &CMPData, option_len: usize) -> i64 {
    0
}
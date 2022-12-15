//! SLP: Serial Link Protocol
//! 
//! The Serial Link Protocol (SLP) provides an efficient packet send and receive mechanism that is used by the Palm desktop software and debugger.
//! SLP provides robust error detection with CRC-16.
//! SLP is a best-effort protocol; it does not guarantee packet delivery (packet delivery is left to the higher-level protocols).
//! For enhanced error detection and implementation convenience of higher-level protocols, SLP specifies packet type, source, destination, and transaction ID information as an integral part of its data packet structure.

use std::any::Any;

use super::{base::Protocol, socket::{OptLevel, Socket}};

const SLP_HEADER_LEN: i32 = 10;
const SLP_FOOTER_LEN: i32 = 2;
const SLP_MTU: i32 = 0xffff;

const SLP_SIG_BYTE1: i32 = 0xbe;
const SLP_SIG_BYTE2: i32 = 0xef;
const SLP_SIG_BYTE3: i32 = 0xed;

const SLP_OFFSET_SIG1: i32 = 0;
const SLP_OFFSET_SIG2: i32 = 1;
const SLP_OFFSET_SIG3: i32 = 2;
const SLP_OFFSET_DEST: i32 = 3;
const SLP_OFFSET_SRC: i32 = 4;
const SLP_OFFSET_TYPE: i32 = 5;
const SLP_OFFSET_SIZE: i32 = 6;
const SLP_OFFSET_TXID: i32 = 8;
const SLP_OFFSET_SUM: i32 = 9;

const SLP_SOCK_DBG: i32 = 0x00;
const SLP_SOCK_CON: i32 = 0x01;
const SLP_SOCK_RUI: i32 = 0x02;
const SLP_SOCK_DLP: i32 = 0x03;

const SLP_TYPE_RDCP: i32 = 0x00;
const SLP_TYPE_PADP: i32 = 0x02;
const SLP_TYPE_LOOP: i32 = 0x03;

struct SLPData {
    destination: i32,
    last_destination: i32,
    source: i32,
    last_source: i32,
    slp_type: i32,
    last_slp_type: i32,
    tx_id: u8,
    last_tx_id: u8,
}

pub fn slp_new() -> Protocol {
    Protocol {
        level: OptLevel::LevelSLP,
        data: Box::new(&SLPData{
            destination: SLP_SOCK_DLP,
            last_destination: -1,
            source: SLP_SOCK_DLP,
            last_source: -1,
            slp_type: SLP_TYPE_PADP,
            last_slp_type: -1,
            tx_id: 0xfe,
            last_tx_id: 0xff,
        }),
        dup: slp_dup,
        read: slp_rx,
        write: slp_tx,
        flush: slp_flush,
        get_socket_opt: slp_get_socket_opt,
        set_socket_opt: slp_set_socket_opt,
    }
}

pub fn slp_dup(ps: &'static Protocol) -> Protocol {
    let data: &SLPData;
    let new_data: SLPData;

    data = ps.data.downcast_ref::<SLPData>().expect("need to be a SLPData");

    new_data = SLPData {
        destination: data.destination,
        last_destination: data.last_destination,
        source: data.source,
        last_source: data.last_source,
        slp_type: data.slp_type,
        last_slp_type: data.last_slp_type,
        tx_id: data.tx_id,
        last_tx_id: data.last_tx_id,
    };

    Protocol {
        level: ps.level,
        data: Box::new(new_data),
        dup: ps.dup,
        read: ps.read,
        write: ps.write,
        flush: ps.flush,
        get_socket_opt: ps.get_socket_opt,
        set_socket_opt: ps.set_socket_opt,
    }
}

pub fn slp_rx(socket: &Socket, buf: &Vec<u8>, length: usize, flags: i32) -> isize {
    let i: i32;
	let computed_crc: i32;
	let received_crc: i32;
	let b1: i32; 
	let b2: i32;
	let b3: i32;
	let state: i32;
	let packet_len: i32;
	let bytes: i32;
	let header_checksum: u8;
	let protocol: Protocol;
	let next: Protocol;
	let slp_buf: Vec<u8>;
	let data: SLPData;
	let expect= 0;

    0
}

pub fn slp_tx(socket: &Socket, buf: &Vec<u8>, length: usize, flags: i32) -> isize {
    0
}

pub fn slp_flush(ps: &Socket, flags: i32) -> i32 {
    0
}

pub fn slp_get_socket_opt(ps: &Socket, level: i32, option_name: i32, option_value: Box<&dyn Any>, option_len: &usize) -> i32 {
    0
}

pub fn slp_set_socket_opt(ps: &Socket, level: i32, option_name: i32, option_value: Box<&dyn Any>, option_len: &usize) -> i32 {
    0
}
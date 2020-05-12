use errno::{set_errno, Errno};

use super::{Error, DLPErrorDefinitions};
use super::socket::{find_socket, Socket};

#[derive(Clone, Copy, PartialEq)]
pub enum OptLevels {
    LevelDev,  // Device level
    LevelSLP,  // Serial link protocol level
    LevelPADP, // PADP protocol level
    LevelNet,  // NET protocol level
    LevelSys,  // System protocol level
    LevelCMP,  // CMP protocol level
    LevelDLP,  // Desktop link protocol level
    LevelSock, // Socket level
}

#[derive(Clone, Copy, PartialEq)]
pub enum Opt {
    // PDAP
    PADPType,
    PADPLastType,
    PADPFreezeTXID,
    PADPUseLongFormat,

    // Socket
    SocketState,			// Socket state (listening, closed, etc.)
    SocketHonorRXTimeout,	// Set to 1 to honor timeouts when waiting for data. Set to 0 to disable timeout (i.e. during dlp_call_application)

    // CMP
	CMPType,
	CMPFlags,
	CMPVers,
	CMPBaud
}

#[derive(Clone)]
pub struct Data {
    // CMP
    pub cmp_data_type: u8,
    pub cmp_flags: u8,
    pub cmp_version: u16,
    pub cmp_baudrate: u64,

    // PADP
    pub padp_data_type: u8,
}

#[derive(Clone)]
pub struct Protocol {
    pub level: OptLevels,
    pub data: Data,
    pub read: fn(&Socket, &mut Vec<u8>, usize, i32) -> DLPErrorDefinitions,
    pub write: fn(&mut Socket, &Vec<u8>, usize, i32) -> DLPErrorDefinitions,
    pub flush: fn(&Socket, i32) -> DLPErrorDefinitions,
    pub get_sock_opt: fn(&Socket, OptLevels, Opt, &mut i64, &mut usize) -> DLPErrorDefinitions,
    pub set_sock_opt: fn(&Socket, OptLevels, Opt, &i64, &mut usize) -> DLPErrorDefinitions,
}

pub fn get_protocol(sd: i32, level: OptLevels) -> Option<Protocol> {
    match find_socket(sd) {
        Some(x) => protocol_queue_find(&x, level),
        None => {
            set_errno(Errno(Error::ESRCH as i32));
            None
        },
    }
}

pub fn get_protocol_next (sd: i32, level: OptLevels) -> Option<Protocol> {
    match find_socket(sd) {
        Some(x) => protocol_queue_find_next(&x, level),
        None => {
            set_errno(Errno(Error::ESRCH as i32));
            None
        },
    }
}

pub fn protocol_queue_find(ps: &Socket, level: OptLevels) -> Option<Protocol> {
    if ps.command != 0 {
        for i in 0..ps.cmd_len {
            if ps.cmd_queue[i].level == level {
                return Some(ps.cmd_queue[i].clone());
            }
        }
    } else {
        for i in 0..ps.queue_len {
            if ps.protocol_queue[i].level == level {
                return Some(ps.protocol_queue[i].clone());
            }
        }
    }

    None
}

pub fn protocol_queue_find_next(ps: &Socket, level: OptLevels) -> Option<Protocol> {
    if ps.command != 0 && ps.cmd_len == 0 {
        return None
    }

    if ps.command == 0 && ps.queue_len == 0 {
        return None
    }

    if ps.command != 0 && level == OptLevels::LevelDev {
        return Some(ps.cmd_queue[0].clone())
    }

    if ps.command == 0 && level == OptLevels::LevelDev {
        return Some(ps.protocol_queue[0].clone())
    }

    if ps.command != 0 {
        for i in 0..(ps.cmd_len - 1) {
            if ps.cmd_queue[i].level == level {
                return Some(ps.cmd_queue[i + 1].clone());
            }
        }
    } else {
        for i in 0..(ps.queue_len - 1) {
            if ps.protocol_queue[i].level == level {
                return Some(ps.protocol_queue[i + 1].clone());
            }
        }
    }

    None
}

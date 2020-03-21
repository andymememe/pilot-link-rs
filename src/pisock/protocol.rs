use errno::{set_errno, Errno};

use super::Error;
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

#[derive(Clone)]
pub struct Data {
    // CMP
    pub cmp_data_type: u8,
    pub cmp_flags: u8,
    pub cmp_version: u16,
    pub cmp_baudrate: u64,
}

#[derive(Clone)]
pub struct Protocol {
    pub level: OptLevels,
    pub data: Data,
    pub read: fn(&Socket, &mut Vec<u8>, usize, i64) -> i128,
    pub write: fn(&mut Socket, &Vec<u8>, usize, i64) -> i128,
    pub flush: fn(&Socket, i64) -> i64,
    pub get_sock_opt: fn(&Socket, i64, i64, &mut Data, usize) -> i64,
    pub set_sock_opt: fn(&Socket, i64, i64, &Data, usize) -> i64,
}

pub fn get_protocol(sd: i64, level: OptLevels) -> Option<Protocol> {
    match find_socket(sd) {
        Some(x) => protocol_queue_find(&x, level),
        None => {
            set_errno(Errno(Error::ESRCH as i32));
            None
        },
    }
}

pub fn get_protocol_next (sd: i64, level: OptLevels) -> Option<Protocol> {
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

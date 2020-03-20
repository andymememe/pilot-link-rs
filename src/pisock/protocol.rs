use super::Data;
use super::socket::{
    Socket,
    SocketList,
    find_socket
};

#[derive(PartialEq, FromPrimitive)]
pub enum OptLevels {
	PI_LEVEL_DEV,			// Device level
	PI_LEVEL_SLP,			// Serial link protocol level
	PI_LEVEL_PADP,			// PADP protocol level
	PI_LEVEL_NET,			// NET protocol level
	PI_LEVEL_SYS,			// System protocol level
	PI_LEVEL_CMP,			// CMP protocol level
	PI_LEVEL_DLP,			// Desktop link protocol level
	PI_LEVEL_SOCK			// Socket level
}

#[derive(Clone, Copy)]
pub struct Protocol<T: Data> {
    pub level: i64,
    pub data: T,
	pub read: fn(&Socket<T>, &mut Vec<u8>, usize, i64) -> i128,
    pub write: fn(&mut Socket<T>, &Vec<u8>, usize, i64) -> i128,
    pub flush: fn(&Socket<T>, i64) -> i64,
    pub get_sock_opt: fn(&Socket<T>, i64, i64, &mut T, usize) -> i64,
    pub set_sock_opt: fn(&Socket<T>, i64, i64, &T, usize) -> i64
}

pub fn get_protocol<T: Data>(sd: i64, level: OptLevels) -> Option<Protocol<T>> {
    match find_socket::<T>(sd) {
        Some(x) => protocol_queue_find::<T>(&x, level),
        None => None
    }
}

pub fn protocol_queue_find<T: Data>(ps: &Socket<T>, level: OptLevels) -> Option<Protocol<T>> {
    if ps.command != 0 {
        for i in 0..ps.cmd_len {
            if ps.cmd_queue[i].level == level as i64 {
                return Some(ps.cmd_queue[i])
            }
        }
    } else {
        for i in 0..ps.queue_len {
            if ps.protocol_queue[i].level == level as i64 {
                return Some(ps.protocol_queue[i])
            }
        }
    }

    None
}
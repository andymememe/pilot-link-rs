use errno::{set_errno, Errno};

use super::protocol::Protocol;
use super::{DLPErrorDefinitions, Error};

use std::collections::LinkedList;
use std::sync::Mutex;

pub type SocketList = LinkedList<Socket>;
pub static SOCKET_LIST: SocketList = SocketList::new();

#[derive(Clone)]
pub struct Device {}

#[derive(Clone)]
pub struct Socket {
    pub socket_descriptor: i64,

    pub socket_type: i64,
    pub protocol: i64,
    pub cmd: i64,
    // Local Socket
    // pub laddr: SocketAddress,
    pub laddrlen: usize,
    // Remote Socket
    // pub raddr: SocketAddress,
    pub raddrlen: usize,
    // Protocol Queue
    pub protocol_queue: Vec<Protocol>,
    pub queue_len: usize,
    // Command Queue
    pub cmd_queue: Vec<Protocol>,
    pub cmd_len: usize,
    pub device: Device,
    pub socket_state: i64,

    // Honor packet reception timeouts
    // Set most to 1 of the time to have timeout management on incoming packets.
    // Can be disabled when needed using pi_setsockopt() with #PI_SOCK_HONOR_RX_TIMEOUT.
    // This is used, for example,
    // to disable timeouts in dlp_CallApplication()
    // so that lengthy tasks don't return an error.
    pub honor_rx_to: i64,
    pub command: i64,
    pub accept_to: i64,
    // DLP
    pub dlprecord: i64,
    pub dlpversion: i64,
    pub maxrecsize: u64,
    // Error code
    pub last_error: DLPErrorDefinitions,
    pub palmos_error: i32,
}

pub fn find_socket(sd: i64) -> Option<Socket> {
    let mutex = Mutex::new(&SOCKET_LIST);
    {
        let list = mutex.lock().unwrap();
        socket_list_search(*list, sd)
    }
}

pub fn socket_list_search(a_socket_list: &SocketList, sd: i64) -> Option<Socket> {
    let mut iter = a_socket_list.iter();
    let res = iter.next();
    while !res.is_none() {
        let ele = res.unwrap();
        if ele.socket_descriptor == sd {
            return Some((*ele).clone());
        }
    }

    None
}

pub fn socket_set_error(sd: i64, error_code: DLPErrorDefinitions) -> DLPErrorDefinitions {
    let mut ps: Socket;
    match find_socket(sd) {
        Some(x) => {
            ps = x;
            ps.last_error = error_code;
        }
        None => {
            set_errno(Errno(Error::ESRCH as i32));
        }
    }

    if error_code == DLPErrorDefinitions::ErrGenericMemory {
        set_errno(Errno(Error::ENOMEM as i32));
    }

    return error_code;
}

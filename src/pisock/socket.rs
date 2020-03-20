use super::Data;
use super::protocol::Protocol;

use std::sync::Mutex;
use std::collections::LinkedList;

pub type SocketList<T> = LinkedList<Socket<T>>;

pub struct Device {

}

pub struct Socket<T: Data + Copy> {
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
	pub protocol_queue: Vec<Protocol<T>>,
    pub queue_len: usize,
    
    // Command Queue
	pub cmd_queue: Vec<Protocol<T>>,
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
	pub last_error: i64,
	pub palmos_error: i64
}

pub fn find_socket<T: Data + Copy>(socket_list: &SocketList<T>, sd: i64) -> Option<&Socket<T>> {
    let mutex = Mutex::new(socket_list);
    {
        let list = mutex.lock().unwrap();
        socket_list_find::<T>(*list, sd)
    }
}

pub fn socket_list_find<T: Data + Copy>(a_socket_list: &SocketList<T>, sd: i64) -> Option<&Socket<T>> {
    let mut iter = a_socket_list.iter();
    let res = iter.next();
    while !res.is_none() {
        let ele = res.unwrap();
        if ele.socket_descriptor == sd {
            return Some(ele);
        }
    }

    None
}
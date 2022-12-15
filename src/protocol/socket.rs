use std::net::SocketAddr;

use super::base::{Protocol, Device};

static pilot_socket_list: Option<&'static SocketList<'static>> = None;

#[derive(Clone, Copy)]
pub enum OptLevel {
	LevelDev,			// Device level
	LevelSLP,			// Serial link protocol level
	LevelPADP,			// PADP protocol level
	LevelNET,			// NET protocol level
	LevelSys,			// System protocol level
	LevelCMP,			// CMP protocol level
	LevelDLP,			// Desktop link protocol 
	LevelSocket,			// Socket level
}

pub struct Socket<'a> {
    pub sd: i32, // Socket descriptor.
    pub socket_type: i32,
    pub protocol: i32, // Usually PF_DLP.
    pub command: i32,
    pub local_addr: &'a SocketAddr,
    pub local_addr_len: usize,
    pub remote_addr: &'a SocketAddr,
    pub remote_addr_len: usize,
    pub protocol_queue: Vec<&'a Protocol>,
    pub protocol_queue_len: i32,
    pub command_queue: Vec<&'a Protocol>,
    pub command_queue_len: i32,
    pub device: &'a Device, // Low-level device we're talking to
    pub state: i32, // Initially SOCK_CLOSE. setsockopt() with SOCK_STATE to set the state.
    pub honor_rx_timeout: i32, // Honor packet reception timeouts. Set most to 1 of the time to have timeout management on incoming packets. Can be disabled when needed using setsockopt() with SOCK_HONOR_RX_TIMEOUT. This is used, for example, to disable timeouts in dlp_CallApplication() so that lengthy tasks don't return an error.
    pub is_in_command: bool,
    pub accept_timeout: i32, // Timeout value for call to accept().
    pub dlp_record: i32, // Index used for some DLP functions.
    pub dlp_version: i32, // Version of the DLP protocol running on the device.
    pub max_record_size: u64, // Max record size on the device.
    pub last_error: i32, // Error code returned by the last dlp_* command.
    pub palmos_error: i32, // Palm OS error code returned by the last transaction with the handheld.
}

pub struct SocketList<'a> {
    pub socket: &'a Socket<'a>,
    pub next: Option<&'a SocketList<'a>>,
}

pub fn find_socket_in_list(socket_list: Option<&'static SocketList<'static>>, sd: i32) -> Option<&Socket> {
    match socket_list {
        Some(root) => {
            let mut elem: Option<&SocketList> = root.next;
        
            loop {
                match elem {
                    Some(socket_list) => {
                        if socket_list.socket.sd == sd {
                            return Some(socket_list.socket)
                        }
                        elem = socket_list.next
                    },
                    None => {
                        break
                    }
                }
            }
            return None
        },
        None => {
            return None
        }
    }
}

// fn find_socket(sd: i32) -> Socket {
//     match FindSocket(sd)
// } 

// fn get_protocol(sd: i32, level: OptLevel) -> Protocol {
//     let socket: Socket;

//     Protocol { level: (), data: (), dup: (), read: (), write: (), flush: (), get_socket_opt: (), set_socket_opt: () }
// }
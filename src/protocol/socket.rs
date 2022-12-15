use std::net::SocketAddr;

use super::base::{Protocol, Device};

// static pilot_socket_list: Option<&'static SocketList<'static>> = None;

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
    sd: i32, // Socket descriptor.
    socket_type: i32,
    protocol: i32, // Usually PF_DLP.
    command: i32,
    local_addr: &'a SocketAddr,
    local_addr_len: usize,
    remote_addr: &'a SocketAddr,
    remote_addr_len: usize,
    protocol_queue: Vec<&'a Protocol>,
    protocol_queue_len: i32,
    command_queue: Vec<&'a Protocol>,
    command_queue_len: i32,
    device: &'a Device, // Low-level device we're talking to
    state: i32, // Initially SOCK_CLOSE. setsockopt() with SOCK_STATE to set the state.
    honor_rx_timeout: i32, // Honor packet reception timeouts. Set most to 1 of the time to have timeout management on incoming packets. Can be disabled when needed using setsockopt() with SOCK_HONOR_RX_TIMEOUT. This is used, for example, to disable timeouts in dlp_CallApplication() so that lengthy tasks don't return an error.
    is_in_command: bool,
    accept_timeout: i32, // Timeout value for call to accept().
    dlp_record: i32, // Index used for some DLP functions.
    dlp_version: i32, // Version of the DLP protocol running on the device.
    max_record_size: u64, // Max record size on the device.
    last_error: i32, // Error code returned by the last dlp_* command.
    palmos_error: i32, // Palm OS error code returned by the last transaction with the handheld.
}

pub struct SocketList<'a> {
    socket: &'a Socket<'a>,
    next: Option<&'a SocketList<'a>>,
}

fn FindSocketInList(socket_list: Option<&'static SocketList<'static>>, sd: i32) -> Option<&Socket> {
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

// fn FindSocket(sd: i32) -> Socket {
//     match FindSocket(sd)
// } 

// fn GetProtocol(sd: i32, level: OptLevel) -> Protocol {
//     let socket: Socket;

//     Protocol { level: (), data: (), dup: (), read: (), write: (), flush: (), get_socket_opt: (), set_socket_opt: () }
// }
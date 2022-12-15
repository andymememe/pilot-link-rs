//! Base for Protocols

use std::any::Any;

use super::socket::{Socket, OptLevel};

pub struct Protocol {
    pub level: OptLevel,
    pub data: Box<dyn Any>,
    pub dup: fn(&'static Protocol) -> Protocol,
    pub read: fn(&Socket, &Vec<u8>, usize, i32) -> isize,
    pub write: fn(&Socket, &Vec<u8>, usize, i32) -> isize,
    pub flush: fn(&Socket, i32) -> i32,
    pub get_socket_opt: fn(&Socket, i32, i32, Box<&dyn Any>, &usize) -> i32,
    pub set_socket_opt: fn(&Socket, i32, i32, Box<&dyn Any>, &usize) -> i32,
}

pub struct Device {

}
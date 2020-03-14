pub mod address;
pub mod appinfo;
pub mod buffer;

use std::str;

fn get_short(ptr: &[u8]) -> u16 {
    (ptr[0] as u16) << 8 | (ptr[1] as u16)
}

fn set_short(buf: &mut Vec<u8>, offset: usize, val: u16) {
    buf[offset] = ((val >> 8) & 0xff) as u8;
    buf[offset + 1] = (val & 0xff) as u8;
}

fn get_long(ptr: &[u8]) -> u64 {
    (ptr[0] as u64) << 24 | (ptr[1] as u64) << 16 | (ptr[2] as u64) << 8 | (ptr[3] as u64)
}

fn set_long(buf: &mut Vec<u8>, offset: usize, val: u64) {
    buf[offset] = ((val >> 24) & 0xff) as u8;
    buf[offset + 1] = ((val >> 16) & 0xff) as u8;
    buf[offset + 2] = ((val >> 8) & 0xff) as u8;
    buf[offset + 3] = (val & 0xff) as u8;
}

fn get_buf_string(buf: &Vec<u8>, offset: usize) -> (String, usize) {
    let mut i = offset;
    while buf[i] != 0 {
        i += 1;
    }
    i += 1;
    let ret = str::from_utf8(&buf[offset..i]).unwrap();

    (String::from(ret), i)
}

pub mod address;
pub mod appinfo;

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

fn reset_block(buffer: &mut Vec<u8>, len: usize, seed: u128) {
    buffer.resize(len, 0);
    for i in 0..len {
        buffer[i] = (i as u128 + seed) as u8 & 0xff;
    }
}

fn check_block(test: i32, buffer: &Vec<u8>, len: usize, count: usize, name: String, seed: u128) -> bool {
    let mut aft: usize = 0;

    for i in count .. len {
        if buffer[i] != (i as u128 + seed) as u8 & 0xff {
            aft = i;
            break;
        }
    }

    if aft != 0 {
        println!("{}: {} scribbled {} byte(s) after the allocated buffer.", test, name, aft);
        return true;
    }
    return false;
}

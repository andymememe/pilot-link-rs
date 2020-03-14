use super::buffer::pi_buffer_expect;
use std::str;

struct Address {
    phone_label: [u64; 5],
    show_phone: u64,
    entry: [String; 19],
}

enum AddressType {
    AddressV1,
}

enum AddressField {
    EntryLastname,
    EntryFirstname,
    EntryCompany,
    EntryPhone1,
    EntryPhone2,
    EntryPhone3,
    EntryPhone4,
    EntryPhone5,
    EntryAddress,
    EntryCity,
    EntryState,
    EntryZip,
    EntryCountry,
    EntryTitle,
    EntryCustom1,
    EntryCustom2,
    EntryCustom3,
    EntryCustom4,
    EntryNote,
    EntryCategory,
}

fn hi(x: u8) -> u8 {
    (x >> 4) & 0x0f
}

fn lo(x: u8) -> u8 {
    x & 0x0f
}

fn pair(x: u8, y: u8) -> u8 {
    (x << 4) | y
}

fn get_long(ptr: &[u8]) -> u64 {
    (ptr[0] as u64) << 24 | (ptr[1] as u64) << 16 | (ptr[2] as u64) << 8 | (ptr[3] as u64)
}

fn set_long(buf: &mut Vec<u8>, offset: usize, val: u64) {
    buf[offset] = ((val >> 24) & 0xff) as u8;
    buf[offset + 1] = ((val >> 16) & 0xff) as u8;
    buf[offset + 2] = ((val >> 8) & 0xff) as u8;
    buf[offset + 3] = ((val >> 0) & 0xff) as u8;
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

fn unpack_address(addr: &mut Address, buf: &Vec<u8>, addrType: AddressType) -> i32 {
    let contents: u64;
    let mut offset: usize;

    if buf.is_empty() && buf.len() < 9 {
        return -1;
    }

    addr.show_phone = hi(buf[1]) as u64;
    addr.phone_label[4] = lo(buf[1]) as u64;
    addr.phone_label[3] = hi(buf[2]) as u64;
    addr.phone_label[2] = lo(buf[2]) as u64;
    addr.phone_label[1] = hi(buf[3]) as u64;
    addr.phone_label[0] = lo(buf[4]) as u64;

    contents = get_long(&buf[4..8]);

    // Get byte offset
    offset = 9;

    for v in 0..19 {
        if (contents & (1 << v)) != 0 {
            if (buf.len() - offset) < 1 {
                return 0;
            }
            let (_entry, _offset) = get_buf_string(buf, offset);
            offset = _offset;
            addr.entry[v] = _entry;
        }
    }

    0
}

fn pack_address(addr: &Address, buf: &mut Vec<u8>, addrType: AddressType) -> i32 {
    let mut phoneFlag: u64;
    let mut contents: u64;
    let mut destlen: usize = 9;
    let mut buffer_offset: usize;
    let mut offset: usize;
    let mut l: usize;
    let mut buffer: String;

    for v in 0..19 {
        if addr.entry[v].len() > 0 {
            destlen += addr.entry[v].len() + 1
        }
    }

    pi_buffer_expect(buf, destlen);
    let (_buffer, _offset) = get_buf_string(buf, 9);
    buffer = _buffer;
    buffer_offset = _offset;
    phoneFlag = 0;
    contents = 0;
    offset = 0;

    for v in 0..19 {
        if addr.entry[v].len() > 0 {
            if v == (AddressField::EntryCompany as usize) {
                offset = buffer_offset - 8;
            }
            contents |= 1 << v;
            l = addr.entry[v].len() + 1;
            buffer = addr.entry[v].clone();
            buffer_offset += l;
        }
    }

    phoneFlag = addr.phone_label[0] << 0;
    phoneFlag |= addr.phone_label[1] << 4;
    phoneFlag |= addr.phone_label[2] << 8;
    phoneFlag |= addr.phone_label[3] << 12;
    phoneFlag |= addr.phone_label[4] << 16;
    phoneFlag |= addr.show_phone << 20;

    set_long(buf, 0, phoneFlag);
    set_long(buf, 4, contents);
    buf[8] = offset as u8;

    0
}

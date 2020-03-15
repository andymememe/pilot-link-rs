use super::appinfo::{pack_category_app_info, unpack_category_app_info, CategoryAppInfo};
use super::{get_buf_string, get_long, get_short, set_long, set_short};

use std::str;

pub struct Address {
    phone_label: [u64; 5],
    show_phone: u64,
    entry: [String; 19],
}

pub struct AddressAppInfo {
    address_type: AddressType,
    category: CategoryAppInfo,
    labels: [String; 16],
    label_renamed: [i64; 22],
    phone_labels: [String; 16],
    country: u16,
    sort_by_company: u8,
}

pub enum AddressType {
    AddressV1,
    Unknown,
}

pub enum AddressField {
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

// Hi bits
fn hi(x: u8) -> u8 {
    (x >> 4) & 0x0f
}

// Low bits
fn lo(x: u8) -> u8 {
    x & 0x0f
}

pub fn unpack_address(
    addr_option: Option<&mut Address>,
    buf_option: Option<&Vec<u8>>,
    addr_type: AddressType,
) -> i32 {
    let contents: u64;
    let buf: &Vec<u8>;
    let addr: &mut Address;
    let mut offset: usize;

    addr = addr_option.unwrap();

    match addr_type {
        AddressType::AddressV1 => {}
        _ => return -1,
    }

    match buf_option {
        Some(x) => buf = x,
        None => return -1,
    }

    if buf.len() < 9 {
        return -1;
    }

    // Unpack Show Phone
    addr.show_phone = hi(buf[1]) as u64;

    // Unpack Phone Label
    addr.phone_label[4] = lo(buf[1]) as u64;
    addr.phone_label[3] = hi(buf[2]) as u64;
    addr.phone_label[2] = lo(buf[2]) as u64;
    addr.phone_label[1] = hi(buf[3]) as u64;
    addr.phone_label[0] = lo(buf[4]) as u64;

    // Unpack Contents
    contents = get_long(&buf[4..8]);

    // Unpack Entry
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

pub fn pack_address(
    addr_option: Option<&Address>,
    buf_option: &mut Option<Vec<u8>>,
    addr_type: AddressType,
) -> i32 {
    let addr: &Address;
    let buf: &mut Vec<u8>;
    let mut phone_flag: u64;
    let mut contents: u64;
    let mut destlen: usize = 9;
    let mut buffer_offset: usize;
    let mut offset: usize;

    match addr_type {
        AddressType::AddressV1 => {}
        _ => return -1,
    }

    match addr_option {
        Some(x) => addr = x,
        None => return -1,
    }

    match buf_option {
        Some(x) => buf = x,
        None => return -1,
    }

    // Pack Contents
    for v in 0..19 {
        if addr.entry[v].len() > 0 {
            destlen += addr.entry[v].len() + 1
        }
    }
    buf.resize(buf.len() + destlen, 0);
    buffer_offset = 9;
    contents = 0;
    offset = 0;
    for v in 0..19 {
        if addr.entry[v].len() > 0 {
            if v == (AddressField::EntryCompany as usize) {
                offset = buffer_offset - 8;
            }
            contents |= 1 << v;
            for ele in addr.entry[v].as_bytes() {
                buf[buffer_offset] = *ele;
                buffer_offset += 1;
            }
        }
    }

    // Pack Phone Flag
    phone_flag = addr.phone_label[0] << 0;
    phone_flag |= addr.phone_label[1] << 4;
    phone_flag |= addr.phone_label[2] << 8;
    phone_flag |= addr.phone_label[3] << 12;
    phone_flag |= addr.phone_label[4] << 16;
    phone_flag |= addr.show_phone << 20;

    // Add to byffer
    set_long(buf, 0, phone_flag);
    set_long(buf, 4, contents);
    buf[8] = offset as u8;

    0
}

pub fn unpack_address_app_info(
    aai_option: Option<&mut AddressAppInfo>,
    record_option: Option<&Vec<u8>>,
    len: usize,
) -> usize {
    let record: &Vec<u8>;
    let aai: &mut AddressAppInfo;
    let r: u64;
    let i: usize;
    let destlen: usize = 4 + 16 * 22 + 2 + 2;
    let mut record_offset: usize = 0;
    let mut len_offset = len;

    aai = aai_option.unwrap();

    // Unpack Address Type
    aai.address_type = AddressType::AddressV1;

    // Unpack Category App Info
    i = unpack_category_app_info(&mut aai.category, record_option, len);
    match record_option {
        Some(x) => record = x,
        None => return i + destlen,
    }

    if i == 0 {
        return i;
    }
    record_offset += i;
    len_offset -= i;
    if len_offset < destlen {
        return 0;
    }

    // Unpack Label Renamed
    r = get_long(&record[record_offset..record_offset + 4]);
    for i in 0..22 {
        aai.label_renamed[i] = !!(r & (1 << i)) as i64;
    }
    record_offset += 4;

    // Unpack Label
    for i in 0..22 {
        aai.labels[i] =
            String::from(str::from_utf8(&record[record_offset..record_offset + 16]).unwrap());
        record_offset += 16;
    }

    // Unpack Country
    aai.country = get_short(&record[record_offset..record_offset + 2]);
    record_offset += 2;

    // Unpack Sort by Company
    aai.sort_by_company = record[record_offset];
    record_offset += 2;

    // Unpack Phone Labels
    for i in 3..8 {
        aai.phone_labels[i - 3] = aai.labels[i].clone();
    }
    for i in 19..22 {
        aai.phone_labels[i - 19 + 5] = aai.labels[i].clone();
    }

    // Return Record Length
    record_offset
}

pub fn pack_address_app_info(
    aai: &AddressAppInfo,
    record: Option<&mut Vec<u8>>,
    len: usize,
) -> usize {
    let i: usize;
    let destlen: usize = 4 + 16 * 22 + 2 + 2;
    let mut r: u64;
    let mut record_offset: usize = 0;

    // Pack Category App Info
    i = pack_category_app_info(&aai.category, record, len);
    if i == 0 {
        return i;
    }
    record_offset += i;

    // Padding Zero
    for i in record_offset..record_offset + destlen {
        record[i] = 0;
    }

    // Pack Label Renamed
    r = 0;
    for i in 0..22 {
        if aai.label_renamed[i] == 0 {
            r |= 1 << i;
        }
    }
    set_long(record, record_offset, r);
    record_offset += 4;

    for i in 0..22 {
        let label = aai.labels[i].as_bytes();
        let label_len = label.len();

        for j in 0..16 {
            if j < label_len {
                record[record_offset] = label[j];
            } else {
                record[record_offset] = 0;
            }
            record_offset += 1;
        }
    }

    // Pack Country
    set_short(record, record_offset, aai.country);
    record_offset += 2;

    // Pack Sort By Company
    record[record_offset] = aai.sort_by_company;
    record_offset += 2;

    // Return Record Length
    record_offset
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add() {}
}

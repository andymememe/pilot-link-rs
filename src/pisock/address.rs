use super::appinfo::{pack_category_app_info, unpack_category_app_info, CategoryAppInfo};
use super::{get_buf_string, get_long, get_short, set_long, set_short};
use std::str;

#[derive(Debug, PartialEq)]
pub enum AddressType {
    AddressV1,
    Unknown,
}

impl Default for AddressType {
    fn default() -> AddressType {
        AddressType::AddressV1
    }
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

#[derive(Default, Debug, PartialEq)]
pub struct Address {
    pub phone_label: [u8; 5],
    pub show_phone: u8,
    pub entry: [String; 19],
}

#[derive(Default, Debug, PartialEq)]
pub struct AddressAppInfo {
    pub address_type: AddressType,
    pub category: CategoryAppInfo,
    pub labels: [String; 22],
    pub label_renamed: [i64; 22],
    pub phone_labels: [String; 8],
    pub country: u16,
    pub sort_by_company: u8,
}

// Hi bits
fn hi(x: u8) -> u8 {
    (x >> 4) & 0x0f
}

// Low bits
fn lo(x: u8) -> u8 {
    x & 0x0f
}

pub fn unpack_address(addr: &mut Address, buf: &Vec<u8>, addr_type: AddressType) -> i32 {
    let contents: u64;
    let mut offset: usize;

    match addr_type {
        AddressType::AddressV1 => {}
        _ => return -1,
    }

    if buf.len() < 9 {
        return -1;
    }

    // Unpack Show Phone
    addr.show_phone = hi(buf[1]);

    // Unpack Phone Label
    addr.phone_label[4] = lo(buf[1]);
    addr.phone_label[3] = hi(buf[2]);
    addr.phone_label[2] = lo(buf[2]);
    addr.phone_label[1] = hi(buf[3]);
    addr.phone_label[0] = lo(buf[4]);

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
            offset = _offset + 1;
            addr.entry[v] = _entry;
        }
    }

    0
}

pub fn pack_address(addr: &Address, buf: &mut Vec<u8>, addr_type: AddressType) -> i32 {
    let mut phone_flag: u64;
    let mut contents: u64;
    let mut destlen: usize = 9;
    let mut buffer_offset: usize;
    let mut offset: usize;

    match addr_type {
        AddressType::AddressV1 => {}
        _ => return -1,
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
            buf[buffer_offset] = 0;
            buffer_offset += 1;
        }
    }

    // Pack Phone Flag
    phone_flag = (addr.phone_label[0] as u64) << 0;
    phone_flag |= (addr.phone_label[1] as u64) << 4;
    phone_flag |= (addr.phone_label[2] as u64) << 8;
    phone_flag |= (addr.phone_label[3] as u64) << 12;
    phone_flag |= (addr.phone_label[4] as u64) << 16;
    phone_flag |= (addr.show_phone as u64) << 20;

    // Add to byffer
    set_long(buf, 0, phone_flag);
    set_long(buf, 4, contents);
    buf[8] = offset as u8;

    0
}

pub fn unpack_address_app_info(aai: &mut AddressAppInfo, record: &Vec<u8>, len: usize) -> usize {
    let r: u64;
    let i: usize;
    let destlen: usize = 4 + 16 * 22 + 2 + 2;
    let mut record_offset: usize = 0;
    let mut len_offset = len;

    // Unpack Address Type
    aai.address_type = AddressType::AddressV1;

    // Unpack Category App Info
    i = unpack_category_app_info(&mut aai.category, record, len);

    if record.capacity() == 0 {
        return i + destlen;
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
        let mut j: usize = 16;
        for _j in 0..16 {
            if record[record_offset + _j] == 0 {
                j = _j;
                break;
            }
        }
        aai.labels[i] =
            String::from(str::from_utf8(&record[record_offset..record_offset + j]).unwrap());
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

pub fn pack_address_app_info(aai: &AddressAppInfo, record: &mut Vec<u8>, len: usize) -> usize {
    let i: usize;
    let destlen: usize = 4 + 16 * 22 + 2 + 2;
    let mut r: u64;
    let mut record_offset: usize = 0;

    // Pack Category App Info
    i = pack_category_app_info(&aai.category, record, len);
    if record.capacity() == 0 {
        return destlen + i;
    }

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
        if aai.label_renamed[i] != 0 {
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
    use crate::pisock::{check_block, reset_block};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn get_address_app_block() -> Vec<u8> {
        String::from(
            "\
            \x00\x10\x55\x6e\x66\x69\x6c\x65\x64\x00\x00\x00\x00\x00\x00\x00\
            \x00\x00\x42\x75\x73\x69\x6e\x65\x73\x73\x00\x00\x00\x00\x00\x00\
            \x00\x00\x50\x65\x72\x73\x6f\x6e\x61\x6c\x00\x00\x00\x00\x00\x00\
            \x00\x00\x51\x75\x69\x63\x6b\x4c\x69\x73\x74\x00\x00\x00\x00\x00\
            \x00\x00\x46\x6f\x6f\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\
            \x00\x00\x00\x01\x02\x03\x11\x00\x00\x00\x00\x00\x00\x00\x00\x00\
            \x00\x00\x11\x00\x00\x00\x00\x00\x0e\x00\x4c\x61\x73\x74\x20\x6e\
            \x61\x6d\x65\x00\x00\x00\x00\x00\x00\x00\x46\x69\x72\x73\x74\x20\
            \x6e\x61\x6d\x65\x00\x00\x00\x00\x00\x00\x43\x6f\x6d\x70\x61\x6e\
            \x79\x00\x00\x00\x00\x00\x00\x00\x00\x00\x57\x6f\x72\x6b\x00\x00\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x48\x6f\x6d\x65\x00\x00\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x46\x61\x78\x00\x00\x00\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x4f\x74\x68\x65\x72\x00\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x45\x2d\x6d\x61\x69\x6c\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x41\x64\x64\x72\x65\x73\
            \x73\x00\x00\x00\x00\x00\x00\x00\x00\x00\x43\x69\x74\x79\x00\x00\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x53\x74\x61\x74\x65\x00\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x5a\x69\x70\x20\x43\x6f\
            \x64\x65\x00\x00\x00\x00\x00\x00\x00\x00\x43\x6f\x75\x6e\x74\x72\
            \x79\x00\x00\x00\x00\x00\x00\x00\x00\x00\x54\x69\x74\x6c\x65\x00\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x43\x75\x73\x74\x6f\x6d\
            \x20\x31\x00\x00\x00\x00\x00\x00\x00\x00\x43\x75\x73\x74\x6f\x6d\
            \x20\x32\x00\x00\x00\x00\x00\x00\x00\x00\x43\x75\x73\x74\x6f\x6d\
            \x20\x33\x00\x00\x00\x00\x00\x00\x00\x00\x43\x75\x73\x74\x6f\x6d\
            \x20\x34\x00\x00\x00\x00\x00\x00\x00\x00\x4e\x6f\x74\x65\x00\x00\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x4d\x61\x69\x6e\x00\x00\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x50\x61\x67\x65\x72\x00\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x4d\x6f\x62\x69\x6c\x65\
            \x00\x00\x00\x00\x00\x00\x00\x00\x00\x00\x17\x00\x00\x00",
        )
        .as_bytes()
        .to_vec()
    }

    fn get_address_app_info() -> AddressAppInfo {
        let mut address_app_info = AddressAppInfo::default();
        address_app_info.category.renamed = [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        address_app_info.category.name = [
            String::from("Unfiled"),
            String::from("Business"),
            String::from("Personal"),
            String::from("QuickList"),
            String::from("Foo"),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
        ];
        address_app_info.category.id = [0, 1, 2, 3, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        address_app_info.category.last_unique_id = 17;
        address_app_info.labels = [
            String::from("Last name"),
            String::from("First name"),
            String::from("Company"),
            String::from("Work"),
            String::from("Home"),
            String::from("Fax"),
            String::from("Other"),
            String::from("E-mail"),
            String::from("Address"),
            String::from("City"),
            String::from("State"),
            String::from("Zip Code"),
            String::from("Country"),
            String::from("Title"),
            String::from("Custom 1"),
            String::from("Custom 2"),
            String::from("Custom 3"),
            String::from("Custom 4"),
            String::from("Note"),
            String::from("Main"),
            String::from("Pager"),
            String::from("Mobile"),
        ];
        address_app_info.label_renamed = [
            0, 0, 0, 0, 0, 0, 0, 0, 0, 512, 1024, 2048, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ];
        address_app_info.phone_labels = [
            String::from("Work"),
            String::from("Home"),
            String::from("Fax"),
            String::from("Other"),
            String::from("E-mail"),
            String::from("Main"),
            String::from("Pager"),
            String::from("Mobile"),
        ];
        address_app_info.country = 5888;
        address_app_info.sort_by_company = 0;

        address_app_info
    }

    fn get_address_record() -> Vec<u8> {
        String::from(
            "\
            \x00\x14\x32\x10\x00\x04\x41\x03\x00\x53\x68\x61\x77\x00\x42\x65\
            \x72\x6e\x61\x72\x64\x00\x4e\x6f\x6e\x65\x20\x6b\x6e\x6f\x77\x6e\
            \x00\x43\x31\x00\x41\x20\x6e\x6f\x74\x65\x2e\x00",
        )
        .as_bytes()
        .to_vec()
    }

    fn get_address() -> Address {
        let mut address = Address::default();
        address.phone_label = [0, 1, 2, 3, 4];
        address.show_phone = 1;
        address.entry = [
            String::from("Shaw"),
            String::from("Bernard"),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from("None known"),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from("C1"),
            String::from(""),
            String::from(""),
            String::from(""),
            String::from("A note."),
        ];

        address
    }

    #[test]
    fn test_unpack_address_app_info() {
        let address_app_block: &Vec<u8> = &get_address_app_block();
        let mai: &mut AddressAppInfo = &mut AddressAppInfo::default();
        let l = unpack_address_app_info(mai, address_app_block, address_app_block.len() + 10);
        assert_eq!(l, address_app_block.len());

        let l = unpack_address_app_info(mai, address_app_block, address_app_block.len() + 1);
        assert_eq!(l, address_app_block.len());

        let l = unpack_address_app_info(mai, address_app_block, address_app_block.len() - 10);
        assert_eq!(l, 0);

        let l = unpack_address_app_info(mai, address_app_block, address_app_block.len());
        assert_eq!(l, address_app_block.len());
        assert_eq!(*mai, get_address_app_info());
    }

    #[test]
    fn test_pack_address_app_info() {
        let buf = &mut vec![];
        let target = &mut Vec::<u8>::new();
        let address_app_block: &Vec<u8> = &get_address_app_block();
        let mai = &get_address_app_info();
        let now = SystemTime::now();
        let seed = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backward")
            .as_millis() as u128;

        let l = pack_address_app_info(mai, buf, 0);
        assert_eq!(l, address_app_block.len());

        target.resize(8192, 0);
        reset_block(target, 8192, seed);
        let l = pack_address_app_info(mai, target, 1);
        assert_eq!(l, 0);
        assert!(!check_block(
            9,
            target,
            8192,
            1,
            String::from("pack_address_app_info"),
            seed
        ));

        reset_block(target, 8192, seed);
        let l = pack_address_app_info(mai, target, 8192 - 256);
        assert_eq!(l, address_app_block.len());
        assert!(!check_block(
            9,
            target,
            8192,
            l,
            String::from("pack_address_app_info"),
            seed
        ));

        for i in 0..address_app_block.len() {
            assert_eq!(target[i], address_app_block[i]);
        }
    }
    #[test]
    fn test_unpack_address() {
        let m = &mut Address::default();
        let address_record = get_address_record();

        unpack_address(m, &address_record, AddressType::AddressV1);
        assert_eq!(*m, get_address());
    }

    #[test]
    fn test_pack_address() {
        let record_buffer = &mut Vec::with_capacity(0);
        let address_record = get_address_record();
        let address = get_address();

        assert_eq!(
            pack_address(&address, record_buffer, AddressType::AddressV1),
            0
        );
        assert_eq!(record_buffer.len(), address_record.len());
        for i in 0..address_record.len() {
            assert_eq!(record_buffer[i], address_record[i]);
        }
    }
}

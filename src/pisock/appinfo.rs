use super::{get_short, set_short};
use std::str;

#[derive(Default, Debug, PartialEq)]
pub struct CategoryAppInfo {
    pub renamed: [u64; 16],
    pub name: [String; 16],
    pub id: [u8; 16],
    pub last_unique_id: u8,
}

pub fn unpack_category_app_info(ai: &mut CategoryAppInfo, record: &Vec<u8>, len: usize) -> usize {
    let rec: u16;
    let mut record_offset: usize = 0;

    // Not enough space
    if len < 2 + 16 * 16 + 16 + 4 {
        return 0;
    }

    rec = get_short(record);

    // Renamed
    for i in 0..16 {
        if (rec & (1 << i)) != 0 {
            ai.renamed[i] = 1;
        } else {
            ai.renamed[i] = 0;
        }
    }
    record_offset += 2;

    // Name
    for i in 0..16 {
        let mut j: usize = 16;
        for _j in 0..16 {
            if record[record_offset + _j] == 0 {
                j = _j;
                break;
            }
        }
        ai.name[i] =
            String::from(str::from_utf8(&record[record_offset..record_offset + j]).unwrap());
        record_offset += 16;
    }

    // ID
    for i in 0..16 {
        ai.id[i] = record[record_offset + i];
    }
    record_offset += 16;

    // Last Unique ID
    ai.last_unique_id = record[record_offset];

    // Return Record Length
    2 + 16 * 16 + 16 + 4
}

pub fn pack_category_app_info(ai: &CategoryAppInfo, record: &mut Vec<u8>, len: usize) -> usize {
    let mut rec: u16;
    let mut record_offset: usize = 0;

    if record.capacity() == 0 {
        return 2 + 16 * 16 + 16 + 4;
    }

    if len < 2 + 16 * 16 + 16 + 4 {
        return 0;
    }

    // Pack Renamed
    rec = 0;
    for i in 0..16 {
        if ai.renamed[i] != 0 {
            rec |= 1 << i;
        }
    }
    set_short(record, record_offset, rec);
    record_offset += 2;

    // Pack Name
    for i in 0..16 {
        let name = ai.name[i].as_bytes();
        let name_len = name.len();

        for j in 0..16 {
            if j < name_len {
                record[record_offset] = name[j];
            } else {
                record[record_offset] = 0;
            }
            record_offset += 1;
        }
    }

    // Pack ID
    for i in 0..16 {
        record[record_offset] = ai.id[i];
        record_offset += 1;
    }

    // Pack Last Unique ID
    record[record_offset] = ai.last_unique_id;
    record_offset += 1;

    record[record_offset] = 0;
    set_short(record, record_offset + 1, 0);
    record_offset += 3;

    // Return Record Length
    record_offset
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::pisock::{check_block, reset_block};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn get_category_app_block() -> Vec<u8> {
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
            \x00\x00\x11\x00\x00\x00",
        )
        .as_bytes()
        .to_vec()
    }

    fn get_category_app_info() -> CategoryAppInfo {
        let mut category_app_info = CategoryAppInfo::default();
        category_app_info.renamed = [0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        category_app_info.name = [
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
        category_app_info.id = [0, 1, 2, 3, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
        category_app_info.last_unique_id = 17;

        category_app_info
    }

    #[test]
    fn test_unpack_category_app_info() {
        let category_app_block: &Vec<u8> = &get_category_app_block();
        let mci: &mut CategoryAppInfo = &mut CategoryAppInfo::default();
        let l = unpack_category_app_info(mci, category_app_block, category_app_block.len() + 10);
        assert_eq!(l, category_app_block.len());

        let l = unpack_category_app_info(mci, category_app_block, category_app_block.len() + 1);
        assert_eq!(l, category_app_block.len());

        let l = unpack_category_app_info(mci, category_app_block, category_app_block.len() - 10);
        assert_eq!(l, 0);

        let l = unpack_category_app_info(mci, category_app_block, category_app_block.len());
        assert_eq!(l, category_app_block.len());
        assert_eq!(*mci, get_category_app_info());
    }

    #[test]
    fn test_pack_category_app_info() {
        let buf = &mut vec![];
        let target = &mut Vec::<u8>::new();
        let category_app_block: &Vec<u8> = &get_category_app_block();
        let mci = &get_category_app_info();
        let now = SystemTime::now();
        let seed = now
            .duration_since(UNIX_EPOCH)
            .expect("Time went backward")
            .as_millis() as u128;

        let l = pack_category_app_info(mci, buf, 0);
        assert_eq!(l, category_app_block.len());

        target.resize(8192, 0);
        reset_block(target, 8192, seed);
        let l = pack_category_app_info(mci, target, 1);
        assert_eq!(l, 0);
        assert!(!check_block(
            9,
            target,
            8192,
            1,
            String::from("pack_category_app_info"),
            seed
        ));

        reset_block(target, 8192, seed);
        let l = pack_category_app_info(mci, target, 8192 - 256);
        assert_eq!(l, category_app_block.len());
        assert!(!check_block(
            9,
            target,
            8192,
            l,
            String::from("pack_category_app_info"),
            seed
        ));

        for i in 0..category_app_block.len() {
            assert_eq!(target[i], category_app_block[i]);
        }
    }
}

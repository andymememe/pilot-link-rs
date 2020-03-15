use super::{get_short, set_short};
use std::str;

pub struct CategoryAppInfo {
    renamed: [u64; 16],
    name: [String; 16],
    id: [u8; 16],
    last_unique_id: u8,
}

pub fn unpack_category_app_info(
    ai: &mut CategoryAppInfo,
    record: &Vec<u8>,
    len: usize,
) -> usize {
    let rec: u16;
    let mut record_offset: usize = 0;

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
        ai.name[i] =
            String::from(str::from_utf8(&record[record_offset..record_offset + 16]).unwrap());
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

pub fn pack_category_app_info(
    ai: &CategoryAppInfo,
    record: &mut Vec<u8>,
    len: usize,
) -> usize {
    let mut rec: u16;
    let mut record_offset: usize = 0;

    if len < 2 + 16 * 16 + 16 + 4 {
        return 0;
    }

    // Pack Renamed
    rec = 0;
    for i in 0..16 {
        if ai.renamed[i] == 0 {
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

    #[test]
    fn test_add() {}
}

use super::{get_short, set_short};
use std::str;

pub const BLOB_TYPE_CALENDAR_TIMEZONE_ID: &str = "Bd00";
pub const BLOB_TYPE_CALENDAR_UNKNOWN_ID: &str = "Bd01";
pub const MAX_BLOBS: usize = 10;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Blob {
    pub blob_type: String,
    pub length: usize,
    pub data: Vec<u8>,
}

pub fn unpack_blob(blob: &mut Blob, data: &Vec<u8>, position: usize) -> usize {
    let mut local_pos = position;

    // Unpack Type
    let mut i: usize = 4;
    for _i in local_pos..local_pos + 4 {
        if data[_i] == 0 {
            i = _i;
            break;
        }
    }
    blob.blob_type = String::from(str::from_utf8(&data[local_pos..local_pos + i]).unwrap());
    local_pos += 4;

    // Unpack Length
    blob.length = get_short(&data[local_pos..local_pos + 2]) as usize;
    local_pos += 2;

    // Unpack Data
    blob.data.resize(blob.length, 0);
    for i in 0..blob.length {
        blob.data[i] = data[local_pos + i];
    }
    local_pos += blob.length;

    local_pos - position
}

pub fn pack_blob(blob: &Blob, buffer: &mut Vec<u8>) -> i32 {
    let mut offset: usize;

    offset = buffer.len();
    buffer.resize(buffer.len() + 6 + blob.length, 0);

    // Pack Type
    let blob_type_byte = blob.blob_type.as_bytes();
    for i in 0..4 {
        if i < blob_type_byte.len() {
            buffer[offset + i] = blob_type_byte[i];
        } else {
            buffer[offset + i] = 0;
        }
    }
    offset += 4;

    // Pack Length
    set_short(buffer, offset, blob.length as u16);
    offset += 2;

    for i in 0..blob.length {
        buffer[offset + i] = blob.data[i];
    }

    0
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_blob_block() -> Vec<u8> {
        String::from(
            "\
            \x00\x00\x42\x64\x30\x30\x00\x02\x01\x02",
        )
        .as_bytes()
        .to_vec()
    }

    fn get_blob() -> Blob {
        let mut blob = Blob::default();
        blob.blob_type = String::from(BLOB_TYPE_CALENDAR_TIMEZONE_ID);
        blob.length = 2;
        blob.data = vec![1, 2];

        blob
    }

    #[test]
    fn test_unpack_blob() {
        let b = &mut Blob::default();
        let blob = &get_blob();
        let blob_block = &get_blob_block();
        unpack_blob(b, blob_block, 2);
        assert_eq!(*b, *blob);
    }

    #[test]
    fn test_pack_blob() {
        let block = &mut Vec::<u8>::with_capacity(0);
        let blob = &get_blob();
        let blob_block = &get_blob_block()[2..];

        pack_blob(blob, block);

        for i in 0..block.len() {
            assert_eq!(block[i], blob_block[i]);
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct GenericPacket {
    pub data: Vec<u8>,
    pub source: i8,
    pub destination: i8,
    pub transaction_id: i8
}

pub fn new(data: Vec<u8>, src_socket: i8, dest_sock: i8) -> GenericPacket {
    GenericPacket{
        data: data,
        source: src_socket,
        destination: dest_sock,
        transaction_id: 0,
    }
}

pub fn new_with_transid(data: Vec<u8>, src_socket: i8, dest_sock: i8, trans_id: i8) -> GenericPacket {
    GenericPacket{
        data: data,
        source: src_socket,
        destination: dest_sock,
        transaction_id: trans_id,
    }
}
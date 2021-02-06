#[derive(Serialize, Deserialize)]
pub struct GenericPacket {
    pub data: Vec<i8>,
    pub source: i8,
    pub destination: i8,
    pub transaction_id: i8
}

pub fn new(data: Vec<i8>, srcSocket: i8, destSock: i8) -> GenericPacket {
    GenericPacket{
        data: data,
        source: srcSocket,
        destination: destSock,
        transaction_id: 0,
    }
}

pub fn new_with_transid(data: Vec<i8>, srcSocket: i8, destSock: i8, transID: i8) -> GenericPacket {
    GenericPacket{
        data: data,
        source: srcSocket,
        destination: destSock,
        transaction_id: transID,
    }
}
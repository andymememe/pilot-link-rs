//! Generic Packet

/// The GenericPacket class
/// 
/// This class represents a generic packet structure.
/// 
/// It is used for passing packets between protocol layers 
/// for encoding and decoding.
#[derive(Serialize, Deserialize)]
pub struct GenericPacket {
    pub data: Vec<u8>,
    pub source: i8,
    pub destination: i8,
    pub transaction_id: i8
}

/// Constructs a new `GenericPacket`
/// 
/// # Parameters
/// 
/// * `data`: A byte array containing the packet data.
/// * `sourceSocket`: The source socket for this packet.
/// * `destSocket`: The destination socket for this packet.
pub fn new(data: Vec<u8>, src_socket: i8, dest_sock: i8) -> GenericPacket {
    GenericPacket{
        data: data,
        source: src_socket,
        destination: dest_sock,
        transaction_id: 0,
    }
}

/// Constructs a new `GenericPacket` with Transaction ID
/// 
/// # Parameters
/// 
/// * `data`: A byte array containing the packet data.
/// * `sourceSocket`: The source socket for this packet.
/// * `destSocket`: The destination socket for this packet.
/// * `transID`: The transaction ID for this packet.
pub fn new_with_transid(data: Vec<u8>, src_socket: i8, dest_sock: i8, trans_id: i8) -> GenericPacket {
    GenericPacket{
        data: data,
        source: src_socket,
        destination: dest_sock,
        transaction_id: trans_id,
    }
}
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
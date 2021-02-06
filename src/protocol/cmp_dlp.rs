use std::any::type_name;
use super::super::type_of;
use super::generic_packet::GenericPacket;
use super::padp::PADP;

pub trait CMPDLPTransfer {
    fn connect(&self);
    fn disconnect(&self);
    fn suspend_connection(&self);
    fn is_connected(&self) -> bool;
    fn use_long_packets(&self, flag: bool);
    fn read_packet(&self) -> GenericPacket;
    fn transmit_packet(&self, data: Vec<i8>, srcSocket: i8, destSock: i8);
}

pub struct CMPDLP<'a> {
    padp_handler: &'a dyn CMPDLPTransfer,
    connected: bool,
    speed: i32
}

pub fn new<'a>(padp: &'a dyn CMPDLPTransfer) -> CMPDLP {
    CMPDLP {
        padp_handler: padp,
        connected: false,
        speed: 9600,
    }
}

impl<'a> CMPDLP<'a> {
    pub fn connect(&self) {
        let flags: i8 = 0;
    
        if self.padp_handler.is_connected() {
            &self.connect();
        }

        if type_of(&self.padp_handler) == type_name::<PADP>() {
            let mut pkt: GenericPacket;
            // TODO: let cmp_pkt: CMPPacket
            loop {
                pkt = self.padp_handler.read_packet();

                if determine_packet_type(pkt) {
                    // TODO: Error msg
                    continue
                }
            }

            // CMPPacketInterface cmpPktI = CMPPacket::bytes2Packet(pkt.data);
        }
    }
}

fn determine_packet_type(generic_packet: GenericPacket) -> bool {
    generic_packet.data[0] >= 16
}
//! CMP DLP Protocol
//!
//! # CMP
//! `CMP` is `Connection Management Protocol`.
//!
//! CMP provides connection-establishment capabilities featuring baud rate arbitration and
//! exchange of communications software version numbers.
//!
//! # DLP
//! `DLP` is `Desktop Link Protocol`.
//!
//! DLP provides remote access to Palm OS data storage and other sub-systems.
//!
//! DLP facilitates efficient data synchronization between desktop and Palm OS applications,
//! database backup, installation of code patches, extensions, applications, and other databases,
//! as well as Remote Inter-Application Communication (RIAC) and Remote Procedure Calls (RPC).

use std::any::type_name;
use std::convert::TryInto;

use super::super::type_of;
use super::generic_packet::GenericPacket;
use super::padp::PADP;

// CMP Packet Type
/// CMP Packet Type: Wake Up
pub const CMP_WAKEUP: u8 = 1;
/// CMP Packet Type: Initialize
pub const CMP_INIT: u8 = 2;
/// CMP Packet Type: Abort
pub const CMP_ABORT: u8 = 3;
/// CMP Packet Type: Extended
pub const CMP_EXTENDED: u8 = 4;

// Flag for CMP Packet
/// CMP Packet Flag: Change Baud Rate
pub const FLAG_CHANGE_BAUD_RATE: u8 = 0x80;
/// CMP Packet Flag: 1 Min. Timeout
pub const FLAG_ONE_MINUTE_TIMEOUT: u8 = 0x40;
/// CMP Packet Flag: 2 Min. Timeout
pub const FLAG_TWO_MINUTE_TIMEOUT: u8 = 0x20;
/// CMP Packet Flag: Long Packet Support
pub const FLAG_LONG_PACKET_SUPPORT: u8 = 0x10;

// Other Constants
/// Wake Up Transaction ID
pub const WAKEUP_TID: i8 = -1;
/// Version Mismatch
pub const VERSION_MISMATCH: u8 = 0x80;
/// Default Speed for Serial Connection
pub const DEFAULT_SPEED: u32 = 9600;

/// CMP / DLP Packet I/O Trait
///
/// Defines functions of connection and transfering with CMP / DLP
pub trait CMPDLPTransferTrait {
    fn connect(&self);
    fn disconnect(&self);
    fn suspend_connection(&self);
    fn is_connected(&self) -> bool;
    fn use_long_packets(&self, flag: bool);
    fn read_packet(&self) -> GenericPacket;
    fn transmit_packet(&self, data: Vec<u8>, src_socket: i8, dest_sock: i8);
}

/// CMP Packet Trait
///
/// Defines CMPPacket converter
pub trait CMPPacketTrait {
    fn packet_to_bytes(&self) -> Vec<u8>;
    fn bytes_to_packet(pkt: &Vec<u8>) -> Self;
}


/// CMP / DLP protocol
/// 
/// A `CMPDLP` contains PADP handler, connected state and 
/// speed of serial connection (default speed is 9600, 
/// ignored for non-serial connection).
pub struct CMPDLP<'a> {
    padp_handler: &'a dyn CMPDLPTransferTrait,
    connected: bool,
    speed: u32,
}

/// CMP Packet
/// 
/// Define CMP Packet, includes packet type, flags, version and baud rate
#[derive(Serialize, Deserialize)]
pub struct CMPPacket {
    pub packet_type: u8,
    pub flags: u8,
    pub major_version: u8,
    pub minor_version: u8,
    pub baud_rate: u32,
}

impl<'a> CMPDLP<'a> {
    /// Start a connection
    pub fn connect(&mut self) {
        if self.padp_handler.is_connected() {
            &self.connect();
        }

        if type_of(&self.padp_handler) == type_name::<PADP>() {
            self.padp_connect()
        } else if type_of(&self.padp_handler) == "USB" { // TODO: Use type_name
        }

        self.connected = true;
    }

    /// Disconnect without reason
    pub fn disconnect(&self) {
        self.disconnect_with_reason('\0');
    }

    /// Disconnect with reason
    pub fn disconnect_with_reason(&self, reason: char) {
        // TODO: Implement
    }

    /// PADP connection
    fn padp_connect(&self) {
        let mut flags: u8 = 0;
        let mut pkt: GenericPacket;
        let cmp_pkt: CMPPacket;

        loop {
            pkt = self.padp_handler.read_packet();

            if determine_packet_type(&pkt) {
                // TODO: Error msg
                continue;
            }

            cmp_pkt = CMPPacket::bytes_to_packet(&pkt.data);
            break;
        }

        if self.speed != 9600 {
            flags = FLAG_CHANGE_BAUD_RATE;
        }

        if cmp_pkt.test_flag(FLAG_LONG_PACKET_SUPPORT) {
            flags |= FLAG_LONG_PACKET_SUPPORT;
            self.padp_handler.use_long_packets(true);
        } else {
            self.padp_handler.use_long_packets(false);
        }

        self.padp_handler.transmit_packet(
            new_cmp_packet_with_setting(CMP_INIT, flags, 0, 0, self.speed).packet_to_bytes(),
            3,
            3,
        )
    }

    /// USB Receive Handshake
    fn usb_rx_handshake(&self) {
        let mut pkt: GenericPacket;
        // TODO: USB RX Handshake
    }
}

impl CMPPacket {
    pub fn test_flag(&self, flag: u8) -> bool {
        (self.flags & flag) == flag
    }
}

impl CMPPacketTrait for CMPPacket {
    fn packet_to_bytes(&self) -> Vec<u8> {
        let mut data: Vec<u8> = vec![0; 10];
        data[0] = self.packet_type;
        data[1] = self.flags;
        data[2] = self.major_version;
        data[3] = self.minor_version;
        data[4] = 0;
        data[5] = 0;

        let baud_rate_arr = self.baud_rate.to_be_bytes();
        for (i, a_byte) in baud_rate_arr.iter().enumerate() {
            data[i + 6] = *a_byte
        }

        data
    }

    fn bytes_to_packet(pkt: &Vec<u8>) -> CMPPacket {
        let mut cmp_pkt = new_cmp_packet();
        cmp_pkt.packet_type = pkt[0];
        cmp_pkt.flags = pkt[1];
        cmp_pkt.major_version = pkt[2];
        cmp_pkt.minor_version = pkt[3];
        cmp_pkt.baud_rate =
            u32::from_be_bytes(pkt[6..=9].try_into().expect("Slice with incorrect length"));
        cmp_pkt
    }
}

pub fn new_cmp_dlp<'a>(padp: &'a dyn CMPDLPTransferTrait) -> CMPDLP {
    CMPDLP {
        padp_handler: padp,
        connected: false,
        speed: 9600,
    }
}

pub fn new_cmp_packet() -> CMPPacket {
    CMPPacket {
        packet_type: 0,
        flags: 0,
        major_version: 0,
        minor_version: 0,
        baud_rate: DEFAULT_SPEED,
    }
}

pub fn new_cmp_packet_with_setting(
    packet_type: u8,
    flags: u8,
    major_version: u8,
    minor_version: u8,
    baud: u32,
) -> CMPPacket {
    CMPPacket {
        packet_type: packet_type,
        flags: flags,
        major_version: major_version,
        minor_version: minor_version,
        baud_rate: baud,
    }
}

fn determine_packet_type(generic_packet: &GenericPacket) -> bool {
    generic_packet.data[0] >= 16
}

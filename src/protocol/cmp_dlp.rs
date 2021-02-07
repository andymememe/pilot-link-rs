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

/// CMP / DLP Packet I/O Trait
///
/// Defines functions of connection and transfering with CMP / DLP
pub trait CMPDLPTransferTrait {
    /// Method to be called to initialize the connection.
    fn connect(&self);

    /// Disconnects the connection to the underlying communication subsystem.
    fn disconnect(&self);

    /// Suspends access to the underlying communication subsystem.
    fn suspend_connection(&self);

    /// Tests the state of the protocol connection.
    /// 
    /// # Return
    /// 
    /// `true` if this protocol layer is connected, `false` otherwise.
    fn is_connected(&self) -> bool;

    /// Sets the use of long packets.
    /// 
    /// # Parameters
    /// 
    /// * `flag`: `true` if we should use long packet support, `false` otherwise.
    fn use_long_packets(&self, flag: bool);

    /// Reads a packet from the underlying communication subsystem.
    /// 
    /// # Return
    /// 
    /// A `GenericPacket` object containing the read data.
    fn read_packet(&self) -> GenericPacket;

    /// Transmits a packet to the underlying communication subsystem.
    /// 
    /// # Parameters
    /// * `data`: The data to transmit.
    /// * `src_socket`: The socket that was the source of this data (may be ignored).
    /// * `dest_socket`: The socket that is the intended destination of this data (may be ignored).
    fn transmit_packet(&self, data: Vec<u8>, src_socket: i8, dest_sock: i8);
}

/// CMP Packet Trait
///
/// Defines CMPPacket converter
pub trait CMPPacketTrait {
    /// Converts this packet to a byte array, suitable for transmission.
    ///
    /// # Return
    ///
    /// An array of bytes containing the byte representation of this packet object.
    fn packet_to_bytes(&self) -> Vec<u8>;

    /// Convert an array of bytes containing a valid CMP packet into an instance of `CMPPacket`.
    ///
    /// # Parameters
    ///
    /// * `pkt`: The byte array containing a CMP packet to convert.
    ///
    /// # Return
    ///
    /// The `CMPPacket` object containing the data from the byte array.
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

/// DLP Version
/// 
/// A class for handling DLP's Version information block format.
/// 
/// This class holds version information, and can convert to and from 
/// DLP version information and Java primitive types.
/// 
/// The Palm uses a four byte format for storing version information in a
/// `major_version.minor_version` format.
#[derive(Serialize, Deserialize)]
pub struct DLPVersion {
    pub major_version: u8,
    pub minor_version: u8,
}

impl<'a> CMPDLP<'a> {
    /// Attempt to listen for a connection to the remote CMP/DLP enabled device.
    ///
    /// This method will block until a connection is initiated,
    /// so it's safe to call and wait until a connection occurs.
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

    /// A method to cause the protocol stack issue a disconnect request.
    pub fn disconnect(&self) {
        self.disconnect_with_reason('\0');
    }

    /// A method to cause the protocol stack issue a disconnect request
    /// with the specified disconnect reason code.
    /// 
    /// # Parameter
    /// 
    /// * `c`: The reason code for the disconnect.
    pub fn disconnect_with_reason(&self, reason: char) {
        // TODO: Implement
    }

    // TODO: public DLP_Packet getDLPPacket(DLP_Packet dlp_packet)

    /// Retreives the speed of the connection.
    /// This value is only valid for serial based connections.
    /// Other connections will return the default speed value of 9600bps, 
    /// and should be ignored.
    /// 
    /// # Return
    /// 
    /// The speed of the serial connection.
    pub fn get_speed(&self) -> u32 {
        self.speed
    }


    /// Returns the connection status of this protocol layer.
    /// 
    /// # Return
    /// 
    /// `true` if we're connected to the remote device still, `false` otherwise.
    pub fn is_connected(&self) -> bool {
        self.connected
    }

    /// Sets the speed that this protocol should attempt to use for serial synchronization sessions.
    /// This method has no effect for non-serial synchronization.
    ///
    /// # Parameters
    /// 
    /// * `new_value`: The speed that the synchronization should be attempted at.
    pub fn set_speed(&mut self, new_value: u32) {
        self.speed = new_value
    }

    /// Suspends the synchronization.
    /// Calling this method closes down the current synchronization session,
    /// while leaving the protocol stack in tact so it can begin another 
    /// synchronization session as soon as this method returns.
    /// This method will also attempt to suspend the protocol layer beneath it.
    pub fn suspend(&self) {
        // TODO: byte data[][] = {{0, 0}};

        if !self.connected {
            return
        }

        /* TODO: DLP_Packet dlp_packet = new DLP_Packet((byte)DLP_Packet.END_OF_SYNC, data);
        dlp_packet.hostSocket = 3;
        dlp_packet.serverSocket = 3;

        try {
            getDLPPacket(dlp_packet);
        } catch(DLPError ex) {}
        
        connected = false;
        padpHandler.suspendConnection(); */
    }

    // TODO: public byte[] readRawPacket() throws NotConnectedException {

    // TODO: public void writeRawPacket(byte[] data, byte srcSocket, byte destSocket) throws NotConnectedException {

    /// Connection function for PADP
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
            flags = CMPPacket::FLAG_CHANGE_BAUD_RATE;
        }

        if cmp_pkt.test_flag(CMPPacket::FLAG_LONG_PACKET_SUPPORT) {
            flags |= CMPPacket::FLAG_LONG_PACKET_SUPPORT;
            self.padp_handler.use_long_packets(true);
        } else {
            self.padp_handler.use_long_packets(false);
        }

        self.padp_handler.transmit_packet(
            new_cmp_packet_with_settings(CMP_INIT, flags, 0, 0, self.speed).packet_to_bytes(),
            3,
            3,
        )
    }

    /// USB Receive Handshake.
    fn usb_rx_handshake(&self) {
        let mut pkt: GenericPacket;
        // TODO: USB RX Handshake
    }
}

impl CMPPacket {
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
    /// Transaction ID for CMP Wake Up Packet
    pub const WAKEUP_TID: i8 = -1;
    /// Version mismatched on either end of the synchronization
    pub const VERSION_MISMATCH: u8 = 0x80;
    /// Default speed for Serial connection
    pub const DEFAULT_SPEED: u32 = 9600;

    /// Test if flag is set
    pub fn test_flag(&self, flag: u8) -> bool {
        (self.flags & flag) == flag
    }
}

impl CMPPacketTrait for CMPPacket {
    /// Packet to Bytes
    ///
    /// # Bytes Definition
    ///
    /// * 0: Packet Type
    /// * 1: Flags
    /// * 2: Major Version
    /// * 3: Minor Version
    /// * 4-5: 0 (Padding)
    /// * 6-9: Baud Rate
    ///
    /// # Return
    ///
    /// An array of bytes containing the byte representation of this packet object.
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

    /// Bytes to Packet
    ///
    /// # Byte Definition
    ///
    /// * 0: Packet Type
    /// * 1: Flags
    /// * 2: Major Version
    /// * 3: Minor Version
    /// * 4-5: 0 (Padding)
    /// * 6-9: Baud Rate
    ///
    /// # Parameters
    ///
    /// * `pkt`: The byte array containing a CMP packet to convert.
    ///
    /// # Return
    ///
    /// The `CMPPacket` object containing the data from the byte array.
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

/// Construct an instance of the `CMPDLP` class using
/// the specified underlying `CMPDLPTransferTrait` protocol handler.
///
/// # Parameters
///
/// * `padp`: The `CMPDLPTransferTrait` protocol handler to use for I/O
///
/// # Return
///
/// A `CMPDLP` instance
pub fn new_cmp_dlp<'a>(padp: &'a dyn CMPDLPTransferTrait) -> CMPDLP {
    CMPDLP {
        padp_handler: padp,
        connected: false,
        speed: CMPPacket::DEFAULT_SPEED,
    }
}

/// Construct an instance of the `CMPPacket` class
///
/// # Return
///
/// A `CMPPacket` instance
pub fn new_cmp_packet() -> CMPPacket {
    CMPPacket {
        packet_type: 0,
        flags: 0,
        major_version: 0,
        minor_version: 0,
        baud_rate: CMPPacket::DEFAULT_SPEED,
    }
}

/// Construct an instance of the `CMPPacket` class with settings
///
/// # Parameters
///
/// * `pkt_type`: The CMP packets type.
/// * `flags`: The flags to attach to this packet.
/// * `major_version`: The major protocol version.
/// * `minor_version`: The minor protocol version.
/// * `baud`: The serial rate to use for serial synchronization.
///
/// # Return
///
/// A `CMPPacket` instance with settings
pub fn new_cmp_packet_with_settings(
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

/// Construct an instance of the `DLPVersion` class as version 1.0.
///
/// # Parameters
///
/// * `pkt_type`: The CMP packets type.
/// * `flags`: The flags to attach to this packet.
/// * `major_version`: The major protocol version.
/// * `minor_version`: The minor protocol version.
/// * `baud`: The serial rate to use for serial synchronization.
///
/// # Return
///
/// A `DLPVersion` instance
pub fn new_dlp_version() -> DLPVersion {
    DLPVersion {
        major_version: 1,
        minor_version: 0
    }
}


/// Construct a new DLPVersion object using the specified version information.
///
/// # Return
///
/// A `DLPVersion` instance with specified version
pub fn new_dlp_version_with_settings(major_version: u8, minor_version: u8) -> DLPVersion {
    DLPVersion {
        major_version: major_version,
        minor_version: minor_version
    }
}

/// A method to determine the type of incoming packet.
///
/// # Parameters
/// * `generic_packet`: A `GenericPacket` to be determined.
///
/// # Return
///
/// * True if this packet is a PADP packet.
/// * False otherwise.
fn determine_packet_type(generic_packet: &GenericPacket) -> bool {
    generic_packet.data[0] >= 16
}

use super::packet_timer::PacketTimerTrait;
use super::slp::SLP;

pub const DEFAULT_QUEUE_LENGTH: i32 = 4;
pub const TICKLE_TIME: i64 = 3000;

// States
pub const STATE_DISCONNECTED: i32 = 0;
pub const STATE_SUSPENDED: i32 = 1;
pub const STATE_CONNECTED: i32 = 2;

// TODO: Add Thread
pub struct PADP<'a> {
    slp_handler: SLP,
    ack_timer: &'a dyn PacketTimerTrait,
    inter_packet_timer: &'a dyn PacketTimerTrait,
    tickle_timer: &'a dyn PacketTimerTrait,
    connect_state: i32,
    trans_id: i8,
    retries: i32,
    debug_mode: bool,
    more: bool,
    tx: bool,
    use_long_packets: bool

    // TODO: PADP_Packet lastFragSent
    // TODO: PADPFragmentSet outputQueue
    // TODO: PADPFragmentSet inputQueue
    // TODO: CircularQueue readyPackets
}

impl<'a> PADP<'a> {
    #[inline]
    fn queue_length() -> i32 {
        DEFAULT_QUEUE_LENGTH
    }

    #[inline]
    fn ack_timeout() -> i64 {
        2000
    }

    #[inline]
    fn ip_timeout() -> i64 {
        10000
    }

    #[inline]
    fn max_send_retries() -> i32 {
        10
    }
}
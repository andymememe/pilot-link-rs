pub trait PacketTimerTrait {
    fn timer_expires(&self, packet_timer: dyn PacketTimerTrait);
}
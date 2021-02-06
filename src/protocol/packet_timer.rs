pub trait PacketTimer {
    fn timer_expires(&self, packet_timer: dyn PacketTimer);
}
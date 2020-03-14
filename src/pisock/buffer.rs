pub fn pi_buffer_expect(buf: &mut Vec<u8>, capacity: usize) {
    buf.resize(capacity, 0);
}

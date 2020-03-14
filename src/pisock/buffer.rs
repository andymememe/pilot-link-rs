pub fn pi_buffer_expect(buf: &mut Vec<u8>, capacity: usize) {
    buf.resize(buf.len() + capacity, 0);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pi_buffer_expect() {
        let mut v: Vec<u8> = vec![1, 2, 3];
        pi_buffer_expect(&mut v, 2);
        assert_eq!(v.capacity() >= 5, true);
    }
}

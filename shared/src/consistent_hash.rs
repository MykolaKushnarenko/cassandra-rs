pub struct ConsistentHash;

impl ConsistentHash {
    pub fn calculate_hash(value: &str) -> u64 {
        let hash = murmur3::murmur3_x64_128(
            &mut std::io::Cursor::new(value),
            0
        ).unwrap();
        hash as u64
    }
}

#[cfg(test)]
#[test]
fn test_hash() {
    let ip = "127.0.0.1:3000";
    let hash = ConsistentHash::calculate_hash(ip);
    assert_eq!(hash, 2784727742823359555);
}
// Minimal implementation of the Jump Consistent Hash algorithm, originally defined in
// the 'A Fast, Minimal Memory, Consistent Hash Algorithm' [1] paper, implemented a close
// to the published C++ code as possible for clarity.
//
// Test vectors from the rust-jumphash and jmphash-rs projects [2,3].
//
// [1] https://arxiv.org/pdf/1406.2294v1.pdf
// [2] https://github.com/jeromefroe/jmphash-rs
// [3] https://github.com/jedisct1/rust-jumphash
pub fn jump_consistent_hash (mut key: u64, num_buckets: u32) -> u32 {
    let     x = (1u64 << 31) as f64;
    let mut b = -1i64;
    let mut j =  0i64;
    while j < num_buckets as i64 {
        b = j;
        key = key.wrapping_mul(2862933555777941757).wrapping_add(1);
        j = ( (b.wrapping_add(1) as f64) * ( x / (((key >> 33) + 1) as f64) ) ) as i64;
    }
    b as u32
}

#[cfg(test)]
mod tests {
    use crate::jump_consistent_hash;
    use std::hash::{Hasher, Hash};
    use std::collections::hash_map::DefaultHasher;

    fn jumphash <T> (key:T, num_buckets:u32) -> u32 where T:Hash {
        jump_consistent_hash(hash(key), num_buckets)
    }

    fn hash <T> (value:T) -> u64 where T: Hash {
        let mut hasher = DefaultHasher::new(); // uses SipHash today, but DefaultHasher does not guarentee consistent value over Rust versions!
        value.hash(&mut hasher);
        hasher.finish()
    }

    #[test]
    fn simple_test_vectors() {
        assert_eq!(jump_consistent_hash(123456, 1000), 984);
        assert_eq!(jumphash("test1", 10000000), 8970050);
        assert_eq!(jumphash("test2", 1000), 10);
        assert_eq!(jumphash("test3", 1000), 76);
        assert_eq!(jumphash("test4", 1000), 161);
        assert_eq!(jumphash("test5", 50), 33);
        assert_eq!(jumphash("", 1000), 392);
        assert_eq!(jumphash("testz", 1), 0);
    }
}

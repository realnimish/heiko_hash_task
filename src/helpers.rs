use crate::aggregator;
use rand::Rng;

pub const MIN_HASHES: usize = 0;
pub const MAX_HASHES: usize = 100;

pub fn generate_data_point() -> (
    Vec<[u64; aggregator::HASH_LENGTH_U64]>,
    [u64; aggregator::HASH_LENGTH_U64],
) {
    let hashes = generate_random_hashes();
    let mut expected = [0u64; aggregator::HASH_LENGTH_U64];
    for i in 0..hashes.len() {
        expected = add_hashes(&expected, &hashes[i]);
    }

    (hashes, expected)
}

pub fn generate_random_hashes() -> Vec<[u64; aggregator::HASH_LENGTH_U64]> {
    let mut rng = rand::thread_rng();
    let len = rng.gen_range(MIN_HASHES..=MAX_HASHES);
    let mut vec = Vec::with_capacity(len);
    for _ in 0..len {
        vec.push(generate_random_hash());
    }
    vec
}

fn generate_random_hash() -> [u64; aggregator::HASH_LENGTH_U64] {
    generate_random_u64_array::<{ aggregator::HASH_LENGTH_U64 }>()
}

fn generate_random_u64_array<const N: usize>() -> [u64; N] {
    let mut rng = rand::thread_rng();
    let mut arr = [0u64; N];
    for elem in &mut arr {
        *elem = rng.gen();
    }
    arr
}

pub fn add_hashes<const N: usize>(hash_a: &[u64; N], hash_b: &[u64; N]) -> [u64; N] {
    let mut res = [0u64; N];
    let mut carry = false;

    for i in 0..N {
        let (sum, overflow_1) = hash_a[i].overflowing_add(hash_b[i]);
        let (sum_with_overflow, overflow_2) = sum.overflowing_add(carry as u64);
        res[i] = sum_with_overflow;
        carry = overflow_1 | overflow_2;
    }

    res
}

use std::{sync::Arc, thread};
pub const HASH_LENGTH_U64: usize = 63;
pub const THREAD_COUNT: usize = 3;

pub fn aggregate_hashes<T>(hashes: T) -> [u64; HASH_LENGTH_U64]
where
    T: AsRef<[[u64; HASH_LENGTH_U64]]>,
{
    let hashes = hashes.as_ref();
    let mut res = [0u64; HASH_LENGTH_U64];
    for hash in hashes {
        append_hash(&mut res, hash);
    }
    res
}

fn append_hash(base_hash: &mut [u64; HASH_LENGTH_U64], hash: &[u64; HASH_LENGTH_U64]) {
    let mut carry = 0u64;

    for (base, &h) in base_hash.iter_mut().zip(hash.iter()) {
        let (sum, overflow) = base.overflowing_add(h);
        let (sum_with_carry, carry_overflow) = sum.overflowing_add(carry);
        *base = sum_with_carry;
        carry = (overflow | carry_overflow) as u64;
    }
}

pub fn aggregate_hashes_by_parts<T>(hashes: T) -> [u64; HASH_LENGTH_U64]
where
    T: AsRef<[[u64; HASH_LENGTH_U64]]>,
{
    let hashes = hashes.as_ref();
    let mut res = [0u64; HASH_LENGTH_U64];
    let mut carry = 0u64;

    (0..HASH_LENGTH_U64).for_each(|index| {
        (res[index], carry) = append_hashes_by_parts(hashes, index, carry);
    });

    res
}

pub fn parallel_aggregate_hashes_by_parts<T>(hashes: T) -> [u64; HASH_LENGTH_U64]
where
    T: AsRef<[[u64; HASH_LENGTH_U64]]> + Send + Sync + 'static,
{
    let hashes = Arc::new(hashes);
    let size_per_thread = HASH_LENGTH_U64 / THREAD_COUNT;
    let mut handles = vec![];

    for i in 0..THREAD_COUNT {
        let shared_hashes = Arc::clone(&hashes);
        let start = size_per_thread * i;
        let end = if i == THREAD_COUNT - 1 {
            HASH_LENGTH_U64
        } else {
            start + size_per_thread
        };
        let len = end - start + 1;
        let mut ranged_res = Vec::with_capacity(len);
        let mut ranged_carry_out = Vec::with_capacity(len);

        let handle = thread::spawn(move || {
            let shared_hashes = shared_hashes.as_ref();
            for index in start..end {
                let (sum, carry_out) = append_hashes_by_parts(shared_hashes, index, 0u64);
                ranged_res.push(sum);
                ranged_carry_out.push(carry_out);
            }

            (ranged_res, ranged_carry_out)
        });

        handles.push(handle);
    }

    let mut flatten_res = Vec::with_capacity(HASH_LENGTH_U64);
    let mut flatten_carry_out = Vec::with_capacity(HASH_LENGTH_U64);

    // Wait for all threads to finish
    for h in handles {
        let (partial_res, partial_carry) = h.join().expect("Failed to join");
        flatten_res.extend(partial_res);
        flatten_carry_out.extend(partial_carry);
    }

    // Ripple add the carry_out forward
    let mut res = [0u64; HASH_LENGTH_U64];
    res[0] = flatten_res[0];
    for index in 1..HASH_LENGTH_U64 {
        let (sum, overflow) = flatten_res[index].overflowing_add(flatten_carry_out[index - 1]);
        if overflow {
            flatten_carry_out[index] = flatten_carry_out[index].checked_add(1).unwrap_or_else(|| {
                // For MSB, carry-out is discarded anyways
                if index != HASH_LENGTH_U64 - 1 {
                    panic!("carry overflow!")
                }
                flatten_carry_out[index]
            })
        }
        res[index] = sum;
    }

    res
}

// Returns (sum: u64, carry: u64)
fn append_hashes_by_parts<T>(hashes: T, idx: usize, carry_sum: u64) -> (u64, u64)
where
    T: AsRef<[[u64; HASH_LENGTH_U64]]>,
{
    hashes
        .as_ref()
        .iter()
        .fold((carry_sum, 0u64), |(sum, mut carry), hash| {
            let (sum, overflow) = sum.overflowing_add(hash[idx]);
            if overflow {
                // Caveat of doing aggregation using "by parts" approach
                carry = carry.checked_add(1).expect("Carry overflow!");
            }
            (sum, carry)
        })
}

use crate::aggregator;
use crate::helpers;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregate_hashes_returns_correct_hash_sum() {
        let hashes = helpers::generate_random_hashes();
        let res = aggregator::aggregate_hashes(&hashes);

        let mut expected = [0u64; aggregator::HASH_LENGTH_U64];
        for i in 0..hashes.len() {
            expected = helpers::add_hashes(&expected, &hashes[i]);
        }

        assert_eq!(
            res, expected,
            "Aggregating multiple hashes should return a correct hash sum"
        );
    }

    #[test]
    fn test_aggregate_by_parts_returns_correct_hash_sum() {
        let hashes = helpers::generate_random_hashes();
        let res = aggregator::aggregate_hashes_by_parts(&hashes);

        let mut expected = [0u64; aggregator::HASH_LENGTH_U64];
        for i in 0..hashes.len() {
            expected = helpers::add_hashes(&expected, &hashes[i]);
        }

        assert_eq!(
            res, expected,
            "Aggregating multiple hashes should return a correct hash sum"
        );
    }
}

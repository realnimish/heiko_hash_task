use crate::aggregator;
use crate::helpers;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregate_hashes_returns_correct_hash_sum() {
        let (hashes, expected) = helpers::generate_data_point();
        let res = aggregator::aggregate_hashes(&hashes);

        assert_eq!(
            res, expected,
            "Aggregating multiple hashes should return a correct hash sum"
        );
    }

    #[test]
    fn test_aggregate_by_parts_returns_correct_hash_sum() {
        let (hashes, expected) = helpers::generate_data_point();
        let res = aggregator::aggregate_hashes_by_parts(&hashes);

        assert_eq!(
            res, expected,
            "Aggregating multiple hashes should return a correct hash sum"
        );
    }

    #[test]
    fn test_parallel_aggregate_by_parts_returns_correct_hash_sum() {
        let (hashes, expected) = helpers::generate_data_point();
        let res = aggregator::parallel_aggregate_hashes_by_parts(hashes);

        assert_eq!(
            res, expected,
            "Aggregating multiple hashes should return a correct hash sum"
        );
    }
}

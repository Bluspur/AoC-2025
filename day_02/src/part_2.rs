use super::*;
use itertools::Itertools;
use log::info;
use rayon::prelude::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PartError {
    #[error(transparent)]
    ParseError(#[from] ParseError),
}
pub fn run(input: &str) -> Result<u64, PartError> {
    let db = input.parse::<ShopDatabase>()?;
    Ok(sum_all_invalid_in_db_multi(&db))
}
// FOR COMPARISONS BETWEEN SINGLE AND MULTI THREADING
// According to benchmarks (see the benches folder), the single-threaded version
// is faster than the multi-threaded version. The test input was used so I guess it is still
// kind of small.
pub fn sum_all_invalid_in_db_multi(db: &ShopDatabase) -> u64 {
    db.ranges
        .iter()
        .map(|range| sum_all_invalid_in_range_multi(range))
        .sum()
}
pub fn sum_all_invalid_in_db_single(db: &ShopDatabase) -> u64 {
    db.ranges
        .iter()
        .map(|range| sum_all_invalid_in_range_single(range))
        .sum()
}
pub fn sum_all_invalid_in_range_single(range: &Range) -> u64 {
    (range.start..=range.end)
        .into_iter()
        .filter(|&n| is_number_invalid(n))
        .sum()
}
pub fn sum_all_invalid_in_range_multi(range: &Range) -> u64 {
    (range.start..=range.end)
        .into_par_iter()
        .filter(|&n| is_number_invalid(n))
        .sum()
}
pub fn is_number_invalid(n: u64) -> bool {
    let len = count_digits(n);
    let factors = get_factors(len as u64);
    for number_of_segments in factors {
        let segments = split_number(n, len, number_of_segments);
        info!("n: {}, f: {}, segs: {:?}", n, number_of_segments, segments);
        if segments.iter().all_equal() {
            return true;
        }
    }

    false
}
/// Returns all factors of a number.
pub fn get_factors(n: u64) -> Vec<u32> {
    if n <= 1 {
        return vec![]; // Special Case
    }
    let mut factors = vec![n as u32];
    // Calculate the square root of the number.
    let sqrt = n.isqrt();
    for i in 2..=sqrt {
        if n % i == 0 {
            factors.push(i as u32);
            if i != n / i {
                factors.push((n / i) as u32);
            }
        }
    }
    factors.sort_unstable();
    info!("Factors of {} are {:?}", n, factors);
    factors
}
fn split_number(n: u64, l: u32, s: u32) -> Vec<u64> {
    let mut n = n;
    let mut v = vec![0; s as usize];
    for i in 0..s as usize {
        let p1 = n / 10_u64.pow(l / s);
        let p2 = n % 10_u64.pow(l / s);
        v[i] = p2;
        n = p1;
    }
    v
}

#[cfg(test)]
mod tests {
    use super::*;

    // Per the test data.
    fn setup_test_database() -> ShopDatabase {
        ShopDatabase::new(&[
            Range::new(11, 22),
            Range::new(95, 115),
            Range::new(998, 1012),
            Range::new(1188511880, 1188511890),
            Range::new(222220, 222224),
            Range::new(1698522, 1698528),
            Range::new(446443, 446449),
            Range::new(38593856, 38593862),
            Range::new(565653, 565659),
            Range::new(824824821, 824824827),
            Range::new(2121212118, 2121212124),
        ])
    }

    #[test]
    fn test_get_factors() {
        let n = 12;
        let expected = vec![2, 3, 4, 6, 12];
        let factors = get_factors(n);
        assert_eq!(factors, expected);
    }

    #[test]
    fn test_get_factors_single_digit() {
        let n = 1;
        let expected = vec![];
        let factors = get_factors(n);
        assert_eq!(
            factors, expected,
            "Expected empty vector for single digit, got {:?}",
            factors
        );
    }

    #[test]
    fn test_get_factors_single_digit_greater_than_one() {
        let n = 2;
        let expected = vec![2];
        let factors = get_factors(n);
        assert_eq!(
            factors, expected,
            "Expected vector with single element for single digit greater than one, got {:?}",
            factors
        );
    }

    #[test]
    fn test_get_factors_zero() {
        let n = 0;
        let expected = vec![];
        let factors = get_factors(n);
        assert_eq!(
            factors, expected,
            "Expected empty vector for zero, got {:?}",
            factors
        );
    }

    #[test]
    fn test_split_number() {
        let n = 123456;
        let l = 6;
        let s = 2;
        let expected = vec![123, 456];
        let mut result = split_number(n, l, s);
        result.sort_unstable(); // Just for the sake of comparison, order does not really matter otherwise.
        assert_eq!(result, expected);
    }

    #[test]
    fn test_is_number_invalid() {
        let test_pairs = [
            (1, false),
            (11, true),
            (12, false),
            (1010, true),
            (1011, false),
            (565656, true),
        ];

        for (n, expected) in test_pairs {
            let result = is_number_invalid(n);
            assert_eq!(
                result, expected,
                "Failed for n = {}, expected = {}",
                n, expected
            );
        }
    }

    #[test]
    fn test_sum_all_invalid_numbers_in_range() {
        let range_sum_pair = [
            (Range::new(11, 22), 11 + 22),
            (Range::new(95, 115), 99 + 111),
            (Range::new(998, 1012), 999 + 1010),
            (Range::new(1188511880, 1188511890), 1188511885),
            (Range::new(222220, 222224), 222222),
            (Range::new(1698522, 1698528), 0),
            (Range::new(446443, 446449), 446446),
            (Range::new(38593856, 38593862), 38593859),
            (Range::new(565653, 565659), 565656),
            (Range::new(824824821, 824824827), 824824824),
            (Range::new(2121212118, 2121212124), 2121212121),
        ];
        for (range, expected) in range_sum_pair {
            let result = sum_all_invalid_in_range_single(&range);
            assert_eq!(
                result, expected,
                "Failed for range = {:?}, expected = {}",
                range, expected
            );
        }
    }

    #[ignore]
    #[test]
    fn test_sum_all_invalid_numbers_in_db() {
        let db = setup_test_database();
        let expected = 4174379265; // From website.
        let result = sum_all_invalid_in_db_multi(&db);
        assert_eq!(
            result, expected,
            "Failed for db = {:?}, expected = {}",
            db, expected
        );
    }
}

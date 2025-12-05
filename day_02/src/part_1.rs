use super::*;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PartError {
    #[error(transparent)]
    ParseError(#[from] ParseError),
}
pub fn run(input: &str) -> Result<u64, PartError> {
    let db = input.parse::<ShopDatabase>()?;
    Ok(sum_invalid_all_ranges(&db))
}
pub fn sum_invalid_all_ranges(db: &ShopDatabase) -> u64 {
    db.ranges.iter().map(|range| sum_invalid(range)).sum()
}
pub fn sum_invalid(range: &Range) -> u64 {
    let mut sum = 0;
    for i in range.start..=range.end {
        if is_number_invalid(i) {
            sum += i;
        }
    }
    sum
}
fn is_number_invalid(n: u64) -> bool {
    // Get the number of digits in the number.
    let len = count_digits(n);
    // Only even numbers can be invalid according to the problem statement.
    if len % 2 == 0 {
        let (left, right) = split_number(n, len);
        // Check if the left and right parts are equal.
        return left == right;
    } else {
        false
    }
}
fn split_number(n: u64, l: u32) -> (u64, u64) {
    let left = n / 10_u64.pow(l / 2);
    let right = n % 10_u64.pow(l / 2);
    (left, right)
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
    fn test_count_digits() {
        assert_eq!(count_digits(1), 1);
        assert_eq!(count_digits(1234), 4);
        assert_eq!(count_digits(12345), 5);
    }
    #[test]
    fn test_split_number() {
        assert_eq!(split_number(1234, count_digits(1234)), (12, 34));
        assert_eq!(split_number(12345, count_digits(12345)), (123, 45));
        assert_eq!(split_number(38593859, count_digits(38593859)), (3859, 3859));
    }
    #[test]
    fn test_sum_invalid() {
        let range = Range::new(11, 22);
        let expected = 11 + 22;
        let actual = sum_invalid(&range);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_sum_invalid_no_candidates() {
        let range = Range::new(1698522, 1698528);
        let expected = 0;
        let actual = sum_invalid(&range);
        assert_eq!(actual, expected);
    }
    #[test]
    fn test_sum_invalid_database() {
        let db = setup_test_database();
        let expected = 1227775554;
        let actual = sum_invalid_all_ranges(&db);
        assert_eq!(actual, expected);
    }
}

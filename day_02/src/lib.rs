use anyhow::{Context, Result};
use std::str::FromStr;
use thiserror::Error;

pub mod part_1;
pub mod part_2;

pub fn run_part_1(input: &str) -> Result<u64> {
    part_1::run(input).context("Failed to run part 1")
}

pub fn run_part_2(input: &str) -> Result<u64> {
    part_2::run(input).context("Failed to run part 2")
}

/// Returns the number of digits in the given number.
pub fn count_digits(n: u64) -> u32 {
    n.ilog10() + 1
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Expected range to be in format 11-22 but found: {0}")]
    SplitRangeFailed(String),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShopDatabase {
    pub ranges: Vec<Range>,
}
impl ShopDatabase {
    pub fn new(ranges: &[Range]) -> Self {
        Self {
            ranges: ranges.to_vec(),
        }
    }
}
impl FromStr for ShopDatabase {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let range_parts = s.trim().split(',');
        let ranges = range_parts
            .map(|part| part.parse::<Range>())
            .collect::<Result<_, _>>()?;
        Ok(Self { ranges })
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Range {
    start: u64,
    end: u64,
}
impl Range {
    pub fn new(start: u64, end: u64) -> Self {
        Self { start, end }
    }
}
impl FromStr for Range {
    type Err = ParseError;
    // Input should be formed as two numbers seperated by a hyphen.
    // e.g. 11-22 or 95-115
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('-');
        let start = parts
            .next()
            .ok_or(ParseError::SplitRangeFailed(s.to_string()))?
            .parse::<u64>()?;
        let end = parts
            .next()
            .ok_or(ParseError::SplitRangeFailed(s.to_string()))?
            .parse::<u64>()?;
        Ok(Self { start, end })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_range() {
        let range = Range::from_str("11-22").unwrap();
        assert_eq!(range.start, 11);
        assert_eq!(range.end, 22);
    }
    // Just compare a minimal set, it doesn't need to be exhaustive.
    #[test]
    fn test_parse_database() {
        let minimal_input = "11-22,95-115,998-1012";
        let db = ShopDatabase::from_str(minimal_input).unwrap();
        assert_eq!(db.ranges.len(), 3);
        assert_eq!(db.ranges[0], Range::new(11, 22));
        assert_eq!(db.ranges[1], Range::new(95, 115));
        assert_eq!(db.ranges[2], Range::new(998, 1012));
    }
}

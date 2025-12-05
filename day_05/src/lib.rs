use std::{num::ParseIntError, str::FromStr};

use anyhow::{Context, Result};
use thiserror::Error;

mod part_1;
mod part_2;
#[derive(Debug, PartialEq, Eq)]
pub struct Database {
    fresh_id_ranges: Vec<IdRange>,
    available_ids: Vec<IngredientId>,
}
impl Database {
    pub fn is_fresh(&self, id: IngredientId) -> bool {
        for range in self.fresh_id_ranges.iter() {
            if range.contains(id) {
                return true;
            }
        }
        false
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IdRange {
    min: IngredientId,
    max: IngredientId,
}
impl IdRange {
    pub fn new(min: IngredientId, max: IngredientId) -> Self {
        Self { min, max }
    }
    pub fn contains(&self, id: IngredientId) -> bool {
        id >= self.min && id <= self.max
    }
    pub fn len(&self) -> usize {
        self.max - self.min + 1
    }
}

pub type IngredientId = usize;

impl FromStr for Database {
    type Err = ParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let (raw_ranges, raw_available) = s
            .trim()
            .split_once("\n\n")
            .ok_or(ParseError::NoDBSeperator)?;
        let fresh_id_ranges = parse_ranges(raw_ranges)?;
        let available_ids = parse_available(raw_available)?;
        let db = Database {
            fresh_id_ranges,
            available_ids,
        };
        Ok(db)
    }
}

fn parse_ranges(s: &str) -> Result<Vec<IdRange>, ParseError> {
    let mut set = Vec::new();
    for raw_range in s.lines() {
        let (min, max) = raw_range
            .split_once('-')
            .ok_or(ParseError::NoRangeSeperator)?;
        let min = IngredientId::from_str(min)?;
        let max = IngredientId::from_str(max)?;
        let range = IdRange { min, max };
        set.push(range);
    }
    Ok(set)
}
fn parse_available(s: &str) -> Result<Vec<IngredientId>, ParseError> {
    let mut available = Vec::new();
    for raw_id in s.lines() {
        let id = IngredientId::from_str(raw_id)?;
        available.push(id);
    }
    Ok(available)
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Expected input to have a blank line between ranges and available")]
    NoDBSeperator,
    #[error("Expected range to have a '-' between the min and max bounds")]
    NoRangeSeperator,
    #[error(transparent)]
    BadID(#[from] ParseIntError),
}

pub fn run_part_1(input: &str) -> Result<u64> {
    part_1::run(input).context("Failed to run part 1")
}

pub fn run_part_2(input: &str) -> Result<u64> {
    part_2::run(input).context("Failed to run part 2")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_minimal_test_db() -> Database {
        let ranges = [IdRange::new(1, 9)].into();
        let available = [3, 7].into();
        Database {
            fresh_id_ranges: ranges,
            available_ids: available,
        }
    }

    #[test]
    fn test_parse_minimal_happy_path() {
        let db_string = "1-9\n\n3\n7\n";
        let db_expected = create_minimal_test_db();
        let db_actual = Database::from_str(db_string).unwrap();
        assert_eq!(db_expected, db_actual);
    }
}

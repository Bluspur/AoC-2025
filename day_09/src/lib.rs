use std::str::FromStr;

use anyhow::{Context, Result};
use thiserror::Error;

mod part_1;
mod part_2;

pub fn run_part_1(input: &str) -> Result<u64> {
    part_1::run(input).context("Failed to run part 1")
}

pub fn run_part_2(input: &str) -> Result<u64> {
    part_2::run(input).context("Failed to run part 2")
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}
impl IVec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
impl FromStr for IVec2 {
    type Err = ParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        s.split_once(',')
            .ok_or(ParseError::MissingSeparator)
            .and_then(|(x, y)| {
                Ok(IVec2 {
                    x: x.parse()?,
                    y: y.parse()?,
                })
            })
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Tiles {
    pub tiles: Vec<IVec2>,
}
impl FromStr for Tiles {
    type Err = ParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let tiles = s
            .trim()
            .lines()
            .map(|line| line.parse())
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Tiles { tiles })
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid format")]
    MissingSeparator,
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn raw_tiles() -> &'static str {
        "7,1\n11,1\n11,7\n9,7\n9,5\n2,5\n2,3\n7,3\n"
    }

    fn setup_test_input() -> Tiles {
        Tiles {
            tiles: vec![
                IVec2 { x: 7, y: 1 },
                IVec2 { x: 11, y: 1 },
                IVec2 { x: 11, y: 7 },
                IVec2 { x: 9, y: 7 },
                IVec2 { x: 9, y: 5 },
                IVec2 { x: 2, y: 5 },
                IVec2 { x: 2, y: 3 },
                IVec2 { x: 7, y: 3 },
            ],
        }
    }

    #[test]
    fn test_parse_input() {
        let input = raw_tiles();
        let expected = setup_test_input();
        let actual = Tiles::from_str(input).unwrap();
        assert_eq!(actual, expected);
    }
}

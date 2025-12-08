use std::{collections::HashSet, str::FromStr};

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
        IVec2 { x, y }
    }
}

fn split(col: i32) -> (i32, i32) {
    (col - 1, col + 1)
}

#[derive(Debug, PartialEq, Eq)]
pub struct TachyonManifold {
    start: IVec2,
    rows: usize,
    mirrors: Vec<IVec2>,
}
impl TachyonManifold {
    fn count_reflections(&self) -> u64 {
        let mut count = 0;
        let mut cols = HashSet::from([self.start.x]);
        for mirror in self.mirrors.iter() {
            let col = mirror.x;
            if cols.remove(&col) {
                count += 1;
                let (l, r) = split(col);
                cols.insert(l);
                cols.insert(r);
            }
        }

        count
    }
}
impl FromStr for TachyonManifold {
    type Err = ParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let rows = s.trim().lines();
        let mut mirrors = vec![];
        let mut start = None;
        for (y, row) in rows.enumerate() {
            for (x, c) in row.char_indices() {
                match c {
                    '.' => continue,
                    'S' => start = Some(IVec2::new(x as i32, y as i32)),
                    '^' => mirrors.push(IVec2::new(x as i32, y as i32)),
                    _ => return Err(ParseError::InvalidChar(c)),
                }
            }
        }
        let rows = s.trim().lines().count();
        let start = start.ok_or(ParseError::NoStartLocation)?;
        Ok(Self {
            start,
            rows,
            mirrors,
        })
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Expected to find a start location, but none was found")]
    NoStartLocation,
    #[error("Encountered invalid char in input: {0}")]
    InvalidChar(char),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn raw_input() -> &'static str {
        r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#
    }
    /// Helper function to make writing points really short.
    fn p(x: i32, y: i32) -> IVec2 {
        IVec2::new(x, y)
    }
    fn setup_test_input() -> TachyonManifold {
        TachyonManifold {
            start: p(7, 0),
            rows: 16,
            mirrors: vec![
                p(7, 2),
                p(6, 4),
                p(8, 4),
                p(5, 6),
                p(7, 6),
                p(9, 6),
                p(4, 8),
                p(6, 8),
                p(10, 8),
                p(3, 10),
                p(5, 10),
                p(9, 10),
                p(11, 10),
                p(2, 12),
                p(6, 12),
                p(12, 12),
                p(1, 14),
                p(3, 14),
                p(5, 14),
                p(7, 14),
                p(9, 14),
                p(13, 14),
            ],
        }
    }

    #[test]
    fn test_parse_input() {
        let actual = TachyonManifold::from_str(&raw_input()).unwrap();
        let expected = setup_test_input();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_count_splits() {
        let manifold = setup_test_input();
        let expected = 21;
        let actual = manifold.count_reflections();

        assert_eq!(expected, actual);
    }
}

use std::str::FromStr;

use anyhow::{Context, Result};
use thiserror::Error;

mod part_1;
mod part_2;

pub fn run_part_1(input: &str, n_pairs: usize) -> Result<u64> {
    part_1::run(input, n_pairs).context("Failed to run part 1")
}

pub fn run_part_2(input: &str, n_pairs: usize) -> Result<u64> {
    part_2::run(input, n_pairs).context("Failed to run part 2")
}

#[derive(Debug, PartialEq, Eq)]
pub struct Decorations {
    junction_boxes: Vec<JunctionBox>,
}
impl Decorations {
    pub fn new(junction_boxes: &[JunctionBox]) -> Self {
        Decorations {
            junction_boxes: junction_boxes.to_vec(),
        }
    }
}
impl FromStr for Decorations {
    type Err = ParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let junction_boxes = s
            .trim()
            .lines()
            .map(|ln| JunctionBox::from_str(ln))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Decorations { junction_boxes })
    }
}
#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct JunctionBox {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
impl JunctionBox {
    pub fn distance(&self, other: &JunctionBox) -> f32 {
        let dx = self.x as f32 - other.x as f32;
        let dy = self.y as f32 - other.y as f32;
        let dz = self.z as f32 - other.z as f32;
        (dx.powi(2) + dy.powi(2) + dz.powi(2)).sqrt()
    }
    pub fn sq_distance(&self, other: &JunctionBox) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        dx.pow(2) + dy.pow(2) + dz.pow(2)
    }
}
impl FromStr for JunctionBox {
    type Err = ParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut parts = s.trim().split(',');
        let x = parts.next().ok_or(ParseError::CountMismatch(0))?.parse()?;
        let y = parts.next().ok_or(ParseError::CountMismatch(1))?.parse()?;
        let z = parts.next().ok_or(ParseError::CountMismatch(2))?.parse()?;
        Ok(JunctionBox { x, y, z })
    }
}

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid junction box count. Expected 3 parts, got {0} parts.")]
    CountMismatch(u32),
    #[error("Invalid junction box coordinate")]
    JunctionBoxCoordinateParseError(#[from] std::num::ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_input() -> Decorations {
        Decorations::new(&[
            jb(162, 817, 812),
            jb(57, 618, 57),
            jb(906, 360, 560),
            jb(592, 479, 940),
            jb(352, 342, 300),
            jb(466, 668, 158),
            jb(542, 29, 236),
            jb(431, 825, 988),
            jb(739, 650, 466),
            jb(52, 470, 668),
            jb(216, 146, 977),
            jb(819, 987, 18),
            jb(117, 168, 530),
            jb(805, 96, 715),
            jb(346, 949, 466),
            jb(970, 615, 88),
            jb(941, 993, 340),
            jb(862, 61, 35),
            jb(984, 92, 344),
            jb(425, 690, 689),
        ])
    }
    /// Helper for building a junction box quickly.
    fn jb(x: i32, y: i32, z: i32) -> JunctionBox {
        JunctionBox { x, y, z }
    }
    #[test]
    fn test_junction_box_distance() {
        let box1 = jb(1, 2, 3);
        let box2 = jb(4, 5, 6);
        assert_eq!(box1.distance(&box2), 5.196152);
    }

    #[test]
    fn test_parse_junction_box() {
        let input = "1,2,3";
        let result = JunctionBox::from_str(input);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), jb(1, 2, 3));
    }
}

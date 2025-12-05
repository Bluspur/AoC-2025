use std::{collections::HashMap, str::FromStr};

use anyhow::{Context, Result};
use thiserror::Error;

mod part_1;
mod part_2;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct IVec2 {
    pub x: i32,
    pub y: i32,
}
impl IVec2 {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    pub fn from_usize(x: usize, y: usize) -> Self {
        Self {
            x: x as i32,
            y: y as i32,
        }
    }
    pub fn neighbours(&self) -> [IVec2; 8] {
        [
            IVec2::new(self.x - 1, self.y - 1), // NW
            IVec2::new(self.x, self.y - 1),     // N
            IVec2::new(self.x + 1, self.y - 1), // NE
            IVec2::new(self.x - 1, self.y),     // W
            IVec2::new(self.x + 1, self.y),     // E
            IVec2::new(self.x - 1, self.y + 1), // SW
            IVec2::new(self.x, self.y + 1),     // S
            IVec2::new(self.x + 1, self.y + 1), // SE
        ]
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct PaperNode {
    pub neighbours: [IVec2; 8],
}
impl PaperNode {
    pub fn new(neighbours: [IVec2; 8]) -> Self {
        Self { neighbours }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grid {
    pub nodes: HashMap<IVec2, PaperNode>,
}
impl FromStr for Grid {
    type Err = ParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let rows = s.trim().lines();
        let mut nodes = HashMap::new();
        for (y, row) in rows.enumerate() {
            for (x, c) in row.char_indices() {
                let has_paper_roll = parse_char(c)?;
                let node_pos = IVec2::from_usize(x, y);
                if has_paper_roll {
                    let neighbours = node_pos.neighbours();
                    let node = PaperNode::new(neighbours);
                    nodes.insert(node_pos, node);
                }
            }
        }
        Ok(Grid { nodes })
    }
}
fn parse_char(c: char) -> Result<bool, ParseError> {
    match c {
        '.' => Ok(false),
        '@' => Ok(true),
        _ => Err(ParseError::InvalidCharacter(c)),
    }
}
pub fn count_node_neighbours(grid: &Grid, node: &PaperNode) -> usize {
    node.neighbours
        .iter()
        .filter(|n| grid.nodes.contains_key(n))
        .count()
}
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid character: {0}")]
    InvalidCharacter(char),
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

    #[test]
    fn test_from_str() {
        let input = ".@.\n.@.\n.@.";
        let grid = Grid::from_str(input).unwrap();
        assert_eq!(grid.nodes.len(), 3);
        assert!(!grid.nodes.contains_key(&IVec2::new(0, 0)));
        assert!(grid.nodes.contains_key(&IVec2::new(1, 0)));
        assert!(!grid.nodes.contains_key(&IVec2::new(2, 0)));
        assert!(!grid.nodes.contains_key(&IVec2::new(0, 1)));
        assert!(grid.nodes.contains_key(&IVec2::new(1, 1)));
        assert!(!grid.nodes.contains_key(&IVec2::new(2, 1)));
        assert!(!grid.nodes.contains_key(&IVec2::new(0, 2)));
        assert!(grid.nodes.contains_key(&IVec2::new(1, 2)));
        assert!(!grid.nodes.contains_key(&IVec2::new(2, 2)));
    }
}

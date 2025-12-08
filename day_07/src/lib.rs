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

#[derive(Debug, PartialEq, Eq)]
pub struct ParsedInput;
impl FromStr for ParsedInput {
    type Err = ParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Debug, Error)]
pub enum ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_input() -> ParsedInput {
        todo!()
    }

    #[test]
    fn test_parse_input() {
        todo!()
    }
}

use anyhow::{Context, Result};

mod part_1;
mod part_2;

pub fn run_part_1(input: &str) -> Result<u64> {
    part_1::run(input).context("Failed to run part 1")
}

pub fn run_part_2(input: &str) -> Result<u64> {
    part_2::run(input).context("Failed to run part 2")
}

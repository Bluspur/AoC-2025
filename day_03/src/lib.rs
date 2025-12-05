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

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Could not convert char {0} to joltage")]
    InvalidChar(char),
}

#[derive(Debug, PartialEq, Eq)]
pub struct AllBatteries(pub Vec<BatteryBank>);
impl FromStr for AllBatteries {
    type Err = ParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let all_banks = s
            .trim()
            .lines()
            .map(|line| BatteryBank::from_str(line))
            .collect::<Result<Vec<_>, ParseError>>()?;
        Ok(AllBatteries(all_banks))
    }
}
impl AllBatteries {
    pub fn iter(&self) -> impl Iterator<Item = &BatteryBank> {
        self.0.iter()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct BatteryBank(pub Vec<u32>);
impl FromStr for BatteryBank {
    type Err = ParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let all_joltages = s
            .chars()
            .map(|c| c.to_digit(10).ok_or(ParseError::InvalidChar(c)))
            .collect::<Result<Vec<_>, ParseError>>()?;
        Ok(BatteryBank(all_joltages))
    }
}
impl BatteryBank {
    pub fn joltages(&self) -> &[u32] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_batteries_from_str() {
        let all_batteries = AllBatteries::from_str("123\n456").unwrap();
        assert_eq!(
            all_batteries.0,
            vec![BatteryBank(vec![1, 2, 3]), BatteryBank(vec![4, 5, 6])]
        );
    }
    #[test]
    fn test_battery_bank_from_str() {
        let battery_bank = BatteryBank::from_str("123").unwrap();
        assert_eq!(battery_bank.0, vec![1, 2, 3]);
    }
}

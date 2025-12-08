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
pub struct Worksheet {
    pub problems: Vec<Problem>,
}
impl FromStr for Worksheet {
    type Err = ParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let mut lines = s.trim().lines().rev();

        let operators = lines
            .next()
            .ok_or(ParseError::MissingOperands)?
            .split_whitespace()
            .map(|raw_operator| Operator::from_str(raw_operator))
            .collect::<Result<Vec<_>, _>>()?;

        let mut p_operands = vec![vec![]; operators.len()];

        for line in lines {
            for (i, raw) in line.split_whitespace().enumerate() {
                let operand = raw.parse::<u64>()?;
                p_operands[i].push(operand);
            }
        }

        let mut problems = vec![];
        for i in 0..p_operands.len() {
            let problem = Problem {
                operands: p_operands[i].clone(),
                operator: operators[i],
            };
            problems.push(problem);
        }

        Ok(Worksheet { problems })
    }
}
#[derive(Debug, PartialEq, Eq)]
pub struct Problem {
    operands: Vec<u64>,
    operator: Operator,
}
impl Problem {
    pub fn solve(&self) -> u64 {
        match self.operator {
            Operator::Addition => self.operands.iter().sum(),
            Operator::Multiply => self.operands.iter().product(),
        }
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Operator {
    Addition,
    Multiply,
}
impl TryFrom<char> for Operator {
    type Error = ParseError;
    fn try_from(c: char) -> std::result::Result<Self, Self::Error> {
        match c {
            '+' => Ok(Operator::Addition),
            '*' => Ok(Operator::Multiply),
            _ => Err(ParseError::InvalidOperatorChar(c)),
        }
    }
}
impl FromStr for Operator {
    type Err = ParseError;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Addition),
            "*" => Ok(Operator::Multiply),
            _ => Err(ParseError::InvalidOperatorString(s.to_string())),
        }
    }
}
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseError {
    #[error("Invalid operator: {0}")]
    InvalidOperatorString(String),
    #[error("Invalid operator: {0}")]
    InvalidOperatorChar(char),
    #[error("Expected at least one line for operands")]
    MissingOperands,
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_input() -> Worksheet {
        todo!()
    }

    #[test]
    fn test_parse_valid_operator() {
        assert_eq!(Operator::from_str("+"), Ok(Operator::Addition));
        assert_eq!(Operator::from_str("*"), Ok(Operator::Multiply));
    }

    #[test]
    fn test_parse_invalid_operator() {
        assert_eq!(
            Operator::from_str("x"),
            Err(ParseError::InvalidOperatorString("x".to_string()))
        );
    }

    #[test]
    fn test_parse_worksheet() {
        let input = "1 2 3 4\n2 3 4 5\n+ + * *\n";
        let actual = Worksheet::from_str(input);
        let expected = Worksheet {
            problems: vec![
                Problem {
                    operands: vec![2, 1],
                    operator: Operator::Addition,
                },
                Problem {
                    operands: vec![3, 2],
                    operator: Operator::Addition,
                },
                Problem {
                    operands: vec![4, 3],
                    operator: Operator::Multiply,
                },
                Problem {
                    operands: vec![5, 4],
                    operator: Operator::Multiply,
                },
            ],
        };
        assert_eq!(actual, Ok(expected));
    }
}

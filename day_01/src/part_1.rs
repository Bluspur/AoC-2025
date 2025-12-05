use std::{num::ParseIntError, str::FromStr};
use thiserror::Error;

/// The initial position of the pointer.
const STARTING_POSITION: u32 = 50;
/// The upper bound of the pointer.
const UPPER_BOUND: u32 = 100;
/// The lower bound of the pointer.
// const LOWER_BOUND: u32 = 0;
/// The value to check against for determining the password.
const MAGIC_NUMBER: u32 = 0;

#[derive(Debug, Error)]
pub enum PartError {
    #[error(transparent)]
    ParseError(#[from] ParseError),
}
pub fn run(input: &str) -> Result<u64, PartError> {
    let mut dial = input.parse::<Dial>()?;
    let mut counter = 0;
    while !dial.is_complete() {
        dial.apply_instruction();
        if dial.position == MAGIC_NUMBER {
            counter += 1;
        }
    }

    Ok(counter)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Dial {
    /// The current position on the dial between the Lower Bound and Upper Bound.
    position: u32,
    /// The current index of the instruction being executed.
    index: usize,
    /// The instructions to execute.
    instructions: Vec<Instruction>,
}
impl Dial {
    fn is_complete(&self) -> bool {
        self.index >= self.instructions.len()
    }
    fn apply_instruction(&mut self) {
        let instruction = self.instructions[self.index];
        let new_position = match instruction.direction {
            Direction::Clockwise => (self.position + instruction.steps) % UPPER_BOUND,
            Direction::AntiClockwise => ((self.position + UPPER_BOUND)
                - (instruction.steps % UPPER_BOUND))
                .rem_euclid(UPPER_BOUND),
        };
        self.position = new_position;
        self.index += 1;
    }
}
impl FromStr for Dial {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Get a list of instructions from the input string.
        let instructions = s
            .trim()
            .lines()
            .map(|ln| ln.parse::<Instruction>())
            .collect::<Result<Vec<_>, _>>()?;
        // Create a `Dial` struct with default values.
        let dial = Dial {
            position: STARTING_POSITION,
            index: 0,
            instructions,
        };
        Ok(dial)
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Instruction {
    direction: Direction,
    steps: u32,
}
impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Split the string into its two components.
        let (dir, distance) = s.trim().split_at(1);
        let direction = dir.parse()?;
        let distance = distance.parse()?;
        Ok(Instruction {
            direction,
            steps: distance,
        })
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Direction {
    Clockwise,
    AntiClockwise,
}
impl FromStr for Direction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Direction::AntiClockwise),
            "R" => Ok(Direction::Clockwise),
            _ => Err(ParseError::InvalidDirection(s.to_string())),
        }
    }
}
#[derive(Debug, Error)]
pub enum ParseError {
    #[error("Invalid direction, expected 'L' or 'R', got {0}")]
    InvalidDirection(String),
    #[error("Invalid distance, expected a positive integer: {0}")]
    InvalidDistance(#[from] ParseIntError),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_happy_path() {
        let input = "L23\nR45 \nL9";
        let parsed = input.parse::<Dial>().unwrap();
        let expected_instructions = vec![
            Instruction {
                direction: Direction::AntiClockwise,
                steps: 23,
            },
            Instruction {
                direction: Direction::Clockwise,
                steps: 45,
            },
            Instruction {
                direction: Direction::AntiClockwise,
                steps: 9,
            },
        ];
        assert_eq!(parsed.instructions, expected_instructions);
    }

    #[test]
    fn test_apply_instruction() {
        let mut dial = Dial {
            position: STARTING_POSITION,
            index: 0,
            instructions: vec![
                Instruction {
                    direction: Direction::AntiClockwise,
                    steps: 68,
                },
                Instruction {
                    direction: Direction::AntiClockwise,
                    steps: 30,
                },
                Instruction {
                    direction: Direction::Clockwise,
                    steps: 48,
                },
            ],
        };

        dial.apply_instruction();
        assert_eq!(dial.position, 82);
        dial.apply_instruction();
        assert_eq!(dial.position, 52);
        dial.apply_instruction();
        assert_eq!(dial.position, 0);
    }
}

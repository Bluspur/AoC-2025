use std::{num::ParseIntError, str::FromStr};
use thiserror::Error;

/// The initial position of the pointer.
const STARTING_POSITION: u32 = 50;
/// The upper bound of the pointer.
const UPPER_BOUND: u32 = 100;

#[derive(Debug, Error)]
pub enum PartError {
    #[error(transparent)]
    ParseError(#[from] ParseError),
}
pub fn run(input: &str) -> Result<u64, PartError> {
    let mut dial = input.parse::<Dial>()?;
    let mut counter = 0;
    while !dial.is_complete() {
        let n = dial.apply_instruction();
        counter += n;
    }
    Ok(counter as u64)
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
    fn current_instruction(&self) -> &Instruction {
        &self.instructions[self.index]
    }
    fn is_complete(&self) -> bool {
        self.index >= self.instructions.len()
    }
    fn apply_instruction(&mut self) -> u32 {
        let instruction = self.instructions[self.index];
        let (new_position, number_of_times_passing_magic_number) = match instruction.direction {
            Direction::Clockwise => self.move_clockwise(instruction.steps),
            Direction::AntiClockwise => self.move_anticlockwise(instruction.steps),
        };

        self.position = new_position;
        self.index += 1;
        number_of_times_passing_magic_number
    }
    /// Moves the dial in a clockwise direction by the given number of steps.
    /// Returns the new position of the dial along with the number of times the dial passes the magic number.
    fn move_clockwise(&self, steps: u32) -> (u32, u32) {
        let number_of_times_passing_magic_number = (self.position + steps) / UPPER_BOUND;
        let new_position = (self.position + steps) % UPPER_BOUND;
        (new_position, number_of_times_passing_magic_number)
    }
    /// Moves the dial in an anticlockwise direction by the given number of steps.
    /// Returns the new position of the dial along with the number of times the dial passes the magic number.
    fn move_anticlockwise(&self, steps: u32) -> (u32, u32) {
        let new_position =
            ((self.position + UPPER_BOUND) - (steps % UPPER_BOUND)).rem_euclid(UPPER_BOUND);
        let mut wraps = (steps + (UPPER_BOUND - self.position) - 1) / UPPER_BOUND;
        if new_position == 0 {
            wraps += 1;
        }
        if self.position == 0 && steps > 0 {
            wraps -= 1;
        }
        (new_position, wraps)
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
impl Instruction {
    fn to_string(&self) -> String {
        format!("{}:{}", self.direction.to_string(), self.steps)
    }
    fn l(steps: u32) -> Self {
        Instruction {
            direction: Direction::AntiClockwise,
            steps,
        }
    }
    fn r(steps: u32) -> Self {
        Instruction {
            direction: Direction::Clockwise,
            steps,
        }
    }
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
impl Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Clockwise => "R".to_string(),
            Direction::AntiClockwise => "L".to_string(),
        }
    }
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

    fn test_dial() -> Dial {
        let instructions = vec![
            Instruction::l(68),
            Instruction::l(30),
            Instruction::r(48),
            Instruction::l(5),
            Instruction::r(60),
            Instruction::l(55),
            Instruction::l(1),
            Instruction::l(99),
            Instruction::r(14),
            Instruction::l(82),
        ];
        Dial {
            position: STARTING_POSITION,
            index: 0,
            instructions,
        }
    }

    fn test_dial_2() -> Dial {
        let instructions = vec![
            Instruction::r(50),
            Instruction::r(50),
            Instruction::l(50),
            Instruction::l(50),
            Instruction::r(75),
            Instruction::l(50),
            Instruction::l(25),
            Instruction::l(75),
            Instruction::r(50),
        ];
        Dial {
            position: STARTING_POSITION,
            index: 0,
            instructions,
        }
    }

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
        let mut dial = test_dial();

        let n = dial.apply_instruction();
        assert_eq!(dial.position, 82);
        assert_eq!(n, 1);
        let n = dial.apply_instruction();
        assert_eq!(dial.position, 52);
        assert_eq!(n, 0);
        let n = dial.apply_instruction();
        assert_eq!(dial.position, 0);
        assert_eq!(n, 1);
    }

    #[test]
    fn test_apply_instruction_2() {
        let mut dial = test_dial_2();
        let expected = [1, 1, 2, 2, 3, 4, 4, 5, 6];
        let mut n = 0;

        for i in 0..expected.len() {
            print!(
                "Pos: {}. Next: {}. Count: ",
                dial.position,
                dial.current_instruction().to_string()
            );
            n += dial.apply_instruction();
            println!("{}", n);
            assert_eq!(n, expected[i]);
        }
    }

    #[test]
    fn test_big_rotation() {
        let instructions = vec![Instruction::r(1000), Instruction::l(1000)];
        let mut dial = Dial {
            position: 0,
            index: 0,
            instructions,
        };

        let n = dial.apply_instruction();
        assert_eq!(dial.position, 0);
        assert_eq!(n, 10);

        let n = dial.apply_instruction();
        assert_eq!(dial.position, 0);
        assert_eq!(n, 10);
    }
}

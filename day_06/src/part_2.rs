use thiserror::Error;

use crate::*;

// Part 2 was a little more awkward than most other days.
// It seemed simple at first, we only need to "rotate" the numbers by staging the chars in a vec.
// However the implementation was a trickier than I expected and required things like padding the input
// to have a uniform length, and combining the numbers into a single number.

#[derive(Debug, Error, PartialEq, Eq)]
pub enum PartError {
    #[error(transparent)]
    ParsingError(#[from] ParseError),
}
/// Parses a worksheet from a string but using the "cephalopod" numbers system.
/// That means that numbers are read top to bottom (and right to left).
/// I've commented this one more than usual since it turned into a rather big function.
fn cephalopod_worksheet(s: &str) -> std::result::Result<Worksheet, PartError> {
    // Find the length of the longest line
    let max_len = s.lines().map(|ln| ln.len()).max().unwrap_or(0);
    // Pad each line with whitespace to make them all a uniform length.
    let padded = s.lines().map(|ln| {
        let padding = " ".repeat(max_len - ln.len());
        format!("{ln}{padding}")
    });
    // We convert the string into a 2d vec of chars.
    let lines = padded
        .map(|l| l.chars().rev().collect())
        .collect::<Vec<Vec<_>>>();
    // Initialize our output of problems.
    let mut problems = vec![];
    // Cache the index of the final line (operators).
    let last_line = lines.len() - 1;
    // The skip isn't strictly needed, but it saves a few calculations
    // when there is a divider between problems in the input.
    let mut skip = false;
    // The operands are stored outside the loop since we go column by column.
    let mut operands = vec![];
    // Loop through each column of the input.
    for i in 0..max_len {
        // Skip if we're in a divider.
        if skip {
            skip = false;
            continue;
        }
        // Initialize a vector of Option<u64> to store the values that make up
        // a single operand.
        let mut values = vec![Option::None; max_len - 1];
        // Loop row by row through the column.
        for j in 0..last_line {
            // If the character is a digit, push it to the values.
            if let Some(n) = lines[j][i].to_digit(10) {
                values[j] = Some(n as u64);
            }
        }
        // Combine the values together into a single number.
        let value = combine_numbers(&values);
        // Push the combined value to the operands vector.
        operands.push(value);
        // Handle the last character in the column uniquely, since it represents the operator.
        let operator_c = lines[last_line][i];
        // Check if our operator character is a valid operator.
        if let Ok(operator) = Operator::try_from(operator_c) {
            // Create a new problem with the operands and operator.
            let problem = Problem {
                operands: operands.clone(),
                operator,
            };
            // Push the problem to the problems vector.
            problems.push(problem);
            // Clear the operands vector for the next problem.
            operands.clear();
            // Set skip to true to skip the next character.
            // We know this since the input is always uniform.
            skip = true;
        }
    }

    Ok(Worksheet { problems })
}

fn combine_numbers(nums: &[Option<u64>]) -> u64 {
    let mut result = 0u64;

    for num in nums {
        if let Some(n) = num {
            // Count digits in n
            let digits = if *n == 0 { 1 } else { n.ilog10() + 1 };
            // Shift result left by that many digits and add n
            result = result * 10u64.pow(digits) + n;
        }
    }

    result
}

pub fn run(input: &str) -> Result<u64, PartError> {
    let worksheet = cephalopod_worksheet(input)?;
    dbg!(&worksheet);
    let score = worksheet
        .problems
        .iter()
        .map(|problem| problem.solve())
        .sum();
    Ok(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_alien_worksheet() {
        let input = "1 2 3 4\n2 3 4 5\n+ + * *\n";
        let actual = cephalopod_worksheet(input);
        let expected = Worksheet {
            problems: vec![
                Problem {
                    operands: vec![54],
                    operator: Operator::Multiply,
                },
                Problem {
                    operands: vec![43],
                    operator: Operator::Multiply,
                },
                Problem {
                    operands: vec![32],
                    operator: Operator::Addition,
                },
                Problem {
                    operands: vec![21],
                    operator: Operator::Addition,
                },
            ],
        };
        assert_eq!(actual, Ok(expected));
    }
}

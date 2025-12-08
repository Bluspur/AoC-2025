use thiserror::Error;

use crate::*;

#[derive(Debug, Error)]
pub enum PartError {
    #[error(transparent)]
    ParsingError(#[from] ParseError),
}

pub fn run(input: &str) -> Result<u64, PartError> {
    let worksheet = Worksheet::from_str(input)?;
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
    #[ignore]
    fn test_run_part() {
        todo!()
    }
}

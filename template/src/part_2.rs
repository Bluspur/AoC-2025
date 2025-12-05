use thiserror::Error;

use crate::*;

#[derive(Debug, Error)]
pub enum PartError {
    #[error(transparent)]
    ParsingError(#[from] ParseError),
}

pub fn run(input: &str) -> Result<u64, PartError> {
    todo!("Implement solution");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_part() {
        todo!()
    }
}

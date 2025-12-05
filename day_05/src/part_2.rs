use thiserror::Error;

#[derive(Debug, Error)]
pub enum PartError {
    #[error("Invalid input")]
    InvalidInput,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedInput {
    // Define fields here
}
pub fn run(input: &str) -> Result<u64, PartError> {
    todo!("Implement solution");
}
pub fn parse(input: &str) -> Result<ParsedInput, PartError> {
    todo!("Implement input parsing");
}
mod tests {
    use super::*;

    #[test]
    fn test_parse_happy_path() {
        let input = "123";
        let parsed = parse(input).unwrap();
        assert_eq!(parsed, ParsedInput {});
    }
}

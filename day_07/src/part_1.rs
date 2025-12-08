use thiserror::Error;

use crate::*;

#[derive(Debug, Error)]
pub enum PartError {
    #[error(transparent)]
    ParsingError(#[from] ParseError),
}

pub fn run(input: &str) -> Result<u64, PartError> {
    let manifold = TachyonManifold::from_str(input)?;
    let count = manifold.count_reflections();
    Ok(count)
}

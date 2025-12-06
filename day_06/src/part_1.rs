use thiserror::Error;

use crate::*;

#[derive(Debug, Error)]
pub enum PartError {
    #[error(transparent)]
    ParsingError(#[from] ParseError),
}

pub fn run(input: &str) -> Result<u64, PartError> {
    let db = Database::from_str(input)?;
    let count = count_fresh_ingredient_ids_in_db(&db);
    Ok(count as u64)
}

pub fn count_fresh_ingredient_ids_in_db(db: &Database) -> usize {
    db.available_ids
        .iter()
        .filter(|id| db.is_fresh(**id))
        .count()
}

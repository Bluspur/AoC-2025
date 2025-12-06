use log::info;
use thiserror::Error;

use crate::*;

#[derive(Debug, Error)]
pub enum PartError {
    #[error(transparent)]
    ParsingError(#[from] ParseError),
}

pub fn run(input: &str) -> Result<u64, PartError> {
    let db = Database::from_str(input)?;
    let merged = MergedIngredientRanges::from_unsorted(db.fresh_id_ranges);
    let count = merged.all_ranges_len();
    Ok(count as u64)
}
#[derive(Debug)]
pub struct MergedIngredientRanges {
    merged: Vec<IdRange>,
}
impl MergedIngredientRanges {
    fn from_unsorted(mut ranges: Vec<IdRange>) -> Self {
        ranges.sort_by_key(|r| r.min);
        info!("Sorted: {ranges:?}");
        let mut merged = Vec::new();
        let mut current = ranges[0];

        for range in ranges.iter().skip(1) {
            if ranges_overlap(current, *range) {
                current = merge_ranges(current, *range);
            } else {
                merged.push(current);
                current = *range;
            }
        }

        merged.push(current);

        MergedIngredientRanges { merged }
    }
    fn all_ranges_len(&self) -> usize {
        self.merged.iter().map(|r| r.len()).sum()
    }
}

fn ranges_overlap(a: IdRange, b: IdRange) -> bool {
    a.min <= b.max && b.min <= a.max
}
/// Merge two ranges and returns a combined range.
/// Assumes the two ranges overlap.
fn merge_ranges(a: IdRange, b: IdRange) -> IdRange {
    IdRange::new(a.min.min(b.min), a.max.max(b.max))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ranges_overlap_return_expected_position_independent() {
        let pair_expected = [
            (IdRange::new(3, 5), IdRange::new(10, 14), false), // Non overlapping
            (IdRange::new(3, 5), IdRange::new(4, 6), true),    // Partial overlap
            (IdRange::new(1, 9), IdRange::new(2, 4), true),    // Full overlap
        ];

        for (a, b, expected) in pair_expected {
            let actual = ranges_overlap(a, b);
            assert_eq!(expected, actual);
            let reversed = ranges_overlap(b, a);
            assert_eq!(expected, reversed);
        }
    }

    #[test]
    fn test_merge_ranges_returns_correct_range() {
        let pair_expected = [
            (IdRange::new(3, 5), IdRange::new(4, 6), IdRange::new(3, 6)), // Partial Overlap
            (IdRange::new(1, 9), IdRange::new(2, 4), IdRange::new(1, 9)), // Full Overlap
        ];

        for (a, b, expected) in pair_expected {
            let actual = merge_ranges(a, b);
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_count_range_len_returns_expected() {
        let range_expected_len = [(IdRange::new(3, 5), 3), (IdRange::new(0, 10), 11)];

        for (range, expected) in range_expected_len {
            let actual = range.len();
            assert_eq!(expected, actual);
        }
    }

    #[test]
    fn test_solve_part_2_returns_expected() {
        let unsorted = [
            IdRange::new(3, 5),
            IdRange::new(10, 14),
            IdRange::new(16, 20),
            IdRange::new(12, 18),
        ];

        let merged = MergedIngredientRanges::from_unsorted(unsorted.into());
        dbg!(&merged);
        let actual = merged.all_ranges_len();
        let expected = 14;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_solve_part_2_unsorted_edge_cases() {
        let unsorted = [
            IdRange::new(0, 10),
            IdRange::new(11, 12),
            IdRange::new(9, 14),
            IdRange::new(1, 15),
        ];

        let merged = MergedIngredientRanges::from_unsorted(unsorted.into());
        dbg!(&merged);
        let actual = merged.all_ranges_len();
        let expected = 16;

        assert_eq!(expected, actual);
    }
}

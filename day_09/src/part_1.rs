use itertools::Itertools;
use thiserror::Error;

use crate::*;

#[derive(Debug, Error)]
pub enum PartError {
    #[error(transparent)]
    ParsingError(#[from] ParseError),
}

pub fn run(input: &str) -> Result<u64, PartError> {
    let tiles = Tiles::from_str(input)?;
    Ok(find_largest_tile_area(&tiles))
}

fn find_largest_tile_area(tiles: &Tiles) -> u64 {
    tiles
        .tiles
        .iter()
        .combinations(2)
        .map(|pair| calculate_area(*pair[0], *pair[1]))
        .max()
        .unwrap_or(0)
}

fn calculate_area(a: IVec2, b: IVec2) -> u64 {
    let width = (b.x - a.x).abs() as u64 + 1; // Plus one to include both endpoints
    let height = (b.y - a.y).abs() as u64 + 1; // Plus one to include both endpoints
    width * height
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_input() -> Tiles {
        Tiles {
            tiles: vec![
                IVec2 { x: 7, y: 1 },
                IVec2 { x: 11, y: 1 },
                IVec2 { x: 11, y: 7 },
                IVec2 { x: 9, y: 7 },
                IVec2 { x: 9, y: 5 },
                IVec2 { x: 2, y: 5 },
                IVec2 { x: 2, y: 3 },
                IVec2 { x: 7, y: 3 },
            ],
        }
    }

    #[test]
    fn test_run_part() {
        let input = setup_test_input();
        let expected = 50;
        let actual = find_largest_tile_area(&input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_calculate_area() {
        let pair_tiles_and_area = [
            (IVec2::new(2, 5), IVec2::new(9, 7), 24),
            (IVec2::new(7, 1), IVec2::new(11, 7), 35),
            (IVec2::new(7, 3), IVec2::new(2, 3), 6),
            (IVec2::new(2, 5), IVec2::new(11, 1), 50),
        ];

        for (a, b, expected) in pair_tiles_and_area {
            assert_eq!(calculate_area(a, b), expected);
        }
    }
}

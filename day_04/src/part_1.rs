use log::info;
use thiserror::Error;

use crate::*;

#[derive(Debug, Error)]
pub enum PartError {
    #[error(transparent)]
    ParsingError(#[from] ParseError),
}
pub fn run(input: &str) -> Result<u64, PartError> {
    let grid = Grid::from_str(input)?;
    let accessable_node_count = count_all_accessible_nodes_in_grid(&grid);
    Ok(accessable_node_count as u64)
}
pub fn count_all_accessible_nodes_in_grid(grid: &Grid) -> usize {
    let mut count = 0;
    for (pos, node) in grid.nodes.iter() {
        let paper_roll_neighbour_count = count_node_neighbours(grid, node);
        if paper_roll_neighbour_count < 4 {
            info!("Node at {pos:?} has {paper_roll_neighbour_count} neighbours");
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    // it'd almost certainly be better to set the grid up directly rather than parsing a string, but it'd be a lot of work to do so.
    fn setup_test() -> Grid {
        Grid::from_str("\n..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.\n").unwrap()
    }

    #[test]
    fn test_count_all_node_neighbours_in_grid() {
        let grid = setup_test();
        assert_eq!(grid.nodes.len(), 71);
        assert_eq!(count_all_accessible_nodes_in_grid(&grid), 13);
    }

    #[test]
    fn test_node_has_less_than_four_neighbours() {
        let grid = setup_test();
        let node = grid.nodes.get(&IVec2::new(2, 0)).unwrap();
        assert_eq!(count_node_neighbours(&grid, node), 3);
    }

    #[test]
    fn test_node_has_more_than_three_neighbours() {
        let grid = setup_test();
        let node = grid.nodes.get(&IVec2::new(4, 1)).unwrap();
        assert_eq!(count_node_neighbours(&grid, node), 4);
    }
}

use log::info;
use thiserror::Error;

use crate::*;

#[derive(Debug, Error)]
pub enum PartError {
    #[error(transparent)]
    ParsingError(#[from] ParseError),
}
pub fn run(input: &str) -> Result<u64, PartError> {
    let mut grid = Grid::from_str(input)?;
    let count = count_all_nodes_that_can_be_removed_in_grid(&mut grid);

    Ok(count)
}

pub fn count_all_nodes_that_can_be_removed_in_grid(grid: &mut Grid) -> u64 {
    let mut count = 0;
    loop {
        let accessible_nodes = get_all_accessible_nodes(grid);
        if accessible_nodes.is_empty() {
            break;
        }
        count += accessible_nodes.len() as u64;
        remove_all_nodes(grid, &accessible_nodes);
    }

    count
}

pub fn get_all_accessible_nodes(grid: &Grid) -> Vec<IVec2> {
    let mut accessible = Vec::new();
    for (pos, node) in grid.nodes.iter() {
        let paper_roll_neighbour_count = count_node_neighbours(grid, node);
        if paper_roll_neighbour_count < 4 {
            info!("Node at {pos:?} has {paper_roll_neighbour_count} neighbours");
            accessible.push(*pos);
        }
    }
    accessible
}

pub fn remove_all_nodes(grid: &mut Grid, to_remove: &[IVec2]) {
    for pos in to_remove {
        grid.nodes.remove(pos);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // it'd almost certainly be better to set the grid up directly rather than parsing a string, but it'd be a lot of work to do so.
    fn setup_test() -> Grid {
        Grid::from_str("\n..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.\n").unwrap()
    }

    #[test]
    fn test_count_all_nodes_that_can_be_removed_in_grid() {
        let mut grid = setup_test();
        let count = count_all_nodes_that_can_be_removed_in_grid(&mut grid);
        assert_eq!(count, 43);
    }
}

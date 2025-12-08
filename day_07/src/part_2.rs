use std::collections::HashMap;

use thiserror::Error;

use crate::*;

#[derive(Debug, Error)]
pub enum PartError {
    #[error(transparent)]
    ParsingError(#[from] ParseError),
    #[error("Expected leaf to have at least one origin")]
    LeafNodeWithNoOrigin,
}

pub fn run(input: &str) -> Result<u64, PartError> {
    let manifold = TachyonManifold::from_str(input)?;
    let map = Map::from(manifold);
    let count = map.count_all_paths_to_all_leaves();
    count
}
#[derive(Debug, PartialEq, Eq)]
struct Map {
    nodes: HashMap<IVec2, Node>,
    leaves: Vec<Node>,
}
impl Map {
    pub fn count_all_paths_to_all_leaves(&self) -> Result<u64, PartError> {
        let mut memo = HashMap::<IVec2, u64>::new();
        // Recursive depth first search.
        fn dfs(pos: IVec2, nodes: &HashMap<IVec2, Node>, memo: &mut HashMap<IVec2, u64>) -> u64 {
            // Cached?
            if let Some(&v) = memo.get(&pos) {
                return v;
            }
            let node = &nodes[&pos];
            // Start node
            let result = match &node.origin {
                None => 1,
                Some(origins) => origins.iter().copied().map(|p| dfs(p, nodes, memo)).sum(),
            };

            memo.insert(pos, result);
            result
        }
        // Perform the search and count the number of unique paths.
        self.leaves.iter().try_fold(0, |acc, leaf| {
            let origins = leaf
                .origin
                .as_ref()
                .ok_or(PartError::LeafNodeWithNoOrigin)?;
            let subtotal = origins
                .iter()
                .copied()
                .map(|p| dfs(p, &self.nodes, &mut memo))
                .sum::<u64>();
            Ok(acc + subtotal)
        })
    }
}
impl From<TachyonManifold> for Map {
    fn from(value: TachyonManifold) -> Self {
        let mut nodes = HashMap::new();
        nodes.insert(value.start, Node::start());
        let mut cols = HashMap::from([(value.start.x, vec![value.start])]);
        for mirror in value.mirrors.iter() {
            let col = mirror.x;
            if let Some(origin) = cols.remove(&col) {
                nodes.insert(*mirror, Node::new(origin));
                let (l, r) = split(col);
                cols.entry(l).or_default().push(*mirror);
                cols.entry(r).or_default().push(*mirror);
            }
        }
        let leaves = cols.values().cloned().map(|v| Node::new(v)).collect();
        Map { nodes, leaves }
    }
}
#[derive(Debug, PartialEq, Eq)]
struct Node {
    origin: Option<Vec<IVec2>>,
}
impl Node {
    fn start() -> Node {
        Node { origin: None }
    }
    fn new(origins: Vec<IVec2>) -> Node {
        Node {
            origin: Some(origins),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to make writing points really short.
    fn p(x: i32, y: i32) -> IVec2 {
        IVec2::new(x, y)
    }
    fn setup_test_input() -> TachyonManifold {
        TachyonManifold {
            start: p(7, 0),
            rows: 16,
            mirrors: vec![
                p(7, 2),
                p(6, 4),
                p(8, 4),
                p(5, 6),
                p(7, 6),
                p(9, 6),
                p(4, 8),
                p(6, 8),
                p(10, 8),
                p(3, 10),
                p(5, 10),
                p(9, 10),
                p(11, 10),
                p(2, 12),
                p(6, 12),
                p(12, 12),
                p(1, 14),
                p(3, 14),
                p(5, 14),
                p(7, 14),
                p(9, 14),
                p(13, 14),
            ],
        }
    }

    #[test]
    fn test_run_part() {
        let manifold = setup_test_input();
        let map = Map::from(manifold);
        let actual = map.count_all_paths_to_all_leaves().unwrap();
        let expected = 40;
        assert_eq!(expected, actual);
    }
}

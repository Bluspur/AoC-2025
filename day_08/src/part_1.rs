use std::collections::{HashMap, HashSet};

use thiserror::Error;

use crate::*;

#[derive(Debug, Error)]
pub enum PartError {
    #[error(transparent)]
    ParsingError(#[from] ParseError),
}

pub fn run(input: &str, n_pairs: usize) -> Result<u64, PartError> {
    let decorations = Decorations::from_str(input)?;
    let network = network_from_decorations_with_x_distances(&decorations, n_pairs);
    let circuit_lengths = network.measure_circuit_lengths();
    let product_of_first_three: usize = circuit_lengths.iter().take(3).product();
    Ok(product_of_first_three as u64)
}
fn network_from_decorations_with_x_distances(decorations: &Decorations, n_pairs: usize) -> Network {
    let mut network = Network::default();
    let pairs = order_all_junction_box_pairs_by_distance(decorations);
    for (a, b) in pairs.into_iter().take(n_pairs) {
        network.add_connection(a, b);
    }
    network
}
#[derive(Debug, Default)]
struct Network {
    nodes: HashMap<JunctionBox, Node>,
}
impl Network {
    /// Adds a connection between two junction boxes.
    fn add_connection(&mut self, a: JunctionBox, b: JunctionBox) {
        self.nodes.entry(a).or_default().connections.insert(b);
        self.nodes.entry(b).or_default().connections.insert(a);
    }
    /// Measures the lengths of all circuits in the network.
    /// A circuit is a connected network of junction boxes.
    /// Results come in descending order.
    fn measure_circuit_lengths(&self) -> Vec<usize> {
        let mut lengths = Vec::new();
        let mut visited = HashSet::new();
        for (junction, _) in &self.nodes {
            if visited.insert(*junction) {
                let connected_junctions = self.get_connected_junctions(junction);
                let circuit_length = connected_junctions.len();
                lengths.push(circuit_length);
                visited.extend(connected_junctions.iter());
            }
        }
        lengths.sort();
        lengths.reverse();
        lengths
    }
    fn get_connected_junctions(&self, start: &JunctionBox) -> Vec<JunctionBox> {
        let mut visited = HashSet::new();
        let mut stack = vec![*start];
        while let Some(node) = stack.pop() {
            if visited.insert(node) {
                for &neighbor in self.nodes.get(&node).unwrap().connections.iter() {
                    stack.push(neighbor);
                }
            }
        }
        visited.into_iter().collect()
    }
}
impl From<Decorations> for Network {
    fn from(decorations: Decorations) -> Self {
        let all_pairs = order_all_junction_box_pairs_by_distance(&decorations);
        let mut network = Network::default();
        for (a, b) in all_pairs {
            network.nodes.entry(a).or_default().connections.insert(b);
            network.nodes.entry(b).or_default().connections.insert(a);
        }
        network
    }
}
#[derive(Debug, Default)]
struct Node {
    connections: HashSet<JunctionBox>,
}
/// Calculates all pairwise distances between junction boxes.
/// Returns a vector of tuples containing each pair of junction boxes ordered by distance.
/// Lowest distance first.
fn order_all_junction_box_pairs_by_distance(
    decorations: &Decorations,
) -> Vec<(JunctionBox, JunctionBox)> {
    let mut distances = Vec::new();
    for i in 0..decorations.junction_boxes.len() {
        for j in i + 1..decorations.junction_boxes.len() {
            distances.push((decorations.junction_boxes[i], decorations.junction_boxes[j]));
        }
    }
    distances.sort_by_key(|&(a, b)| a.sq_distance(&b));
    distances
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_input() -> Decorations {
        Decorations::new(&[
            jb(162, 817, 812),
            jb(57, 618, 57),
            jb(906, 360, 560),
            jb(592, 479, 940),
            jb(352, 342, 300),
            jb(466, 668, 158),
            jb(542, 29, 236),
            jb(431, 825, 988),
            jb(739, 650, 466),
            jb(52, 470, 668),
            jb(216, 146, 977),
            jb(819, 987, 18),
            jb(117, 168, 530),
            jb(805, 96, 715),
            jb(346, 949, 466),
            jb(970, 615, 88),
            jb(941, 993, 340),
            jb(862, 61, 35),
            jb(984, 92, 344),
            jb(425, 690, 689),
        ])
    }
    /// Helper for building a junction box quickly.
    fn jb(x: i32, y: i32, z: i32) -> JunctionBox {
        JunctionBox { x, y, z }
    }
    #[test]
    fn test_order_all_junction_box_pairs_by_distance() {
        let decorations = setup_test_input();
        let first_four_expected_pairs = vec![
            (jb(162, 817, 812), jb(425, 690, 689)),
            (jb(162, 817, 812), jb(431, 825, 988)),
            (jb(906, 360, 560), jb(805, 96, 715)),
            (jb(431, 825, 988), jb(425, 690, 689)),
        ];
        let actual_pairs = order_all_junction_box_pairs_by_distance(&decorations);
        assert_eq!(first_four_expected_pairs, actual_pairs[..4]);
    }
}

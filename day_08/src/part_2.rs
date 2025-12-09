use std::collections::{HashMap, HashSet};

use thiserror::Error;

use crate::*;

#[derive(Debug, Error)]
pub enum PartError {
    #[error(transparent)]
    ParsingError(#[from] ParseError),
}

pub fn run(input: &str, _: usize) -> Result<u64, PartError> {
    let decorations = Decorations::from_str(input)?;
    let last_pair = last_pair_needed_to_connect_all_junction_boxes(&decorations);
    let product = last_pair.unwrap().0.x as u64 * last_pair.unwrap().1.x as u64;
    Ok(product as u64)
}
/// Ignoring the "depth limit" that we had on part 1, my solution is just a brute force version of part 1.
/// It takes the Decorations and performs the same counting circuits method for the network, but just does it after every connection is added.
/// This results in a slow solution, but is just a few lines changed from part 1.
fn last_pair_needed_to_connect_all_junction_boxes(
    decorations: &Decorations,
) -> Option<(JunctionBox, JunctionBox)> {
    let mut network = Network::default();
    let pairs = order_all_junction_box_pairs_by_distance(decorations);
    for (a, b) in pairs {
        network.add_connection(a, b);
        if network.measure_circuit_lengths()[0] == decorations.junction_boxes.len() {
            return Some((a, b));
        }
    }
    None
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

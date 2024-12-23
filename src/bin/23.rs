advent_of_code::solution!(23);

use itertools::Itertools;
use petgraph::prelude::*;

fn parse_input_graph_map(input: &str) -> UnGraphMap<&str, ()> {
    UnGraphMap::from_edges(input.lines().map(|line| {
        let mut parts = line.split("-");
        let a_name = parts.next().unwrap();
        let b_name = parts.next().unwrap();
        (a_name, b_name)
    }))
}

pub fn part_one(input: &str) -> Option<u32> {
    //let input_graph = Frozen::new(&mut parse_input_ungraph(input));
    //let triple = Frozen::new(&mut get_triple_ungraph());
    let input_graph = &parse_input_graph_map(input);

    let count = input_graph
        .nodes()
        .flat_map(|node| {
            input_graph
                .neighbors(node) // for each node we connect to
                .tuple_combinations() // make pairs of all of them
                .filter(move |(a, b)| {
                    // and check if any are connected
                    input_graph.contains_edge(*a, *b)
                        && [node, *a, *b].iter().any(|x| x.starts_with("t"))
                })
                .map(move |(a, b)| {
                    // sort the triples cannonically
                    let mut nodes = [node, a, b];
                    nodes.sort();
                    nodes
                })
        })
        .unique() // get rid of duplicates
        .count();
    Some(count as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}

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

pub fn part_two(input: &str) -> Option<String> {
    let input_graph = &parse_input_graph_map(input);

    let mut clusters_found;
    let mut biggest_cluster: Option<Vec<&str>> = None;

    for cluster_size in [3, 12].iter() {
        let cluster = input_graph
            .nodes()
            .flat_map(|node| {
                input_graph
                    .neighbors(node) // for each node we connect to
                    .combinations(*cluster_size as usize)
                    .filter_map(
                        // make pairs of all of them
                        move |neighbor_subset| {
                            if neighbor_subset
                                .iter()
                                .tuple_combinations()
                                .all(move |(a, b)| input_graph.contains_edge(a, b))
                            {
                                // sort the triples cannonically
                                let mut nodes = vec![node]
                                    .into_iter()
                                    .chain(neighbor_subset.into_iter())
                                    .collect::<Vec<_>>();
                                nodes.sort();
                                Some(nodes)
                            } else {
                                None
                            }
                        },
                    )
            })
            .unique() // get rid of duplicates
            .collect::<Vec<_>>();
        clusters_found = cluster.len();
        if clusters_found == 1 {
            biggest_cluster = Some(cluster[0].clone());
        }
    }
    if let Some(biggest_cluster) = biggest_cluster {
        Some(biggest_cluster.join(",").to_string())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(["co", "de", "ka", "ta"].join(",")));
    }
}

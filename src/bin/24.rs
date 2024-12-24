use std::collections::HashMap;

use itertools::Itertools;
use petgraph::graphmap::DiGraphMap;

advent_of_code::solution!(24);

enum Op {
    And(String, String, String),
    Or(String, String, String),
    Xor(String, String, String),
}

fn parse_input(input: &str) -> (HashMap<String, bool>, Vec<Op>) {
    let mut wires = HashMap::new();
    let mut ops = Vec::new();

    let mut input_parts = input.split("\n\n");
    let wire_input = input_parts.next().unwrap();
    let op_input = input_parts.next().unwrap();

    for wire in wire_input.lines() {
        let mut parts = wire.split(": ");
        let name = parts.next().unwrap().to_string();
        let value = parts
            .last()
            .unwrap()
            .parse::<u32>()
            .expect("invalid wire input")
            == 1;
        wires.insert(name, value);
    }

    for op in op_input.lines() {
        let mut parts = op.split(" ");
        let x = parts.next().unwrap();
        let op = parts.next().unwrap();
        let y = parts.next().unwrap();
        let _ = parts.next();
        let z = parts.next().unwrap();

        match op {
            "AND" => ops.push(Op::And(x.to_string(), y.to_string(), z.to_string())),
            "OR" => ops.push(Op::Or(x.to_string(), y.to_string(), z.to_string())),
            "XOR" => ops.push(Op::Xor(x.to_string(), y.to_string(), z.to_string())),
            _ => panic!("Unknown op: {}", op),
        }
    }

    (wires, ops)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (mut wires, ops) = parse_input(input);

    loop {
        let mut doing_work = false;

        for op in ops.iter() {
            match op {
                Op::And(x, y, z) => {
                    if wires.contains_key(z) {
                        continue;
                    }
                    if let (Some(x), Some(y)) = (wires.get(x), wires.get(y)) {
                        wires.insert(z.to_string(), *x & *y);
                        doing_work = true;
                    }
                }
                Op::Or(x, y, z) => {
                    if wires.contains_key(z) {
                        continue;
                    }
                    if let (Some(x), Some(y)) = (wires.get(x), wires.get(y)) {
                        wires.insert(z.to_string(), *x | *y);
                        doing_work = true;
                    }
                }
                Op::Xor(x, y, z) => {
                    if wires.contains_key(z) {
                        continue;
                    }
                    if let (Some(x), Some(y)) = (wires.get(x), wires.get(y)) {
                        wires.insert(z.to_string(), *x ^ *y);
                        doing_work = true;
                    }
                }
            }
        }
        if !doing_work {
            break;
        }
    }

    let keys = wires.keys().sorted().rev().collect_vec();
    /*for wire in keys {
        println!("{}: {}", wire, wires.get(wire).unwrap());
    }*/

    let result: usize = keys
        .iter()
        //.inspect(|wire| println!("{}: {}", wire, wires.get(**wire).unwrap()))
        .filter(|wire| wire.starts_with("z"))
        .fold(0, |result, wire| {
            (result << 1) | *wires.get(*wire).unwrap() as usize
        });

    Some(result)
}

pub fn part_two(input: &str) -> Option<String> {
    part_two_ex(input, 4)
}

pub fn part_two_ex(input: &str, swaps: u32) -> Option<String> {
    let (mut wires, ops) = parse_input(input);
    let mut di_graph: DiGraphMap<&str, &str> = DiGraphMap::new();
    for op in ops.iter() {
        match op {
            Op::And(x, y, z) => {
                di_graph.add_edge(x, z, "&");
                di_graph.add_edge(y, z, "&");
            }
            Op::Or(x, y, z) => {
                di_graph.add_edge(x, z, "|");
                di_graph.add_edge(y, z, "|");
            }
            Op::Xor(x, y, z) => {
                di_graph.add_edge(x, z, "^");
                di_graph.add_edge(y, z, "^");
            }
        }
    }
    // print di_graph in dot format
    //println!("{:?}", di_graph);
    println!("{:?}", petgraph::dot::Dot::new(&di_graph));

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_small() {
        let result = part_one(
            "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02",
        );
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2024));
    }

    #[test]
    #[ignore]
    fn test_part_two() {
        let result = part_two_ex(
            "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00",
            2,
        );
        assert_eq!(result, Some("z00,z01,z02,z05".to_string()));
    }
}

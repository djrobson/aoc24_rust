advent_of_code::solution!(8);

use std::collections::{HashMap, HashSet};

fn parse_input(input: &str) -> (u8, HashMap<u8, Vec<(u8, u8)>>) {
    let mut antenna_locations = HashMap::new();
    let mut x;
    let mut y = 0;
    for line in input.lines() {
        x = 0;
        for byte in line.bytes() {
            if byte != b'.' {
                //println!("Antenna at {} ({},{})", byte as char, x, y);
                if antenna_locations.contains_key(&byte) {
                    let vec: &mut Vec<(u8, u8)> = antenna_locations.get_mut(&byte).unwrap();
                    vec.push((x, y));
                } else {
                    let mut vec = Vec::new();
                    vec.push((x, y));
                    antenna_locations.insert(byte, vec);
                }
            }
            x = x + 1;
        }
        y = y + 1;
    }
    (y - 1, antenna_locations)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (input_size, antenna_locations) = parse_input(input);
    //let mut node_count = 0;
    let mut nodes = HashSet::new();

    /* for key in antenna_locations.keys() {
        println!("Antenna type: {}", *key as char);
        for antenna in antenna_locations.get(key).unwrap() {
            println!("\t({},{})", antenna.0, antenna.1);
        }
    } */

    //let mut max_nodes = 0;
    for antenna_type in antenna_locations.keys() {
        let antennas_of_cur_type = antenna_locations.get(antenna_type).unwrap();
        //max_nodes += antennas_of_cur_type.len() * (antennas_of_cur_type.len() - 1);
        for antenna in 0..antennas_of_cur_type.len() {
            let (x1, y1) = antennas_of_cur_type[antenna];

            for other_antenna in 0..antennas_of_cur_type.len() {
                if antenna == other_antenna {
                    continue;
                }
                let (x2, y2) = antennas_of_cur_type[other_antenna];
                let x_diff = x1 as i32 - x2 as i32;
                let y_diff = y1 as i32 - y2 as i32;
                let node_x = x1 as i32 + x_diff;
                let node_y = y1 as i32 + y_diff;
                if node_x < 0
                    || node_x > input_size as i32
                    || node_y < 0
                    || node_y > input_size as i32
                {
                    /*println!(
                        "Node: {} ({},{})-({},{})=({}, {}) skipped",
                        *antenna_type as char, x1, y1, x2, y2, node_x, node_y
                    );*/
                    continue;
                }
                /*println!(
                    "Node: {} ({},{})-({},{})=({}, {})",
                    *antenna_type as char, x1, y1, x2, y2, node_x, node_y
                );*/
                nodes.insert((node_x as u8, node_y as u8));
            }
        }
    }
    //println!("Max nodes: {}, on grid {}", max_nodes, node_count);
    Some(nodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_one_1() {
        let result = part_one(
            "..........
..........
..........
....a.....
..........
.....a....
..........
..........
..........
..........",
        );
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(
            "..........
..........
..........
....a.....
........a.
.....a....
..........
..........
..........
..........",
        );
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_3() {
        let result = part_one(
            "..........
..........
..........
....a.....
........a.
.....a....
..........
.......A..
..........
..........",
        );
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_4() {
        let result = part_one(
            "............
........0...
.....0......
.......0....
....0.......
............
............
............
............
............
............
............",
        );
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_one_5() {
        let result = part_one(
            "............
............
............
............
............
......A.....
............
............
........A...
.........A..
............
............",
        );
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

advent_of_code::solution!(12);
use std::collections::{HashMap, HashSet};

struct Region {
    label: u8,
    squares: HashSet<(i32, i32)>,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
enum SideType {
    Above,
    Below,
    Left,
    Right,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct SideDirection {
    dx: i32,
    dy: i32,
    side_type: SideType,
}

fn find_all_adjacent_squares(
    x: i32,
    y: i32,
    grid: &Vec<Vec<((usize, usize), u8)>>,
    visited: &mut HashSet<(i32, i32)>,
    label: u8,
) -> HashSet<(i32, i32)> {
    let mut squares = HashSet::new();
    let mut to_visit = vec![(x, y)];

    while let Some((x, y)) = to_visit.pop() {
        if visited.contains(&(x, y)) {
            continue;
        }

        visited.insert((x, y));
        squares.insert((x, y));

        for (dx, dy) in &[(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let new_x = x + dx;
            let new_y = y + dy;

            if new_x < 0 || new_y < 0 {
                continue;
            }

            if let Some(row) = grid.get(new_y as usize) {
                if let Some((_, new_label)) = row.get(new_x as usize) {
                    if *new_label == label {
                        to_visit.push((new_x, new_y));
                    }
                }
            }
        }
    }

    squares
}

fn build_grid(input: &str) -> Vec<Vec<((usize, usize), u8)>> {
    input
        .lines()
        .enumerate()
        .map(|(row_num, line)| {
            line.bytes()
                .enumerate()
                .map(|(col_num, val)| ((col_num, row_num), val))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn build_regions_by_label(grid: &Vec<Vec<((usize, usize), u8)>>) -> HashMap<u8, Vec<Region>> {
    let mut regions_by_label: HashMap<u8, Vec<Region>> = HashMap::new();

    for row in grid.iter() {
        for ((x, y), label) in row.iter() {
            let label = *label;
            let mut already_found = false;
            if regions_by_label.contains_key(&label) {
                // check for previous region to merge with
                for region in regions_by_label.get_mut(&label).unwrap() {
                    if region.squares.contains(&(*x as i32, *y as i32)) {
                        already_found = true;
                        break;
                    }
                }
            }

            if already_found {
                continue;
            } else {
                let adjacent_squares = find_all_adjacent_squares(
                    *x as i32,
                    *y as i32,
                    &grid,
                    &mut HashSet::new(),
                    label,
                );
                regions_by_label
                    .entry(label)
                    .or_insert_with(Vec::new)
                    .push(Region {
                        label,
                        squares: adjacent_squares,
                    });
            }
        }
    }

    regions_by_label
}

fn get_side_directions() -> Vec<SideDirection> {
    vec![
        SideDirection {
            dx: 0,
            dy: -1,
            side_type: SideType::Above,
        },
        SideDirection {
            dx: 1,
            dy: 0,
            side_type: SideType::Right,
        },
        SideDirection {
            dx: -1,
            dy: 0,
            side_type: SideType::Left,
        },
        SideDirection {
            dx: 0,
            dy: 1,
            side_type: SideType::Below,
        },
    ]
}

fn get_edge_neighbor_directions() -> HashMap<SideType, (i32, i32)> {
    let mut edge_neighbor_directions = HashMap::new();
    edge_neighbor_directions.insert(SideType::Above, (-1, 0));
    edge_neighbor_directions.insert(SideType::Below, (-1, 0));
    edge_neighbor_directions.insert(SideType::Left, (0, -1));
    edge_neighbor_directions.insert(SideType::Right, (0, -1));
    edge_neighbor_directions
}

fn get_sides_for_square(
    x: i32,
    y: i32,
    grid: &Vec<Vec<((usize, usize), u8)>>,
    side_directions: &Vec<SideDirection>,
) -> Vec<SideType> {
    side_directions
        .iter()
        .filter(|side_dir| {
            let neighbor_x = x + side_dir.dx;
            let neighbor_y = y + side_dir.dy;

            if let Some(row) = grid.get(neighbor_y as usize) {
                // middle of the grid
                if let Some((_, neighbor_label)) = row.get(neighbor_x as usize) {
                    return *neighbor_label != grid[y as usize][x as usize].1;
                } else {
                    // left or right edge
                    return true;
                }
            } else {
                // bottom or top edge
                return true;
            }
        })
        .map(|side_dir| side_dir.side_type)
        .collect::<Vec<_>>()
}

fn add_new_side_to_list(
    region_sides: &mut HashMap<SideType, Vec<HashSet<(i32, i32)>>>,
    side: SideType,
    x: &i32,
    y: &i32,
) {
    let mut new_side = HashSet::new();
    new_side.insert((*x, *y));
    region_sides
        .entry(side)
        .or_insert_with(Vec::new)
        .push(new_side);
}

fn sorted_squares_from_region(region: &Region) -> Vec<(i32, i32)> {
    // sort the squares in the region by x and then y
    let mut squares = region.squares.iter().collect::<Vec<_>>();
    squares.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));
    squares.iter().map(|(x, y)| (*x, *y)).collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<usize> {
    let grid = build_grid(input);
    let regions_by_label: HashMap<u8, Vec<Region>> = build_regions_by_label(&grid);

    let mut fence_cost = 0;
    for key in regions_by_label.keys() {
        let regions = regions_by_label.get(key).unwrap();
        for region in regions {
            let mut region_perimeter = 0;
            // println!( "Label: {}, Squares: {}", region.label as char, region.squares.len());
            for (x, y) in &region.squares {
                for (dx, dy) in &[(0, 1), (1, 0), (-1, 0), (0, -1)] {
                    let new_x = x + dx;
                    let new_y = y + dy;

                    if new_x < 0 || new_y < 0 {
                        // top or left edge
                        region_perimeter += 1;
                    } else if let Some(row) = grid.get(new_y as usize) {
                        // middle of the grid
                        if let Some((_, new_label)) = row.get(new_x as usize) {
                            if *new_label != region.label {
                                region_perimeter += 1;
                            }
                            // else it was part of the region
                        } else {
                            region_perimeter += 1;
                        }
                    } else {
                        // bottom or right edge
                        region_perimeter += 1;
                    }
                }
            }
            fence_cost += region.squares.len() * region_perimeter;
        }
    }

    Some(fence_cost)
}

pub fn part_two(input: &str) -> Option<usize> {
    let grid = build_grid(input);
    let regions_by_label: HashMap<u8, Vec<Region>> = build_regions_by_label(&grid);
    let side_directions = get_side_directions();
    let edge_neighbor_directions = get_edge_neighbor_directions();

    let mut fence_cost = 0;
    for key in regions_by_label.keys() {
        let regions = regions_by_label.get(key).unwrap();
        for region in regions {
            let mut region_sides: HashMap<SideType, Vec<HashSet<(i32, i32)>>> = HashMap::new();
            // println!( "Label: {}, Squares: {}", region.label as char, region.squares.len());
            let sorted_squares = sorted_squares_from_region(region);
            for (x, y) in sorted_squares.iter() {
                let my_sides = get_sides_for_square(*x, *y, &grid, &side_directions);
                /*println!(
                    "sides for {}: ({},{}) {:?}",
                    region.label as char, x, y, my_sides
                );*/

                for side in my_sides {
                    // check if my neighbor has the same side
                    let (dx, dy) = edge_neighbor_directions.get(&side).unwrap();
                    let neighbor_x = x + dx;
                    let neighbor_y = y + dy;

                    if region.squares.contains(&(neighbor_x, neighbor_y)) {
                        // check if that neighbor has the same side
                        let neighbor_sides =
                            get_sides_for_square(neighbor_x, neighbor_y, &grid, &side_directions);

                        /*println!(
                            "sides for neighbor {}: ({},{}) {:?}",
                            region.label as char, neighbor_x, neighbor_y, neighbor_sides
                        );*/

                        if neighbor_sides.contains(&side) {
                            // neighbor should have already made a side, find it and merge with it
                            if let Some(sides_for_dir) = region_sides.get_mut(&side) {
                                // find the side that contains the neighbor and merge with it
                                let mut found = false;
                                for some_side in sides_for_dir.iter_mut() {
                                    if some_side.contains(&(neighbor_x, neighbor_y)) {
                                        some_side.insert((*x, *y));
                                        /*println!(
                                            "found the side in the list ({},{})  -> ({},{}) {}",
                                            x, y, neighbor_x, neighbor_y, region.label as char
                                        );*/
                                        found = true;
                                        break;
                                    }
                                }
                                if !found {
                                    panic!("didn't find the side in any {:?} list ({},{})  -> ({},{}) {}", side, x, y, neighbor_x, neighbor_y, region.label as char);
                                }
                            } else {
                                panic!("didn't find the any lists with side {:?} for ({},{})  -> ({},{}) {}", side, x, y, neighbor_x, neighbor_y, region.label as char);
                            }
                        } else {
                            // our neighbor didn't have a continuous side, add our side to the region sides
                            /*println!(
                                "neighbor didn't have a continuous side ({},{})  -> ({},{}) {}",
                                x, y, neighbor_x, neighbor_y, region.label as char
                            );*/
                            add_new_side_to_list(&mut region_sides, side, x, y);
                        }
                    } else {
                        // our neighbor wasn't in our region, add our side to the region sides
                        /*println!(
                            "neighbor wasn't in our region ({},{})  -> ({},{}) {}",
                            x, y, neighbor_x, neighbor_y, region.label as char
                        );*/
                        add_new_side_to_list(&mut region_sides, side, x, y);
                    }
                }
            }
            for side_type in region_sides.keys() {
                let sides_of_type = region_sides.get(side_type).unwrap();
                /*println!(
                    "region with label {} and area {} had {} sides of type {:?}",
                    region.label as char,
                    region.squares.len(),
                    sides_of_type.len(),
                    side_type
                );*/
                fence_cost += region.squares.len() * sides_of_type.len();
            }
            //todo!("why are sides missing from regions?")
        }
    }

    Some(fence_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1930));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(
            "AAAA
BBCD
BBCC
EEEC",
        );
        assert_eq!(result, Some(140));
    }

    #[test]
    fn test_part_one_3() {
        let result = part_one(
            "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
        );
        assert_eq!(result, Some(772));
    }

    #[test]
    fn test_part_two_0() {
        let result = part_two(
            "AAAA
BBCD
BBCC
EEEC",
        );
        assert_eq!(result, Some(80));
    }

    #[test]
    fn test_part_two_1() {
        let result = part_two(
            "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE",
        );
        assert_eq!(result, Some(236));
    }
    #[test]
    fn test_part_two_2() {
        let result = part_two(
            "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA",
        );
        assert_eq!(result, Some(368));
    }
    #[test]
    fn test_part_two_3() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1206));
    }
}

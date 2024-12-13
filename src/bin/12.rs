advent_of_code::solution!(12);
use std::collections::{HashMap, HashSet};

struct Region {
    label: u8,
    squares: HashSet<(i32, i32)>,
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

pub fn part_one(input: &str) -> Option<usize> {
    let grid = input
        .lines()
        .enumerate()
        .map(|(row_num, line)| {
            line.bytes()
                .enumerate()
                .map(|(col_num, val)| ((col_num, row_num), val))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

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
    None
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

advent_of_code::solution!(10);

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum EnteredFrom {
    Head,
    Top,
    Bottom,
    Left,
    Right,
}

fn count_paths(
    grid: &Vec<Vec<u8>>,
    prev_value: u8,
    x: i32,
    y: i32,
    entered_from: EnteredFrom,
    cache: &mut HashMap<(i32, i32, EnteredFrom), u32>,
) -> u32 {
    if x < 0 || y < 0 || x >= grid[0].len() as i32 || y >= grid.len() as i32 {
        // off the side of the grid
        return 0;
    }
    let my_value = grid[y as usize][x as usize];
    if my_value == b'0' && entered_from != EnteredFrom::Head {
        // found a trail head, but not the first one
        return 0;
    }
    if (entered_from != EnteredFrom::Head) && (my_value != prev_value + 1) {
        // not the next number in the sequence
        return 0;
    }
    if grid[y as usize][x as usize] == b'9' {
        // found the end of the trail
        return 1;
    }
    if let Some(&count) = cache.get(&(x, y, entered_from)) {
        // we've been here before, return the cached value
        return count;
    }

    let count = match entered_from {
        EnteredFrom::Head => {
            count_paths(grid, my_value, x, y - 1, EnteredFrom::Top, cache)
                + count_paths(grid, my_value, x, y + 1, EnteredFrom::Bottom, cache)
                + count_paths(grid, my_value, x - 1, y, EnteredFrom::Left, cache)
                + count_paths(grid, my_value, x + 1, y, EnteredFrom::Right, cache)
        }
        EnteredFrom::Top => {
            count_paths(grid, my_value, x, y - 1, EnteredFrom::Top, cache)
                + count_paths(grid, my_value, x - 1, y, EnteredFrom::Left, cache)
                + count_paths(grid, my_value, x + 1, y, EnteredFrom::Right, cache)
        }
        EnteredFrom::Bottom => {
            count_paths(grid, my_value, x, y + 1, EnteredFrom::Bottom, cache)
                + count_paths(grid, my_value, x - 1, y, EnteredFrom::Left, cache)
                + count_paths(grid, my_value, x + 1, y, EnteredFrom::Right, cache)
        }
        EnteredFrom::Left => {
            count_paths(grid, my_value, x - 1, y, EnteredFrom::Left, cache)
                + count_paths(grid, my_value, x, y - 1, EnteredFrom::Top, cache)
                + count_paths(grid, my_value, x, y + 1, EnteredFrom::Bottom, cache)
        }
        EnteredFrom::Right => {
            count_paths(grid, my_value, x + 1, y, EnteredFrom::Right, cache)
                + count_paths(grid, my_value, x, y - 1, EnteredFrom::Top, cache)
                + count_paths(grid, my_value, x, y + 1, EnteredFrom::Bottom, cache)
        }
    };
    cache.insert((x, y, entered_from), count);
    count
}

pub fn part_one(input: &str) -> Option<u32> {
    // read input into a Vec of Vec of u8
    let grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();
    let trail_heads: Vec<(i32, i32)> = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, &cell)| {
                if cell == b'0' {
                    Some((x as i32, y as i32))
                } else {
                    None
                }
            })
        })
        .collect();

    let mut total_paths = 0;
    let mut cache = HashMap::new();
    for trail_head in trail_heads {
        total_paths += count_paths(
            &grid,
            b'0',
            trail_head.0,
            trail_head.1,
            EnteredFrom::Head,
            &mut cache,
        );
    }
    Some(total_paths)
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
        assert_eq!(result, Some(36));
    }

    #[test]
    fn test_part_one_3() {
        let result = part_one(
            "...0...
...1...
...2...
6543456
7.....7
8.....8
9.....9",
        );
        assert_eq!(result, Some(2));
    }
    #[test]
    fn test_part_one_4() {
        let result = part_one(
            "..90..9
            ...1.98
            ...2..7
            6543456
            765.987
            876....
            987....",
        );
        assert_eq!(result, Some(4));
    }
    #[test]
    fn test_part_one_5() {
        let result = part_one(
            "10..9..
2...8..
3...7..
4567654
...8..3
...9..2
.....01",
        );
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

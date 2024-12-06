use std::collections::HashSet;

advent_of_code::solution!(6);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn part_one(input: &str) -> Option<u32> {
    // read text like @examples06.txt into a 2d vec of u8
    let grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as u8).collect())
        .collect();
    // find the max x and y dimensions
    let max_x = grid[0].len();
    let max_y = grid.len();

    // find the '^' character's x,y location in the grid
    let mut x = 0;
    let mut y = 0;
    for i in 0..max_y {
        for j in 0..max_x {
            if grid[i][j] == b'^' {
                x = j;
                y = i;
                break;
            }
        }
    }

    let mut locations_visited = HashSet::new();
    let mut on_grid = true;
    let mut cur_location = (x, y);
    let mut cur_direction = Direction::Up;
    while on_grid {
        locations_visited.insert(cur_location);

        // take a step in current direction
        // if you bump into an obstacle, turn right
        // if you fall off the grid, stop
        match cur_direction {
            Direction::Up => {
                if cur_location.1 > 0 {
                    if grid[cur_location.1 - 1][cur_location.0] == b'#' {
                        cur_direction = Direction::Right;
                    } else {
                        cur_location.1 -= 1;
                    }
                } else {
                    on_grid = false;
                }
            }
            Direction::Down => {
                if cur_location.1 < max_y - 1 {
                    if grid[cur_location.1 + 1][cur_location.0] == b'#' {
                        cur_direction = Direction::Left;
                    } else {
                        cur_location.1 += 1;
                    }
                } else {
                    on_grid = false;
                }
            }
            Direction::Left => {
                if cur_location.0 > 0 {
                    if grid[cur_location.1][cur_location.0 - 1] == b'#' {
                        cur_direction = Direction::Up;
                    } else {
                        cur_location.0 -= 1;
                    }
                } else {
                    on_grid = false;
                }
            }
            Direction::Right => {
                if cur_location.0 < max_x - 1 {
                    if grid[cur_location.1][cur_location.0 + 1] == b'#' {
                        cur_direction = Direction::Down;
                    } else {
                        cur_location.0 += 1;
                    }
                } else {
                    on_grid = false;
                }
            }
        }
    }

    Some(locations_visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // read text like @examples06.txt into a 2d vec of u8
    let mut grid: Vec<Vec<u8>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as u8).collect())
        .collect();
    // find the max x and y dimensions
    let max_x = grid[0].len();
    let max_y = grid.len();

    // find the '^' character's x,y location in the grid
    let mut x = 0;
    let mut y = 0;
    for i in 0..max_y {
        for j in 0..max_x {
            if grid[i][j] == b'^' {
                x = j;
                y = i;
                break;
            }
        }
    }

    let mut loop_count = 0;
    for obs_y in 0..max_y {
        for obs_x in 0..max_x {
            // for each open space, try making it an obstacle
            if grid[obs_y][obs_x] == b'.' {
                grid[obs_y][obs_x] = b'#';
            } else {
                continue;
            }
            let mut locations_visited = HashSet::new();
            let mut on_grid = true;
            let mut cur_location = (x, y);
            let mut cur_direction = Direction::Up;
            // check if we walk off or hit a loop
            while on_grid {
                if locations_visited.contains(&((cur_location), cur_direction)) {
                    loop_count += 1;
                    break;
                }
                locations_visited.insert(((cur_location), cur_direction));

                // take a step in current direction
                // if you bump into an obstacle, turn right
                // if you fall off the grid, stop
                match cur_direction {
                    Direction::Up => {
                        if cur_location.1 > 0 {
                            if grid[cur_location.1 - 1][cur_location.0] == b'#' {
                                cur_direction = Direction::Right;
                            } else {
                                cur_location.1 -= 1;
                            }
                        } else {
                            on_grid = false;
                        }
                    }
                    Direction::Down => {
                        if cur_location.1 < max_y - 1 {
                            if grid[cur_location.1 + 1][cur_location.0] == b'#' {
                                cur_direction = Direction::Left;
                            } else {
                                cur_location.1 += 1;
                            }
                        } else {
                            on_grid = false;
                        }
                    }
                    Direction::Left => {
                        if cur_location.0 > 0 {
                            if grid[cur_location.1][cur_location.0 - 1] == b'#' {
                                cur_direction = Direction::Up;
                            } else {
                                cur_location.0 -= 1;
                            }
                        } else {
                            on_grid = false;
                        }
                    }
                    Direction::Right => {
                        if cur_location.0 < max_x - 1 {
                            if grid[cur_location.1][cur_location.0 + 1] == b'#' {
                                cur_direction = Direction::Down;
                            } else {
                                cur_location.0 += 1;
                            }
                        } else {
                            on_grid = false;
                        }
                    }
                }
            }
            grid[obs_y][obs_x] = b'.'
        }
    }

    Some(loop_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}

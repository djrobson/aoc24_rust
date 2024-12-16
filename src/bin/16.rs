advent_of_code::solution!(16);
use std::collections::HashMap;

struct Maze {
    maze: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

fn parse_input(input: &str) -> Maze {
    let mut maze = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.bytes().enumerate() {
            match c {
                b'S' => start = (x, y),
                b'E' => end = (x, y),
                _ => {}
            }
            row.push(c);
        }
        maze.push(row);
    }
    Maze { maze, start, end }
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

pub fn part_one(input: &str) -> Option<usize> {
    let maze = parse_input(input);
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((maze.start, Direction::East, 0));

    let mut best_path = usize::MAX;
    let mut best_cache: HashMap<(usize, usize, Direction), usize> =
        std::collections::HashMap::new();

    while let Some((pos, dir, steps)) = queue.pop_front() {
        let (x, y) = pos;
        if pos == maze.end {
            if steps < best_path {
                best_path = steps;
            }
            continue;
        }
        if best_cache.contains_key(&(x, y, dir)) && best_cache[&(x, y, dir)] <= steps {
            continue;
        }

        best_cache.insert((x, y, dir), steps);
        match dir {
            Direction::North => {
                if maze.maze[y - 1][x] != b'#' {
                    queue.push_back(((x, y - 1), Direction::North, steps + 1));
                }
                if maze.maze[y][x - 1] != b'#' {
                    queue.push_back(((x, y), Direction::West, steps + 1000));
                }
                if maze.maze[y][x + 1] != b'#' {
                    queue.push_back(((x, y), Direction::East, steps + 1000));
                }
            }
            Direction::South => {
                if maze.maze[y + 1][x] != b'#' {
                    queue.push_back(((x, y + 1), Direction::South, steps + 1));
                }
                if maze.maze[y][x - 1] != b'#' {
                    queue.push_back(((x, y), Direction::West, steps + 1000));
                }
                if maze.maze[y][x + 1] != b'#' {
                    queue.push_back(((x, y), Direction::East, steps + 1000));
                }
            }
            Direction::East => {
                if maze.maze[y - 1][x] != b'#' {
                    queue.push_back(((x, y), Direction::North, steps + 1000));
                }
                if maze.maze[y + 1][x] != b'#' {
                    queue.push_back(((x, y), Direction::South, steps + 1000));
                }
                if maze.maze[y][x + 1] != b'#' {
                    queue.push_back(((x + 1, y), Direction::East, steps + 1));
                }
            }
            Direction::West => {
                if maze.maze[y - 1][x] != b'#' {
                    queue.push_back(((x, y), Direction::North, steps + 1000));
                }
                if maze.maze[y + 1][x] != b'#' {
                    queue.push_back(((x, y), Direction::South, steps + 1000));
                }
                if maze.maze[y][x - 1] != b'#' {
                    queue.push_back(((x - 1, y), Direction::West, steps + 1));
                }
            }
        }
    }

    Some(best_path)
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
        assert_eq!(result, Some(7036));
    }
    #[test]
    fn test_part_one_2() {
        let result = part_one(
            "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################",
        );
        assert_eq!(result, Some(11048));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

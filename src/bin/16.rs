advent_of_code::solution!(16);
use std::collections::{HashMap, HashSet};

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
enum Dr {
    N,
    S,
    E,
    W,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Path {
    steps: usize,
    route: Vec<(usize, usize)>,
}

pub fn part_one(input: &str) -> Option<usize> {
    let maze = parse_input(input);
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((maze.start, Dr::E, 0));

    let mut best_path = usize::MAX;
    let mut best_cache: HashMap<(usize, usize, Dr), usize> = std::collections::HashMap::new();

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
            Dr::N => {
                if maze.maze[y - 1][x] != b'#' {
                    queue.push_back(((x, y - 1), Dr::N, steps + 1));
                }
                if maze.maze[y][x - 1] != b'#' {
                    queue.push_back(((x, y), Dr::W, steps + 1000));
                }
                if maze.maze[y][x + 1] != b'#' {
                    queue.push_back(((x, y), Dr::E, steps + 1000));
                }
            }
            Dr::S => {
                if maze.maze[y + 1][x] != b'#' {
                    queue.push_back(((x, y + 1), Dr::S, steps + 1));
                }
                if maze.maze[y][x - 1] != b'#' {
                    queue.push_back(((x, y), Dr::W, steps + 1000));
                }
                if maze.maze[y][x + 1] != b'#' {
                    queue.push_back(((x, y), Dr::E, steps + 1000));
                }
            }
            Dr::E => {
                if maze.maze[y - 1][x] != b'#' {
                    queue.push_back(((x, y), Dr::N, steps + 1000));
                }
                if maze.maze[y + 1][x] != b'#' {
                    queue.push_back(((x, y), Dr::S, steps + 1000));
                }
                if maze.maze[y][x + 1] != b'#' {
                    queue.push_back(((x + 1, y), Dr::E, steps + 1));
                }
            }
            Dr::W => {
                if maze.maze[y - 1][x] != b'#' {
                    queue.push_back(((x, y), Dr::N, steps + 1000));
                }
                if maze.maze[y + 1][x] != b'#' {
                    queue.push_back(((x, y), Dr::S, steps + 1000));
                }
                if maze.maze[y][x - 1] != b'#' {
                    queue.push_back(((x - 1, y), Dr::W, steps + 1));
                }
            }
        }
    }

    Some(best_path)
}

fn new_queue_entry(
    next_pos: (usize, usize),
    dir: Dr,
    steps: usize,
    path: &Path,
) -> ((usize, usize), Dr, Path) {
    let mut route = path.route.clone();
    route.push(next_pos);
    (next_pos, dir, Path { steps, route })
}

pub fn part_two(input: &str) -> Option<usize> {
    let maze = parse_input(input);
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((
        maze.start,
        Dr::E,
        Path {
            steps: 0,
            route: vec![maze.start],
        },
    ));

    //println!("Starting at {:?} and ending at {:?}", maze.start, maze.end);

    let mut best_path = usize::MAX;
    let mut best_cache: HashMap<(usize, usize, Dr), Vec<Path>> = std::collections::HashMap::new();

    while let Some((pos, dir, path)) = queue.pop_front() {
        let (x, y) = pos;
        let steps = path.steps;
        if steps > best_path {
            // we have a better path all the way to the end
            continue;
        }
        #[allow(clippy::comparison_chain)]
        if best_cache.contains_key(&(x, y, dir)) {
            if best_cache[&(x, y, dir)][0].steps < steps {
                // we have a better path to here
                continue;
            } else if best_cache[&(x, y, dir)][0].steps == steps {
                // keep track of the ties
                //println!("Found a tie at {:?} {:?} at {} steps", pos, dir, steps);
                best_cache.get_mut(&(x, y, dir)).unwrap().push(path.clone());
            } else {
                // this is the new best
                //println!("New best path to {:?} {:?} at {} steps", pos, dir, steps);
                best_cache.insert((x, y, dir), vec![path.clone()]);
            }
        } else {
            // this is our first time here
            //println!("First time at {:?} {:?} after {} steps", pos, dir, steps);
            best_cache.insert((x, y, dir), vec![path.clone()]);
        }
        if pos == maze.end {
            // this is the end, we have a new best path
            //println!("Path to finish in {} steps: {:?}", steps, path);
            if steps < best_path {
                best_path = steps;
            }
            continue;
        }

        match dir {
            Dr::N => {
                if maze.maze[y - 1][x] != b'#' {
                    queue.push_back(new_queue_entry((x, y - 1), dir, steps + 1, &path));
                }
                if maze.maze[y][x - 1] != b'#' {
                    queue.push_back(new_queue_entry((x, y), Dr::W, steps + 1000, &path));
                }
                if maze.maze[y][x + 1] != b'#' {
                    queue.push_back(new_queue_entry((x, y), Dr::E, steps + 1000, &path));
                }
            }
            Dr::S => {
                if maze.maze[y + 1][x] != b'#' {
                    queue.push_back(new_queue_entry((x, y + 1), dir, steps + 1, &path));
                }
                if maze.maze[y][x - 1] != b'#' {
                    queue.push_back(new_queue_entry((x, y), Dr::W, steps + 1000, &path));
                }
                if maze.maze[y][x + 1] != b'#' {
                    queue.push_back(new_queue_entry((x, y), Dr::E, steps + 1000, &path));
                }
            }
            Dr::E => {
                if maze.maze[y - 1][x] != b'#' {
                    queue.push_back(new_queue_entry((x, y), Dr::N, steps + 1000, &path));
                }
                if maze.maze[y + 1][x] != b'#' {
                    queue.push_back(new_queue_entry((x, y), Dr::S, steps + 1000, &path));
                }
                if maze.maze[y][x + 1] != b'#' {
                    queue.push_back(new_queue_entry((x + 1, y), dir, steps + 1, &path));
                }
            }
            Dr::W => {
                if maze.maze[y - 1][x] != b'#' {
                    queue.push_back(new_queue_entry((x, y), Dr::N, steps + 1000, &path));
                }
                if maze.maze[y + 1][x] != b'#' {
                    queue.push_back(new_queue_entry((x, y), Dr::S, steps + 1000, &path));
                }
                if maze.maze[y][x - 1] != b'#' {
                    queue.push_back(new_queue_entry((x - 1, y), dir, steps + 1, &path));
                }
            }
        }
    }

    let mut steps = HashSet::new();
    for direction in [Dr::N, Dr::S, Dr::E, Dr::W].iter() {
        if let Some(dir_routes) = best_cache.get(&(maze.end.0, maze.end.1, *direction)) {
            /*println!(
                "\nResults:\nfound {} routes pointing {:?}",
                dir_routes.len(),
                direction
            );*/
            for route in dir_routes {
                if route.steps != best_path {
                    continue;
                }
                //println!("Path: {:?}", route);
                for step in route.route.iter() {
                    steps.insert(step);
                }
            }
        }
    }

    Some(steps.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two_big() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(593));
    }

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
    fn test_part_two_1() {
        let result = part_two(
            "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############",
        );
        assert_eq!(result, Some(45));
    }
    #[test]
    fn test_part_two_2() {
        let result = part_two(
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
        assert_eq!(result, Some(64));
    }
}

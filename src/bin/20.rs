advent_of_code::solution!(20);

use std::collections::HashMap;

use itertools::Itertools;

struct Maze {
    maze: Vec<Vec<u8>>,
    start: (usize, usize),
    end: (usize, usize),
}

fn parse_input(input: &str) -> Maze {
    let mut maze = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, line) in input.lines().enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.bytes().enumerate() {
            if c == b'S' {
                start = (x, y);
                row.push(b'.');
            } else if c == b'E' {
                end = (x, y);
                row.push(b'.');
            } else {
                row.push(c);
            }
        }
        maze.push(row);
    }

    Maze { maze, start, end }
}

fn get_steps_for_maze(maze: &Maze) -> Option<usize> {
    let mut best_cache : HashMap<(usize,usize), usize> = HashMap::new();
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((maze.start,0));
    let mut best_path = usize::MAX;

    while let Some((pos, steps)) = queue.pop_front() {
        let (x, y) = pos;
        if steps > best_path {
            // we have a better path all the way to the end
            continue;
        }
        if best_cache.contains_key(&(x, y)) && best_cache[&(x, y)] <= steps {
            // we have a better path to here
            continue;
        }

        best_cache.insert((x, y), steps);
        if pos == maze.end {
            // this is the end, we have a new best path
            //println!("Path to finish in {} steps: {:?}", steps, path);
            if steps < best_path {
                best_path = steps;
            }
            continue;
        }

        if maze.maze[y - 1][x] != b'#' {
            queue.push_back(((x, y - 1), steps + 1));
        }
        if maze.maze[y][x - 1] != b'#' {
            queue.push_back(((x-1, y), steps + 1));
        }
        if maze.maze[y][x + 1] != b'#' {
            queue.push_back(((x+1, y), steps + 1));
        }
        if maze.maze[y + 1][x] != b'#' {
            queue.push_back(((x, y + 1),  steps + 1));
        }
    }
    if best_path == usize::MAX {
        None
    } else {
        Some(best_path)
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    part_one_with_limit(input, 100)
}

pub fn part_one_with_limit(input: &str, limit:usize) -> Option<usize> {
    let mut maze = parse_input(input);

    let max_y = maze.maze.len();
    let max_x = maze.maze[0].len();
    let default_route = get_steps_for_maze(&maze).expect("default maze was unsolvable");
    println!("Baseline route takes {} steps", default_route);

    let min_better_amount = limit;
    let mut better_routes :HashMap<usize, Vec<(usize,usize)>> = HashMap::new();
    for y in 0..max_y {
        for x in 0..max_x {
            if y == 0 || y >= max_y-1 || x == 0 || x >= max_x-1 || maze.maze[y][x] != b'#' {
                continue;
            }
            // swap the wall for a space and see if we solve faster
            maze.maze[y][x] = b'.';
            let new_solve = get_steps_for_maze(&maze).expect("removing a wall made the maze unsolvable?");

            let route_improvement = default_route - new_solve;
            //println!("Removing wall at {} {} made a route that is {} better at {}", x,y,default_route - new_solve, new_solve);
            if better_routes.contains_key(&route_improvement) {
                better_routes.get_mut(&route_improvement).unwrap().push((x,y));
            } else {
                better_routes.insert(route_improvement, vec![(x,y)]);
            }
            maze.maze[y][x] = b'#';
        }
    }
    let mut better_count = 0;
    for key in better_routes.keys().sorted().filter(|k| **k >= min_better_amount) {
        better_count += better_routes[key].len();
        println!("Found {} routes that are {} better", better_routes[key].len(), key);
    }
    Some(better_count)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one_with_limit(&advent_of_code::template::read_file("examples", DAY),1);
        assert_eq!(result, Some(44));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

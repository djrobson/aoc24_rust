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
            if let std::collections::hash_map::Entry::Vacant(e) = better_routes.entry(route_improvement) {
                e.insert(vec![(x,y)]);
            } else {
                better_routes.get_mut(&route_improvement).unwrap().push((x,y));
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

fn get_list_of_nearby_spots(pos: (usize,usize), distance: i32, maze: &Maze) -> Vec<(usize,usize)> {
    let max_x = maze.maze[0].len();
    let max_y = maze.maze.len();
    let mut spots = Vec::new();
    // if spot is inside the maze, and == '.' and distance is less than distance
    let (x, y) = pos;
    for x_offset in -distance..=distance {
        for y_offset in -distance..=distance {
            if x_offset.abs() + y_offset.abs() > distance {
                continue;
            }
            let new_x = x as i32 + x_offset;
            let new_y = y as i32 + y_offset;
            if new_x < 0 || new_x >= max_x as i32 || new_y < 0 || new_y >= max_y as i32 {
                continue;
            }
            let new_x = new_x as usize;
            let new_y = new_y as usize;
            if maze.maze[new_y][new_x] == b'.' {
                spots.push((new_x, new_y));
            }
        }
    }
    spots
}
pub fn part_two(input: &str) -> Option<u32> {
    part_two_ex(input, 20, 100)
}
pub fn part_two_ex(input: &str, shortcut_distance: i32, min_improvement: usize) -> Option<u32> {
    let maze = parse_input(input);
    let mut maze_steps_for_each_position: HashMap<(usize,usize),usize> = HashMap::new();
    let mut route = Vec::new();
    let mut cursor = maze.start;
    let mut steps = 0;
    maze_steps_for_each_position.insert(cursor, steps);
    while cursor != maze.end {
        let (x, y) = cursor;
        route.push(cursor);
        steps +=1;
        if steps > 10000 {
            panic!("Too many steps");
        }
        if maze.maze[y - 1][x] != b'#' && !maze_steps_for_each_position.contains_key(&(x, y - 1)) {
            cursor = (x, y - 1);
            maze_steps_for_each_position.insert(cursor, steps);
            continue;
        }
        if maze.maze[y][x - 1] != b'#' && !maze_steps_for_each_position.contains_key(&(x - 1, y)) {
            cursor = (x-1, y);
            maze_steps_for_each_position.insert(cursor, steps);
            continue;
        }
        if maze.maze[y][x + 1] != b'#' && !maze_steps_for_each_position.contains_key(&(x + 1, y)) {
            cursor = (x+1, y);
            maze_steps_for_each_position.insert(cursor, steps);
            continue;
        }
        if maze.maze[y + 1][x] != b'#' && !maze_steps_for_each_position.contains_key(&(x, y + 1)) {
            cursor = (x, y + 1);
            maze_steps_for_each_position.insert(cursor, steps);
            continue;
        }
    }
    route.push(cursor); // add the end

    let mut shortcut_count: HashMap<usize, usize> = HashMap::new();
    for here in route.iter() {
        get_list_of_nearby_spots(*here, shortcut_distance, &maze).iter().for_each(|shortcut| {
            let steps_to_here =  maze_steps_for_each_position[here];

            // calculate the absolute value of the x,y offset from here to shortcut
            let steps_to_shortcut = maze_steps_for_each_position[shortcut];
            let x_offset= (here.0 as i32 - shortcut.0 as i32).unsigned_abs() as usize;
            let y_offset = (here.1 as i32 - shortcut.1 as i32).unsigned_abs() as usize;
            let steps_from_here_to_shortcut = x_offset + y_offset;
            if steps_to_shortcut > (steps_to_here + steps_from_here_to_shortcut) {
                // we can get to the shortcut faster than going to normal way
                let steps_to_end_from_here = maze_steps_for_each_position[&maze.end] - steps_to_here;
                let steps_to_end_from_shortcut = maze_steps_for_each_position[&maze.end] - steps_to_shortcut;
                let steps_to_end_via_shortcut = steps_from_here_to_shortcut + steps_to_end_from_shortcut;
                let steps_saved = steps_to_end_from_here - steps_to_end_via_shortcut;
                if shortcut_count.contains_key(&steps_saved) {
                    shortcut_count.insert(steps_saved, shortcut_count[&steps_saved] + 1);
                } else {
                    shortcut_count.insert(steps_saved, 1);
                }
            }
        });
    }

    let mut total_shortcuts = 0;
    for key in shortcut_count.keys().sorted() {
        if key >= &min_improvement {
            total_shortcuts += shortcut_count[key];

            println!("Found {} shortcuts that save {} steps", shortcut_count[key], key);
        } else {
            println!("Found {} shortcuts that save {} too few steps", shortcut_count[key], key);
        }
    }

    Some(total_shortcuts as u32)
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
        let result = part_two_ex(&advent_of_code::template::read_file("examples", DAY), 6, 50);
        assert_eq!(result, Some(46));
    }
    #[test]
    #[ignore]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(986082));
    }
}

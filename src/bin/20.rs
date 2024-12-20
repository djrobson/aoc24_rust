advent_of_code::solution!(20);

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

pub fn part_one(input: &str) -> Option<u32> {
    let maze = parse_input(input);

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(maze.start);

    // get the best baseline length

    // for each wall, remove it, count if we got to the end faster, put it back

    //println!("Starting at {:?} and ending at {:?}", maze.start, maze.end);

    let mut best_path = usize::MAX;

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

    Some(better_path_count)
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

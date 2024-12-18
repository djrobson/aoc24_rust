use core::fmt;

advent_of_code::solution!(18);

fn print_grid(grid: &Vec<Vec<u32>>, start: &(u32, u32), end: &(u32, u32)) {
    for (y, row) in grid.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if (x as u32, y as u32) == *start {
                print!("S");
            } else if (x as u32, y as u32) == *end {
                print!("E");
            } else if *cell == u32::MAX {
                print!("X");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn solve_one(
    x_max: u32,
    y_max: u32,
    max_move: u32,
    falling_sequence: Vec<(u32, u32)>,
) -> Option<u32> {
    let mut grid = vec![vec![1; x_max as usize]; y_max as usize];
    let start = (0, 0);
    let end = (x_max - 1, y_max - 1);
    for (i, (x, y)) in falling_sequence.iter().enumerate() {
        if i < max_move as usize {
            grid[*y as usize][*x as usize] = u32::MAX;
        }
    }
    //print_grid(&grid, &start, &end);

    // use djikstra to find a path from 0,0 to x,y
    let mut visited = vec![vec![false; x_max as usize]; y_max as usize];
    let mut distance = vec![vec![std::u32::MAX; x_max as usize]; y_max as usize];
    let mut queue = std::collections::BinaryHeap::new();
    queue.push(std::cmp::Reverse((0, start)));
    distance[start.1 as usize][start.0 as usize] = 0;
    while let Some(std::cmp::Reverse((dist, (x, y)))) = queue.pop() {
        if visited[y as usize][x as usize] {
            continue;
        }
        visited[y as usize][x as usize] = true;
        if (x, y) == end {
            return Some(dist);
        }
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < x_max as i32 && ny >= 0 && ny < y_max as i32 {
                let nx = nx as usize;
                let ny = ny as usize;
                if !visited[ny][nx] {
                    let new_dist = grid[ny][nx].saturating_add(dist);
                    if new_dist < distance[ny][nx] {
                        distance[ny][nx] = new_dist;
                        queue.push(std::cmp::Reverse((new_dist, (nx as u32, ny as u32))));
                    }
                }
            }
        }
    }
    // print the grid distance for all visited nodes
    /*for (y, row) in visited.iter().enumerate() {
        for (x, cell) in row.iter().enumerate() {
            if *cell {
                print!("{}", distance[y][x] % 10);
            } else {
                print!("_");
            }
        }
        println!();
    }*/

    None
}

pub fn part_one(input: &str) -> Option<u32> {
    let falling_sequence = input
        .lines()
        .map(|line| {
            let mut parts = line.split(",");
            (
                parts.next().unwrap().parse::<u32>().unwrap(),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();
    solve_one(71, 71, 1024, falling_sequence)
}

fn solve_two(x_max: u32, y_max: u32, grid: &Vec<Vec<u32>>) -> Option<u32> {
    let start = (0, 0);
    let end = (x_max - 1, y_max - 1);

    //print_grid(&grid, &start, &end);

    // use djikstra to find a path from 0,0 to x,y
    let mut visited = vec![vec![false; x_max as usize]; y_max as usize];
    let mut distance = vec![vec![std::u32::MAX; x_max as usize]; y_max as usize];
    let mut queue = std::collections::BinaryHeap::new();
    queue.push(std::cmp::Reverse((0, start)));
    distance[start.1 as usize][start.0 as usize] = 0;
    while let Some(std::cmp::Reverse((dist, (x, y)))) = queue.pop() {
        if visited[y as usize][x as usize] {
            continue;
        }
        visited[y as usize][x as usize] = true;
        if (x, y) == end {
            return Some(dist);
        }
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && nx < x_max as i32 && ny >= 0 && ny < y_max as i32 {
                let nx = nx as usize;
                let ny = ny as usize;
                if !visited[ny][nx] {
                    let new_dist = grid[ny][nx].saturating_add(dist);
                    if new_dist < distance[ny][nx] {
                        distance[ny][nx] = new_dist;
                        queue.push(std::cmp::Reverse((new_dist, (nx as u32, ny as u32))));
                    }
                }
            }
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<String> {
    wrapped_two(input, 71, 71)
}

fn wrapped_two(input: &str, x_max: u32, y_max: u32) -> Option<String> {
    let falling_sequence = input
        .lines()
        .map(|line| {
            let mut parts = line.split(",");
            (
                parts.next().unwrap().parse::<u32>().unwrap(),
                parts.next().unwrap().parse::<u32>().unwrap(),
            )
        })
        .collect::<Vec<_>>();

    let mut grid = vec![vec![1; x_max as usize]; y_max as usize];
    for drop in falling_sequence.iter() {
        grid[drop.1 as usize][drop.0 as usize] = u32::MAX;
        if solve_two(x_max, x_max, &grid).is_none() {
            //println!("{},{}", drop.0, drop.1);
            return Some(format!("{},{}", drop.0, drop.1));
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_one(
            7,
            7,
            12,
            vec![
                (5, 4),
                (4, 2),
                (4, 5),
                (3, 0),
                (2, 1),
                (6, 3),
                (2, 4),
                (1, 5),
                (0, 6),
                (3, 3),
                (2, 6),
                (5, 1),
            ],
        );
        assert_eq!(result, Some(22));
    }

    #[test]
    fn test_part_two() {
        let result = wrapped_two(&advent_of_code::template::read_file("examples", DAY), 7, 7);
        assert_eq!(result, Some("6,1".to_string()));
    }
}

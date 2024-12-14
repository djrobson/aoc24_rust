use std::collections::HashSet;

advent_of_code::solution!(14);

#[derive(Debug)]
struct Robot {
    x_loc: i32,
    y_loc: i32,
    x_vel: i32,
    y_vel: i32,
}
fn print_robots_on_grid(robots: &Vec<Robot>, grid_max_x: usize, grid_max_y: usize) {
    let mut grid = vec![vec!['.'; grid_max_x]; grid_max_y];
    for robot in robots.iter() {
        grid[robot.y_loc as usize][robot.x_loc as usize] = '#';
    }
    for row in grid.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

fn process_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            // split line like p=6,3 v=-1,-3
            let mut positions = line
                .split(" ")
                .nth(0)
                .unwrap()
                .split("=")
                .nth(1)
                .unwrap()
                .split(",");
            let mut velocities = line
                .split(" ")
                .nth(1)
                .unwrap()
                .split("=")
                .nth(1)
                .unwrap()
                .split(",");
            Robot {
                x_loc: positions.next().unwrap().parse::<i32>().unwrap(),
                y_loc: positions.next().unwrap().parse::<i32>().unwrap(),
                x_vel: velocities.next().unwrap().parse::<i32>().unwrap(),
                y_vel: velocities.next().unwrap().parse::<i32>().unwrap(),
            }
        })
        .collect()
}

fn solve_one(input: &str, grid_max_x: usize, grid_max_y: usize) -> Option<u32> {
    let mut robots = process_input(input);
    let ticks = 100;
    for robot in robots.iter_mut() {
        robot.x_loc = (robot.x_loc + robot.x_vel * ticks).rem_euclid(grid_max_x as i32);
        robot.y_loc = (robot.y_loc + robot.y_vel * ticks).rem_euclid(grid_max_y as i32);
        //println!("x: {}, y: {}", robot.x_loc, robot.y_loc);
    }

    //print_robots_on_grid(&robots, grid_max_x, grid_max_y);

    let quad1 = (0, 0, grid_max_x / 2, grid_max_y / 2); // top left
    let quad2 = (grid_max_x / 2 + 1, 0, grid_max_x, grid_max_y / 2); // top right
    let quad3 = (0, grid_max_y / 2 + 1, grid_max_x / 2, grid_max_y); // bottom left
    let quad4 = (
        grid_max_x / 2 + 1,
        grid_max_y / 2 + 1,
        grid_max_x,
        grid_max_y,
    ); // bottom right

    let quads = vec![quad1, quad2, quad3, quad4];

    let mut saftey_factor = 1;
    for quad in quads {
        let mut count = 0;
        for robot in robots.iter() {
            if robot.x_loc >= quad.0 as i32
                && robot.x_loc < quad.2 as i32
                && robot.y_loc >= quad.1 as i32
                && robot.y_loc < quad.3 as i32
            {
                count += 1;
            }
        }
        /*println!(
            "count:({},{}), ({},{}) {}",
            quad.0, quad.1, quad.2, quad.3, count
        );*/
        saftey_factor *= count;
    }

    Some(saftey_factor)
}

pub fn part_one(input: &str) -> Option<u32> {
    solve_one(input, 101, 103)
}

fn solve_two(input: &str, grid_max_x: usize, grid_max_y: usize) -> Option<u32> {
    let robots = process_input(input);
    let mut tick = 0;
    loop {
        //let mut grid = vec![vec!['.'; grid_max_x]; grid_max_y];
        let mut grid_locs = HashSet::new();
        for robot in robots.iter() {
            let new_x = (robot.x_loc + robot.x_vel * tick).rem_euclid(grid_max_x as i32);
            let new_y = (robot.y_loc + robot.y_vel * tick).rem_euclid(grid_max_y as i32);
            grid_locs.insert((new_x, new_y));
        }
        if grid_locs.len() == robots.len() {
            print_robots_on_grid(&robots, grid_max_x, grid_max_y);
            return Some(tick as u32);
        }

        tick += 1;
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    solve_two(input, 101, 103)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = solve_one(&advent_of_code::template::read_file("examples", DAY), 11, 7);
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two_real() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(6243));
    }
}

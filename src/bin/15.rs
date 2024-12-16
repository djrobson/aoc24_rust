advent_of_code::solution!(15);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LocationType {
    Wall,
    Open,
    Box,
    Robot,
}

struct GameData {
    grid: Vec<Vec<LocationType>>,
    robot_location: (usize, usize),
    //boxes: Vec<(usize, usize)>,
    //walls: Vec<(usize, usize)>,
    grid_size: (usize, usize),
    directions: Vec<u8>,
}

#[allow(dead_code)]
fn print_grid(grid: &[Vec<LocationType>], robot_location: &(usize, usize)) {
    for (y, row) in grid.iter().enumerate() {
        for (x, location) in row.iter().enumerate() {
            if (x, y) == *robot_location {
                print!("@");
            } else {
                match location {
                    LocationType::Wall => print!("#"),
                    LocationType::Open => print!("."),
                    LocationType::Box => print!("O"),
                    LocationType::Robot => print!("@"),
                }
            }
        }
        println!();
    }
}

fn parse_input(input: &str) -> GameData {
    let input_parts = input.split("\n\n").collect::<Vec<&str>>();
    let mut boxes = Vec::new();
    let mut walls = Vec::new();
    let mut robot_location = (0, 0);
    let grid_data: Vec<Vec<LocationType>> = input_parts[0]
        .lines()
        .enumerate()
        .map(|(line_num, line)| {
            line.chars()
                .enumerate()
                .map(|(char_num, c)| match c {
                    '#' => {
                        walls.push((char_num, line_num));
                        LocationType::Wall
                    }
                    '.' => LocationType::Open,
                    'O' => {
                        boxes.push((char_num, line_num));
                        LocationType::Box
                    }
                    '@' => {
                        robot_location = (line_num, char_num);
                        LocationType::Robot
                    }
                    _ => panic!("Unknown location type: {}", c),
                })
                .collect()
        })
        .collect();
    let mut directions: Vec<u8> = Vec::new();
    for dir_line in input_parts[1].lines() {
        for c in dir_line.chars() {
            directions.push(c as u8);
        }
    }
    let grid_size = (grid_data[0].len(), grid_data.len());
    GameData {
        grid: grid_data,
        robot_location,
        //boxes,
        //walls,
        grid_size,
        directions,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut game_data = parse_input(input);
    //print_grid(&game_data.grid, &game_data.robot_location);
    game_data.grid[game_data.robot_location.1][game_data.robot_location.0] = LocationType::Open;

    for next_move in game_data.directions {
        let (robot_x, robot_y) = game_data.robot_location;
        //println! ("Next move: {}", next_move as char);
        let (vector_x, vector_y) = match next_move {
            b'^' => (0, -1),
            b'v' => (0, 1),
            b'<' => (-1, 0),
            b'>' => (1, 0),
            _ => panic!("Unknown direction: {}", next_move),
        };
        let next_x = robot_x.checked_add_signed(vector_x).unwrap();
        let next_y = robot_y.checked_add_signed(vector_y).unwrap();
        match game_data.grid[next_y][next_x] {
            LocationType::Wall => {
                // do nothing
            }
            LocationType::Box => {
                // start scanning over boxes in direction, if you hit an empty spot then swap the box into the empty spot
                // if you find a wall before you find an empty spot then do nothing
                let mut next_box_x = next_x.checked_add_signed(vector_x).unwrap();
                let mut next_box_y = next_y.checked_add_signed(vector_y).unwrap();
                while next_box_x < game_data.grid_size.0 && next_box_y < game_data.grid_size.1 {
                    match game_data.grid[next_box_y][next_box_x] {
                        LocationType::Wall => {
                            // block the movement of the box
                            break;
                        }
                        LocationType::Box => {
                            next_box_x = next_box_x.checked_add_signed(vector_x).unwrap();
                            next_box_y = next_box_y.checked_add_signed(vector_y).unwrap();
                        }
                        LocationType::Open => {
                            // push the box to the end and make a new open spot for the robot
                            game_data.grid[next_y][next_x] = LocationType::Open;
                            game_data.grid[next_box_y][next_box_x] = LocationType::Box;
                            game_data.robot_location = (next_x, next_y);

                            //game_data.boxes.retain(|&x| x != (box_x, box_y));
                            //game_data.boxes.push((next_box_x as usize, next_box_y as usize));
                            break;
                        }
                        _ => panic!("Unknown location type"),
                    }
                }
            }
            LocationType::Open => {
                // move the robot
                game_data.robot_location = (next_x, next_y);
            }
            _ => panic!("Unknown location type"),
        }
        //print_grid(&game_data.grid, &game_data.robot_location);
    }

    let mut gps_sum = 0;
    for y in 0..game_data.grid.len() {
        for x in 0..game_data.grid[y].len() {
            if game_data.grid[y][x] == LocationType::Box {
                gps_sum += y * 100 + x
            }
        }
    }

    Some(gps_sum as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10092));
    }

    #[test]
    fn test_part_one_2() {
        let result = part_one(
            "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
        );
        assert_eq!(result, Some(2028));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

advent_of_code::solution!(15);
use itertools::Itertools;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LocationType2 {
    Wall,
    Open,
    BoxLeft,
    BoxRight,
    Robot,
}

struct GameData2 {
    grid: Vec<Vec<LocationType2>>,
    robot_location_x: usize,
    robot_location_y: usize,
    //boxes: Vec<(usize, usize)>,
    //walls: Vec<(usize, usize)>,
    grid_size: (usize, usize),
    directions: Vec<u8>,
}

#[allow(dead_code)]
fn print_grid2(grid: &[Vec<LocationType2>], robot_location_x: &usize, robot_location_y: &usize) {
    for (y, row) in grid.iter().enumerate() {
        for (x, location) in row.iter().enumerate() {
            if (x, y) == (*robot_location_x, *robot_location_y) {
                print!("@");
            } else {
                match location {
                    LocationType2::Wall => print!("#"),
                    LocationType2::Open => print!("."),
                    LocationType2::BoxLeft => print!("["),
                    LocationType2::BoxRight => print!("]"),
                    LocationType2::Robot => print!("@"),
                }
            }
        }
        println!();
    }
}

fn parse_input2(input: &str) -> GameData2 {
    let input_parts = input.split("\n\n").collect::<Vec<&str>>();
    let mut robot_location_x = 0;
    let mut robot_location_y = 0;
    let grid_data: Vec<Vec<LocationType2>> = input_parts[0]
        .lines()
        .enumerate()
        .map(|(line_num, line)| {
            line.chars()
                .enumerate()
                .map(|(char_num, c)| match c {
                    '#' => (LocationType2::Wall, LocationType2::Wall),
                    '.' => (LocationType2::Open, LocationType2::Open),
                    'O' => (LocationType2::BoxLeft, LocationType2::BoxRight),
                    '@' => {
                        robot_location_x = char_num * 2;
                        robot_location_y = line_num;
                        (LocationType2::Robot, LocationType2::Open)
                    }
                    _ => panic!("Unknown location type: {}", c),
                })
                .fold(Vec::new(), |mut acc, (left, right)| {
                    acc.push(left);
                    acc.push(right);
                    acc
                })
        })
        .collect();
    let mut directions: Vec<u8> = Vec::new();
    for dir_line in input_parts[1].lines() {
        for c in dir_line.chars() {
            directions.push(c as u8);
        }
    }
    let grid_size = (grid_data[0].len(), grid_data.len());
    GameData2 {
        grid: grid_data,
        robot_location_x,
        robot_location_y,
        grid_size,
        directions,
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut game_data = parse_input2(input);
    /*print_grid2(
        &game_data.grid,
        &game_data.robot_location_x,
        &game_data.robot_location_y,
    );*/
    game_data.grid[game_data.robot_location_y][game_data.robot_location_x] = LocationType2::Open;

    for next_move in game_data.directions.iter() {
        let robot_x = game_data.robot_location_x;
        let robot_y = game_data.robot_location_y;
        let (vector_x, vector_y) = match next_move {
            b'^' => (0, -1),
            b'v' => (0, 1),
            b'<' => (-1, 0),
            b'>' => (1, 0),
            _ => panic!("Unknown direction: {}", next_move),
        };

        let next_x = robot_x.checked_add_signed(vector_x).unwrap();
        let next_y = robot_y.checked_add_signed(vector_y).unwrap();
        match next_move {
            b'<' | b'>' => match game_data.grid[next_y][next_x] {
                // if left or right then find an open spot and shift all the left/right pairs over one
                LocationType2::Open => {
                    // move the robot
                    game_data.robot_location_x = next_x;
                    game_data.robot_location_y = next_y;
                }
                LocationType2::Wall => {
                    // do nothing
                }
                LocationType2::BoxLeft | LocationType2::BoxRight => {
                    //println!("moving left or right and found a box");
                    // start scanning over boxes in direction, if you hit an empty spot then swap the box into the empty spot
                    // if you find a wall before you find an empty spot then do nothing
                    let mut next_box_x = next_x.checked_add_signed(vector_x).unwrap();
                    let mut next_box_y = next_y.checked_add_signed(vector_y).unwrap();
                    while next_box_x < game_data.grid_size.0 && next_box_y < game_data.grid_size.1 {
                        /*println!(
                            "Next box: {}, {} = {:?}",
                            next_box_x, next_box_y, game_data.grid[next_box_y][next_box_x]
                        );*/
                        match game_data.grid[next_box_y][next_box_x] {
                            LocationType2::Wall => {
                                // block the movement of the box
                                break;
                            }
                            LocationType2::BoxLeft => {
                                next_box_x = next_box_x.checked_add_signed(vector_x).unwrap();
                                next_box_y = next_box_y.checked_add_signed(vector_y).unwrap();
                            }
                            LocationType2::BoxRight => {
                                next_box_x = next_box_x.checked_add_signed(vector_x).unwrap();
                                next_box_y = next_box_y.checked_add_signed(vector_y).unwrap();
                            }
                            LocationType2::Open => {
                                // push the box to the end and make a new open spot for the robot
                                //for boxes between next_x, next_y and next_box_x shift over
                                if next_box_x < next_x {
                                    for x in next_box_x..next_x {
                                        game_data.grid[next_y][x] = game_data.grid[next_y]
                                            [x.checked_add_signed(-vector_x).unwrap()];
                                    }
                                } else {
                                    for x in (next_x..next_box_x).rev() {
                                        game_data.grid[next_y]
                                            [x.checked_add_signed(vector_x).unwrap()] =
                                            game_data.grid[next_y][x];
                                    }
                                }
                                game_data.grid[next_y][next_x] = LocationType2::Open;
                                game_data.robot_location_x = next_x;
                                game_data.robot_location_y = next_y;
                                break;
                            }
                            _ => panic!("Unknown location type"),
                        }
                    }
                }
                _ => panic!("Unknown location type"),
            },
            b'^' | b'v' => match game_data.grid[next_y][next_x] {
                // up or down then scan upwards in the possible pyramic of boxes and move them all up or down
                LocationType2::Wall => {
                    // we hit a wall right away, do nothing
                }
                LocationType2::BoxLeft | LocationType2::BoxRight => {
                    // start scanning over boxes in direction,
                    // if every box has an empty spot then push all of them,
                    // otherwise do nothing

                    let mut boxes_to_move = Vec::new();
                    boxes_to_move.push((next_x, next_y));
                    match game_data.grid[next_y][next_x] {
                        LocationType2::BoxLeft => {
                            boxes_to_move.push((next_x + 1, next_y));
                        }
                        LocationType2::BoxRight => {
                            boxes_to_move.push((next_x - 1, next_y));
                        }
                        _ => panic!("unmatched box pair"),
                    }

                    let mut found_wall = false;
                    let mut boxes_checked = 0;
                    // for each box in boxes to move, scan up or down until you hit a wall or open spot
                    while boxes_checked < boxes_to_move.len() {
                        let (box_x, box_y) = boxes_to_move[boxes_checked];
                        // check in the vector_y direction
                        let next_box_x = box_x.checked_add_signed(vector_x).unwrap();
                        let next_box_y = box_y.checked_add_signed(vector_y).unwrap();
                        match game_data.grid[next_box_y][next_box_x] {
                            LocationType2::Wall => {
                                // block the movement of the box
                                found_wall = true;
                                break;
                            }
                            LocationType2::BoxLeft => {
                                boxes_to_move.push((next_box_x, next_box_y));
                                boxes_to_move.push((next_box_x + 1, next_box_y));
                            }
                            LocationType2::BoxRight => {
                                boxes_to_move.push((next_box_x, next_box_y));
                                boxes_to_move.push((next_box_x - 1, next_box_y));
                            }
                            LocationType2::Open => {
                                // do nothing
                            }
                            _ => panic!("Unknown location type"),
                        }
                        boxes_checked += 1;
                    }

                    /*println!(
                        "Box parts to move: {:?} wall found {:?}",
                        boxes_to_move, found_wall
                    );*/

                    /*print_grid2(
                        &game_data.grid,
                        &game_data.robot_location_x,
                        &game_data.robot_location_y,
                    );*/
                    if !found_wall {
                        for box_to_move in boxes_to_move.iter().unique().rev() {
                            let (box_x, box_y) = box_to_move;
                            game_data.grid[box_y.checked_add_signed(vector_y).unwrap()][*box_x] =
                                game_data.grid[*box_y][*box_x];

                            game_data.grid[*box_y][*box_x] = LocationType2::Open;

                            /*print_grid2(
                                &game_data.grid,
                                &game_data.robot_location_x,
                                &game_data.robot_location_y,
                            );*/
                        }
                        game_data.robot_location_x = next_x;
                        game_data.robot_location_y = next_y;
                    }
                }
                LocationType2::Open => {
                    // move the robot
                    game_data.robot_location_x = next_x;
                    game_data.robot_location_y = next_y;
                }
                _ => panic!("Unknown location type"),
            },
            _ => panic!("Unknown direction"),
        }
        /*print_grid2(
            &game_data.grid,
            &game_data.robot_location_x,
            &game_data.robot_location_y,
        );*/
    }
    /*print_grid2(
        &game_data.grid,
        &game_data.robot_location_x,
        &game_data.robot_location_y,
    );*/

    let mut gps_sum = 0;
    for y in 0..game_data.grid.len() {
        for x in 0..game_data.grid[y].len() {
            if game_data.grid[y][x] == LocationType2::BoxLeft {
                //println!("Box found at: {}, {}", x, y);
                gps_sum += y * 100 + x
            }
        }
    }

    Some(gps_sum as u32)
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
        assert_eq!(result, Some(9021));
    }

    #[test]
    #[ignore]
    fn test_part_two_input() {
        let result = part_two(&advent_of_code::template::read_file("input", DAY));
        assert_eq!(result, Some(9021));
    }

    #[test]
    fn test_part_two_up() {
        let result = part_two(
            "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O...O.#
#O#..O...#
#O...O.O.#
#.OO.@.OO#
#....O...#
##########

^",
        );
        assert_eq!(result, Some(8894));
    }
    #[test]
    fn test_part_two_over_up() {
        let result = part_two(
            "##########
#..O....O#
#...@OOO.#
#.OO.O.OO#
#........#
##########

>v>v>^",
        );
        assert_eq!(result, Some(2109));
    }
    #[test]
    fn test_part_two_over_over() {
        let result = part_two(
            "#####
#@O.#
#####

>>>",
        );
        assert_eq!(result, Some(106));
    }

    #[test]
    fn test_part_two_2() {
        let result = part_two(
            "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^",
        );
        assert_eq!(result, Some(618));
    }
}

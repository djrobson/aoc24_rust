advent_of_code::solution!(21);
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum NumPad {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Zero,
    A,
}
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum DirPad {
    Up,
    Down,
    Left,
    Right,
    A,
}

fn parse_inputs(input: &str) -> Vec<Vec<NumPad>> {
    input
        .lines()
        .map(|line| {
            line.bytes()
                .map(|num| match num {
                    b'1' => NumPad::One,
                    b'2' => NumPad::Two,
                    b'3' => NumPad::Three,
                    b'4' => NumPad::Four,
                    b'5' => NumPad::Five,
                    b'6' => NumPad::Six,
                    b'7' => NumPad::Seven,
                    b'8' => NumPad::Eight,
                    b'9' => NumPad::Nine,
                    b'0' => NumPad::Zero,
                    b'A' => NumPad::A,
                    _ => panic!("Invalid input"),
                })
                .collect()
        })
        .collect()
}
/*
    +---+---+
    | ^ | A |
+---+---+---+
| < | v | > |
+---+---+---+
*/
fn traverse_dir_pad(start_pos: &DirPad, end_pos: &DirPad) -> (DirPad, String) {
    let commands: String = match (start_pos, end_pos) {
        (DirPad::Up, DirPad::Up) => String::new(),
        (DirPad::Up, DirPad::Down) => "v".to_string(),
        (DirPad::Up, DirPad::Left) => "v<".to_string(),
        (DirPad::Up, DirPad::Right) => "v>".to_string(),
        (DirPad::Up, DirPad::A) => ">".to_string(),
        (DirPad::Down, DirPad::Up) => "^".to_string(),
        (DirPad::Down, DirPad::Down) => String::new(),
        (DirPad::Down, DirPad::Left) => "<".to_string(),
        (DirPad::Down, DirPad::Right) => ">".to_string(),
        (DirPad::Down, DirPad::A) => ">^".to_string(),
        (DirPad::Left, DirPad::Up) => ">^".to_string(),
        (DirPad::Left, DirPad::Down) => ">".to_string(),
        (DirPad::Left, DirPad::Left) => String::new(),
        (DirPad::Left, DirPad::Right) => ">>".to_string(),
        (DirPad::Left, DirPad::A) => ">>^".to_string(),
        (DirPad::Right, DirPad::Up) => "<^".to_string(),
        (DirPad::Right, DirPad::Down) => "<".to_string(),
        (DirPad::Right, DirPad::Left) => "<<".to_string(),
        (DirPad::Right, DirPad::Right) => String::new(),
        (DirPad::Right, DirPad::A) => "^".to_string(),
        (DirPad::A, DirPad::Up) => "<".to_string(),
        (DirPad::A, DirPad::Down) => "<v".to_string(),
        (DirPad::A, DirPad::Left) => "v<<".to_string(),
        (DirPad::A, DirPad::Right) => "v".to_string(),
        (DirPad::A, DirPad::A) => String::new(),
    };
    (*end_pos, commands)
}

fn traverse_dir_pad_for_all(inputs: &str) -> String {
    let mut dir_pad_pos = DirPad::A;
    let mut commands = String::new();
    for dir_pad_button in inputs.bytes() {
        // traverse the num pad by moving the cursor to the desired number
        let (new_dir_pad_pos, num_commands) = traverse_dir_pad(
            &dir_pad_pos,
            &match dir_pad_button {
                b'^' => DirPad::Up,
                b'v' => DirPad::Down,
                b'<' => DirPad::Left,
                b'>' => DirPad::Right,
                b'A' => DirPad::A,
                _ => panic!("Invalid input {}", dir_pad_button as char),
            },
        );
        commands.push_str(&num_commands);
        commands.push('A');
        dir_pad_pos = new_dir_pad_pos;
    }
    commands
}
/*
+---+---+---+
| 7 | 8 | 9 |
+---+---+---+
| 4 | 5 | 6 |
+---+---+---+
| 1 | 2 | 3 |
+---+---+---+
    | 0 | A |
    +---+---+
*/
fn traverse_num_pad(start_pos: &NumPad, end_pos: &NumPad) -> (NumPad, String) {
    let mut commands: String = match (start_pos, end_pos) {
        (NumPad::One, NumPad::One) => String::new(),
        (NumPad::One, NumPad::Two) => ">".to_string(),
        (NumPad::One, NumPad::Three) => ">>".to_string(),
        (NumPad::One, NumPad::Four) => "^".to_string(),
        (NumPad::One, NumPad::Five) => ">^".to_string(),
        (NumPad::One, NumPad::Six) => ">>^".to_string(),
        (NumPad::One, NumPad::Seven) => "^^".to_string(),
        (NumPad::One, NumPad::Eight) => ">^^".to_string(),
        (NumPad::One, NumPad::Nine) => ">>^^".to_string(),
        (NumPad::One, NumPad::Zero) => ">v".to_string(),
        (NumPad::One, NumPad::A) => ">>v".to_string(),

        (NumPad::Two, NumPad::One) => "<".to_string(),
        (NumPad::Two, NumPad::Two) => String::new(),
        (NumPad::Two, NumPad::Three) => ">".to_string(),
        (NumPad::Two, NumPad::Four) => "<^".to_string(),
        (NumPad::Two, NumPad::Five) => "^".to_string(),
        (NumPad::Two, NumPad::Six) => "^>".to_string(),
        (NumPad::Two, NumPad::Seven) => "<^^".to_string(),
        (NumPad::Two, NumPad::Eight) => "^^".to_string(),
        (NumPad::Two, NumPad::Nine) => ">^^".to_string(),
        (NumPad::Two, NumPad::Zero) => "v".to_string(),
        (NumPad::Two, NumPad::A) => "v>".to_string(),

        (NumPad::Three, NumPad::One) => "<<".to_string(),
        (NumPad::Three, NumPad::Two) => "<".to_string(),
        (NumPad::Three, NumPad::Three) => String::new(),
        (NumPad::Three, NumPad::Four) => "<<^".to_string(),
        (NumPad::Three, NumPad::Five) => "<^".to_string(),
        (NumPad::Three, NumPad::Six) => "^".to_string(),
        (NumPad::Three, NumPad::Seven) => "<<^^".to_string(),
        (NumPad::Three, NumPad::Eight) => "<^^".to_string(),
        (NumPad::Three, NumPad::Nine) => "^^".to_string(),
        (NumPad::Three, NumPad::Zero) => "<v".to_string(),
        (NumPad::Three, NumPad::A) => "v".to_string(),

        (NumPad::Four, NumPad::One) => "v".to_string(),
        (NumPad::Four, NumPad::Two) => "v>".to_string(),
        (NumPad::Four, NumPad::Three) => "v>>".to_string(),
        (NumPad::Four, NumPad::Four) => String::new(),
        (NumPad::Four, NumPad::Five) => ">".to_string(),
        (NumPad::Four, NumPad::Six) => ">>".to_string(),
        (NumPad::Four, NumPad::Seven) => "^".to_string(),
        (NumPad::Four, NumPad::Eight) => ">^".to_string(),
        (NumPad::Four, NumPad::Nine) => ">>^".to_string(),
        (NumPad::Four, NumPad::Zero) => ">vv".to_string(),
        (NumPad::Four, NumPad::A) => ">>vv".to_string(),

        (NumPad::Five, NumPad::One) => "<v".to_string(),
        (NumPad::Five, NumPad::Two) => "v".to_string(),
        (NumPad::Five, NumPad::Three) => "v>".to_string(),
        (NumPad::Five, NumPad::Four) => "<".to_string(),
        (NumPad::Five, NumPad::Five) => String::new(),
        (NumPad::Five, NumPad::Six) => ">".to_string(),
        (NumPad::Five, NumPad::Seven) => "<^".to_string(),
        (NumPad::Five, NumPad::Eight) => "^".to_string(),
        (NumPad::Five, NumPad::Nine) => "^>".to_string(),
        (NumPad::Five, NumPad::Zero) => "vv".to_string(),
        (NumPad::Five, NumPad::A) => "vv>".to_string(),

        (NumPad::Six, NumPad::One) => "<<v".to_string(),
        (NumPad::Six, NumPad::Two) => "<v".to_string(),
        (NumPad::Six, NumPad::Three) => "v".to_string(),
        (NumPad::Six, NumPad::Four) => "<<".to_string(),
        (NumPad::Six, NumPad::Five) => "<".to_string(),
        (NumPad::Six, NumPad::Six) => String::new(),
        (NumPad::Six, NumPad::Seven) => "<<^".to_string(),
        (NumPad::Six, NumPad::Eight) => "<^".to_string(),
        (NumPad::Six, NumPad::Nine) => "^".to_string(),
        (NumPad::Six, NumPad::Zero) => "<vv".to_string(),
        (NumPad::Six, NumPad::A) => "vv".to_string(),

        (NumPad::Seven, NumPad::One) => "vv".to_string(),
        (NumPad::Seven, NumPad::Two) => "vv>".to_string(),
        (NumPad::Seven, NumPad::Three) => "vv>>".to_string(),
        (NumPad::Seven, NumPad::Four) => "v".to_string(),
        (NumPad::Seven, NumPad::Five) => "v>".to_string(),
        (NumPad::Seven, NumPad::Six) => "v>>".to_string(),
        (NumPad::Seven, NumPad::Seven) => String::new(),
        (NumPad::Seven, NumPad::Eight) => ">".to_string(),
        (NumPad::Seven, NumPad::Nine) => ">>".to_string(),
        (NumPad::Seven, NumPad::Zero) => ">vvv".to_string(),
        (NumPad::Seven, NumPad::A) => ">>vvv".to_string(),

        (NumPad::Eight, NumPad::One) => "<vv".to_string(),
        (NumPad::Eight, NumPad::Two) => "vv".to_string(),
        (NumPad::Eight, NumPad::Three) => "vv>".to_string(),
        (NumPad::Eight, NumPad::Four) => "<v".to_string(),
        (NumPad::Eight, NumPad::Five) => "v".to_string(),
        (NumPad::Eight, NumPad::Six) => "v>".to_string(),
        (NumPad::Eight, NumPad::Seven) => "<".to_string(),
        (NumPad::Eight, NumPad::Eight) => String::new(),
        (NumPad::Eight, NumPad::Nine) => ">".to_string(),
        (NumPad::Eight, NumPad::Zero) => "vvv".to_string(),
        (NumPad::Eight, NumPad::A) => "vvv>".to_string(),

        (NumPad::Nine, NumPad::One) => "<<vv".to_string(),
        (NumPad::Nine, NumPad::Two) => "<vv".to_string(),
        (NumPad::Nine, NumPad::Three) => "vv".to_string(),
        (NumPad::Nine, NumPad::Four) => "<<v".to_string(),
        (NumPad::Nine, NumPad::Five) => "<v".to_string(),
        (NumPad::Nine, NumPad::Six) => "v".to_string(),
        (NumPad::Nine, NumPad::Seven) => "<<".to_string(),
        (NumPad::Nine, NumPad::Eight) => "<".to_string(),
        (NumPad::Nine, NumPad::Nine) => String::new(),
        (NumPad::Nine, NumPad::Zero) => "<vvv".to_string(),
        (NumPad::Nine, NumPad::A) => "vvv".to_string(),

        (NumPad::Zero, NumPad::One) => "^<".to_string(),
        (NumPad::Zero, NumPad::Two) => "^".to_string(),
        (NumPad::Zero, NumPad::Three) => ">^".to_string(),
        (NumPad::Zero, NumPad::Four) => "^^<".to_string(),
        (NumPad::Zero, NumPad::Five) => "^^".to_string(),
        (NumPad::Zero, NumPad::Six) => ">^^".to_string(),
        (NumPad::Zero, NumPad::Seven) => "^^^<".to_string(),
        (NumPad::Zero, NumPad::Eight) => "^^^".to_string(),
        (NumPad::Zero, NumPad::Nine) => ">^^^".to_string(),
        (NumPad::Zero, NumPad::Zero) => String::new(),
        (NumPad::Zero, NumPad::A) => ">".to_string(),

        (NumPad::A, NumPad::One) => "^<<".to_string(),
        (NumPad::A, NumPad::Two) => "<^".to_string(),
        (NumPad::A, NumPad::Three) => "^".to_string(),
        (NumPad::A, NumPad::Four) => "^^<<".to_string(),
        (NumPad::A, NumPad::Five) => "<^^".to_string(),
        (NumPad::A, NumPad::Six) => "^^".to_string(),
        (NumPad::A, NumPad::Seven) => "^^^<<".to_string(),
        (NumPad::A, NumPad::Eight) => "<^^^".to_string(),
        (NumPad::A, NumPad::Nine) => "^^^".to_string(),
        (NumPad::A, NumPad::Zero) => "<".to_string(),
        (NumPad::A, NumPad::A) => String::new(),
    };
    commands.push('A');
    (*end_pos, commands)
}

fn get_num_from_numpad(num: &NumPad) -> usize {
    match num {
        NumPad::One => 1,
        NumPad::Two => 2,
        NumPad::Three => 3,
        NumPad::Four => 4,
        NumPad::Five => 5,
        NumPad::Six => 6,
        NumPad::Seven => 7,
        NumPad::Eight => 8,
        NumPad::Nine => 9,
        NumPad::Zero => 0,
        NumPad::A => 0,
    }
}

fn get_num_from_input(input: &[NumPad]) -> usize {
    let mut num = 0;
    num += get_num_from_numpad(&input[0]) * 100;
    num += get_num_from_numpad(&input[1]) * 10;
    num += get_num_from_numpad(&input[2]);
    num
}

pub fn part_one(input: &str) -> Option<usize> {
    let inputs = parse_inputs(input);
    let mut complexity = 0;
    for input in inputs {
        let mut robot1_commands = String::new();
        let mut robot2_commands = String::new();

        let mut all_numpad_commands = String::new();
        let mut all_robot1_commands = String::new();
        let mut all_robot2_commands = String::new();

        // starting at the bottom right corner
        let mut num_pad_pos = NumPad::A;
        for num_pad_button in &input {
            // traverse the num pad by moving the cursor to the desired number
            let (new_num_pad_pos, num_commands) = traverse_num_pad(&num_pad_pos, num_pad_button);
            num_pad_pos = new_num_pad_pos;
            all_numpad_commands.push_str(&num_commands);
            //println!("numpad: {:?} -> {}", num_pad_button, num_commands);

            let dir1_commands = traverse_dir_pad_for_all(&num_commands);
            robot1_commands.push_str(&dir1_commands);
            all_robot1_commands.push_str(&dir1_commands);
            //println!("robot1: {}", dir1_commands);

            let dir2_commands = traverse_dir_pad_for_all(&dir1_commands);
            robot2_commands.push_str(&dir2_commands);
            all_robot2_commands.push_str(&dir2_commands);
            //println!("robot2: {}", dir2_commands);
        }

        //println!("all robot 2 commands: {}", all_robot2_commands);
        //println!("all robot 1 commands: {}", all_robot1_commands);
        //println!("all numpad commands: {}", all_numpad_commands);
        complexity += all_robot2_commands.len() * get_num_from_input(&input);
    }
    Some(complexity)
}

fn robot_presses(
    presses: &str,
    robot_count: usize,
    cache: &mut HashMap<(String, usize), usize>,
) -> usize {
    let input_count;

    if let Some(count) = cache.get(&(presses.to_owned(), robot_count)) {
        // we have already computed the count for this input
        input_count = *count;
    } else {
        // break the input input parts and add the cost of all parts
        let dir_pushes = traverse_dir_pad_for_all(presses);
        if robot_count == 0 {
            input_count = dir_pushes.len();
        } else {
            // split input by A and recurse on each part
            input_count = dir_pushes.split_inclusive('A').map(|part| {
                // compute count for each part at the next level
                robot_presses(&part, robot_count - 1, cache)
            }).sum();
        }
        cache.insert((presses.to_owned(), robot_count), input_count);
    }
    input_count
}

pub fn part_two(input: &str) -> Option<usize> {
    part_two_ex(input, 25)
}

pub fn part_two_ex(input: &str, robot_count: usize) -> Option<usize> {
    let inputs = parse_inputs(input);
    let mut total_complexity = 0;
    let mut push_cache: HashMap<(String, usize), usize> = HashMap::new();
    let mut num_pad_pos = NumPad::A;
    for input in inputs {
        for num_pad_button in &input {
            // for each number on the number, move the robot and remember where it is
            let (new_num_pad_pos, num_commands) = traverse_num_pad(&num_pad_pos, num_pad_button);
            num_pad_pos = new_num_pad_pos; // update the position of the numpad robot

            let input_count = robot_presses(&num_commands, robot_count-1, &mut push_cache);

            total_complexity += input_count * get_num_from_input(&input);
        }
    }
    Some(total_complexity)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(175396398527088));
    }

    #[test]
    fn test_part_two_ex() {
        let result = part_two_ex(&advent_of_code::template::read_file("examples", DAY), 2);
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_two_029() {
        let result = part_two_ex("029A", 2);
        assert_eq!(
            result,
            Some("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len() * 29)
        );
    }
    #[test]
    fn test_part_two_980() {
        let result = part_two_ex("980A", 2);
        assert_eq!(
            result,
            Some("<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len() * 980)
        );
    }
    #[test]
    fn test_part_two_179() {
        let result = part_two_ex("179A", 2);
        assert_eq!(
            result,
            Some(
                "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len() * 179
            )
        );
    }
    #[test]
    fn test_part_two_456() {
        let result = part_two_ex("456A", 2);
        assert_eq!(
            result,
            Some("<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".len() * 456)
        );
    }
    #[test]
    fn test_part_two_379() {
        let result = part_two_ex("379A", 2);
        assert_eq!(
            result,
            Some("<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len() * 379)
        );
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(126384));
    }

    #[test]
    fn test_part_one_029() {
        let result = part_one("029A");
        assert_eq!(
            result,
            Some("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len() * 29)
        );
    }
    #[test]
    fn test_part_one_980() {
        let result = part_one("980A");
        assert_eq!(
            result,
            Some("<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A".len() * 980)
        );
    }
    #[test]
    fn test_part_one_179() {
        let result = part_one("179A");
        assert_eq!(
            result,
            Some(
                "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len() * 179
            )
        );
    }
    #[test]
    fn test_part_one_456() {
        let result = part_one("456A");
        assert_eq!(
            result,
            Some("<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A".len() * 456)
        );
    }
    #[test]
    fn test_part_one_379() {
        let result = part_one("379A");
        assert_eq!(
            result,
            Some("<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len() * 379)
        );
    }
}

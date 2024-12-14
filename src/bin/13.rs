advent_of_code::solution!(13);

#[derive(Debug)]
struct MachineDetails {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
}

fn parse_input(input: &str, offset: i64) -> Vec<MachineDetails> {
    // parse input like this, repeat until you don't find a blank line after
    // Button A: X+94, Y+34
    // Button B: X+22, Y+67
    // Prize: X=8400, Y=5400
    //

    let mut machines = Vec::new();
    let mut input_lines = input.lines();
    loop {
        let a = input_lines.next().unwrap();
        let a_x = a[12..14].parse::<i64>().unwrap();
        let a_y = a[18..20].parse::<i64>().unwrap();

        let b = input_lines.next().unwrap();
        let b_x = b[12..14].parse::<i64>().unwrap();
        let b_y = b[18..20].parse::<i64>().unwrap();

        let prize = input_lines.next().unwrap();
        // parse input like "Prize: X=18641, Y=10279" annd "Prize: X=7870, Y=6450"
        // to get the x and y coordinates
        let prize_x = prize.split("X=").collect::<Vec<&str>>()[1]
            .split(", ")
            .collect::<Vec<&str>>()[0]
            .parse::<i64>()
            .unwrap()
            + offset;
        let prize_y = prize.split("Y=").collect::<Vec<&str>>()[1]
            .parse::<i64>()
            .unwrap()
            + offset;

        let mut machine = MachineDetails {
            a_x,
            a_y,
            b_x,
            b_y,
            prize_x,
            prize_y,
        };

        // parse the input and assign to machine
        machines.push(machine);

        if input_lines.next().is_none() {
            break;
        }
    }
    machines
}

pub fn part_one(input: &str) -> Option<i64> {
    let machines = parse_input(input, 0);
    let mut total_cost = 0;

    for machine in machines {
        let mut machine_options = Vec::new();
        // search for all options in the first 100 presses
        for a_press in 1..101 {
            let a_x = machine.a_x * a_press;
            let a_y = machine.a_y * a_press;
            if a_x > machine.prize_x || a_y > machine.prize_y {
                // we're monotonic and missed the goal
                break;
            }
            for b_press in 1..101 {
                let ab_x = machine.b_x * b_press + a_x;
                let ab_y = machine.b_y * b_press + a_y;

                if ab_x > machine.prize_x || ab_y > machine.prize_y {
                    // we're monotonic and missed the goal
                    break;
                }

                if ab_x == machine.prize_x && ab_y == machine.prize_y {
                    machine_options.push(((a_press * 3) + b_press, a_press, b_press));
                }
            }
        }
        // sort machine options by the first element of the tuple
        if machine_options.len() > 0 {
            machine_options.sort_by(|a, b| a.0.cmp(&b.0));
            //println!("{:?} had cost {}", machine_options, machine_options[0].0);
            total_cost += machine_options[0].0;
        } else {
            //println!("No options found for {:?}", machine);
        }
    }
    Some(total_cost)
}

fn determinant(matrix: [[i64; 2]; 2]) -> i64 {
    matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]
}

fn solve_cramers_rule(a_x: i64, a_y: i64, b_x: i64, b_y: i64, prize_x: i64, prize_y: i64) -> Option<(i64, i64)> {

    let det = determinant([[a_x, b_x], [a_y, b_y]]);
    if det == 0 {
        return None;
    }
    let det_x = determinant([[prize_x, b_x], [prize_y, b_y]]);
    let det_y = determinant([[a_x, prize_x], [a_y, prize_y]]);
    if det_x % det == 0 && det_y % det == 0 {
        return Some((det_x / det, det_y / det))
    }
    None
}

fn solve_cramers_rule_by_machine_detail(machine: &MachineDetails) -> Option<(i64, i64)> {
    solve_cramers_rule(machine.a_x, machine.a_y, machine.b_x, machine.b_y, machine.prize_x, machine.prize_y)
}

pub fn part_two(input: &str) -> Option<i64> {
    let machines = parse_input(input, 10000000000000);
    let mut total_cost = 0;

    for machine in machines {
        if let Some(solution) = solve_cramers_rule_by_machine_detail(&machine){
            //println!("Machine {:?} solutions: A: {}, B: {}", machine, a, b);
            total_cost += solution.0 * 3 + solution.1;
        } else {
            //println!("No solutions found for {:?}", machine);
        }
    }
    Some(total_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(480));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

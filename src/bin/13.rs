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

fn extended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = extended_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

fn solve_diophantine(machine: &MachineDetails) -> Option<Vec<i64>> {
    // Solve the first equation for A
    // A = (machine.prize_x - B * machine.b_x) / machine.a_x
    // Substitute into the second equation
    // ((machine.prize_x - B * machine.b_x) / machine.a_x) * machine.b_x + B * machine.b_y = machine.prize_y
    // Simplify to get a single-variable equation in terms of B
    let a = machine.b_x * machine.b_x - machine.a_x * machine.b_y;
    let b = machine.a_x * machine.prize_y - machine.b_x * machine.prize_x;
    let c = machine.a_x * machine.b_y;

    let (g, x, y) = extended_gcd(a, c);

    if b % g != 0 {
        return None; // No solution exists
    }

    let b0 = x * (b / g);
    let a0 = (machine.prize_x - b0 * machine.b_x) / machine.a_x;

    // General solution: A = a0 + k * (c / g), B = b0 - k * (a / g)
    let mut solutions = Vec::new();
    let k_min = -500; // Arbitrary range for k
    let k_max = 500; // Arbitrary range for k

    for k in k_min..=k_max {
        let a = a0 + k * (c / g);
        let b = b0 - k * (a / g);
        if a < 0 || b < 0 {
            continue;
        }
        solutions.push(a * 3 + b);
    }
    Some(solutions)
}

pub fn part_two(input: &str) -> Option<i64> {
    let machines = parse_input(input, 10000000000000);
    let mut total_cost = 0;

    for machine in machines {
        match solve_diophantine(&machine) {
            Some(solutions) => {
                let mut lowest_solution = i64::MAX;
                for solution in &solutions {
                    if lowest_solution > *solution {
                        lowest_solution = *solution;
                    }
                }
                //println!("Machine {:?} solutions: A: {}, B: {}", machine, a, b);
                total_cost += lowest_solution;
            }
            None => println!("No solution found"),
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

advent_of_code::solution!(13);

#[derive(Debug)]
struct MachineDetails {
    a_x: u32,
    a_y: u32,
    b_x: u32,
    b_y: u32,
    prize_x: u32,
    prize_y: u32,
}

fn parse_input(input: &str) -> Vec<MachineDetails> {
    // parse input like this, repeat until you don't find a blank line after
    // Button A: X+94, Y+34
    // Button B: X+22, Y+67
    // Prize: X=8400, Y=5400
    //

    let mut machines = Vec::new();
    let mut input_lines = input.lines();
    loop {
        let a = input_lines.next().unwrap();
        let a_x = a[12..14].parse::<u32>().unwrap();
        let a_y = a[18..20].parse::<u32>().unwrap();

        let b = input_lines.next().unwrap();
        let b_x = b[12..14].parse::<u32>().unwrap();
        let b_y = b[18..20].parse::<u32>().unwrap();

        let prize = input_lines.next().unwrap();
        // parse input like "Prize: X=18641, Y=10279" annd "Prize: X=7870, Y=6450"
        // to get the x and y coordinates
        let prize_x = prize.split("X=").collect::<Vec<&str>>()[1]
            .split(", ")
            .collect::<Vec<&str>>()[0]
            .parse::<u32>()
            .unwrap();
        let prize_y = prize.split("Y=").collect::<Vec<&str>>()[1]
            .parse::<u32>()
            .unwrap();

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

pub fn part_one(input: &str) -> Option<u32> {
    let machines = parse_input(input);
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

pub fn part_two(input: &str) -> Option<u32> {
    None
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

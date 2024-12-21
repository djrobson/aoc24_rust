advent_of_code::solution!(7);

fn parse_input(input: &str) -> Vec<(usize, Vec<usize>)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(':');
            let total: usize = parts.next().unwrap().parse().unwrap();
            let operands = parts
                .next()
                .unwrap()
                .split(' ')
                .skip(1) // Skip the first space
                .map(|s| s.parse().unwrap())
                .collect();
            (total, operands)
        })
        .collect()
}

fn search_operators1(total: usize, operands: Vec<usize>) -> bool {
    if operands.len() == 1 {
        return operands[0] == total;
    }
    if operands[0] > total {
        return false;
    }
    let mut add_operands = Vec::new();
    add_operands.push(operands[0] + operands[1]);
    add_operands.extend(&operands[2..]);

    let mut mul_operands = Vec::new();
    mul_operands.push(operands[0] * operands[1]);
    mul_operands.extend(&operands[2..]);
    search_operators1(total, add_operands) || search_operators1(total, mul_operands)
}

pub fn part_one(input: &str) -> Option<usize> {
    let calibrations = parse_input(input);
    let mut total = 0;
    for (cal_total, operands) in calibrations {
        if search_operators1(cal_total, operands) {
            total += cal_total;
        }
    }
    Some(total)
}

fn search_operators2(total: usize, operands: Vec<usize>) -> bool {
    if operands.len() == 1 {
        return operands[0] == total;
    }
    if operands[0] > total {
        return false;
    }
    let mut add_operands = Vec::new();
    add_operands.push(operands[0] + operands[1]);
    add_operands.extend(&operands[2..]);

    let mut mul_operands = Vec::new();
    mul_operands.push(operands[0] * operands[1]);
    mul_operands.extend(&operands[2..]);

    let mut cat_operands: Vec<usize> = Vec::new();
    let new_first = format!("{}{}", operands[0], operands[1]).parse().unwrap();
    //dbg!(new_first);
    cat_operands.push(new_first);
    cat_operands.extend(&operands[2..]);
    search_operators2(total, add_operands)
        || search_operators2(total, mul_operands)
        || search_operators2(total, cat_operands)
}

pub fn part_two(input: &str) -> Option<usize> {
    let calibrations = parse_input(input);
    let mut total = 0;
    for (cal_total, operands) in calibrations {
        if search_operators2(cal_total, operands) {
            total += cal_total;
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result: Option<usize> = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}

advent_of_code::solution!(11);

fn get_num_digits(num: &usize) -> usize {
    let mut num_digits = 0;
    let mut num = *num;
    while num > 0 {
        num_digits += 1;
        num /= 10;
    }
    num_digits
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut this_input: Vec<usize> = input
        .split_ascii_whitespace()
        .map(|num| num.parse().expect("Invalid number"))
        .collect::<Vec<_>>();
    let mut output_len: usize = 0;

    for _ in 0..25 {
        let mut new_input = Vec::new();
        for num in this_input.iter() {
            if *num == 0 {
                new_input.push(1);
            } else if get_num_digits(num) % 2 == 0 {
                let printed_num = num.to_string();
                let half = printed_num.len() / 2;

                new_input.push(printed_num[..half].parse().unwrap());
                new_input.push(printed_num[half..].parse().unwrap());
            } else {
                new_input.push(num * 2024);
            }
        }
        this_input = new_input;
        output_len = this_input.len();
    }
    Some(output_len as u32)
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
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

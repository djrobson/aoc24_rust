advent_of_code::solution!(22);

use itertools::Itertools;

fn get_next_prn(current: u32) -> u32 {
    /* Calculate the result of multiplying the secret number by 64.
    Then, mix this result into the secret number.
    Finally, prune the secret number.
    Calculate the result of dividing the secret number by 32.
    Round the result down to the nearest integer.
    Then, mix this result into the secret number.
    Finally, prune the secret number.
    Calculate the result of multiplying the secret number by 2048.
    Then, mix this result into the secret number.
    Finally, prune the secret number. */
    let step1 = ((current << 6) ^ current) & 0xffffff;
    let step2 = ((step1 >> 5 ) ^ step1) & 0xffffff;
    let step3 = ((step2 << 11) ^ step2) & 0xffffff;
    step3
}


pub fn part_one(input: &str) -> Option<usize> {
    let input_nums: Vec<u32> = input.lines().map(
        |line| line.parse().expect("input wasn't a number"))
        .collect();
    let mut total_results: usize = 0;
    for num in input_nums.iter() {
        let mut result = *num;
        for _ in 0 ..2000 {
            result = get_next_prn(result);
        }
        total_results += result as usize;
    }
    Some(total_results)
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
        assert_eq!(result, Some(37327623));
    }
    #[test]
    fn test_part_one_1() {
        let result = get_next_prn(123);
        assert_eq!(result, 15887950);
    }
    /*If the secret number is 42 and you were to mix 15 into the secret number, the secret number would become 37. */
    #[test]
    fn test_part_one_mix() {
        let result = 42 ^ 15;
        assert_eq!(result, 37);
    }

    /*If the secret number is 100000000 and you were to prune the secret number, the secret number would become 16113920 */
    #[test]
    fn test_part_one_prune() {
        let result = 100000000 & 0xffffff;
        assert_eq!(result, 16113920);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

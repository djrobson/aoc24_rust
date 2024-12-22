advent_of_code::solution!(22);

use std::collections::HashMap;
use rayon::prelude::*;

use itertools::Itertools;

fn get_next_prn(current: i32) -> i32 {
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
    let step2 = ((step1 >> 5) ^ step1) & 0xffffff;
    ((step2 << 11) ^ step2) & 0xffffff
}

pub fn part_one(input: &str) -> Option<usize> {
    let input_nums: Vec<i32> = input
        .lines()
        .map(|line| line.parse().expect("input wasn't a number"))
        .collect();
    let mut total_results: usize = 0;
    for num in input_nums.iter() {
        let mut result = *num;
        for _ in 0..2000 {
            result = get_next_prn(result);
        }
        total_results += result as usize;
    }
    Some(total_results)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input_nums: Vec<i32> = input
        .lines()
        .map(|line| line.parse().expect("input wasn't a number"))
        .collect();

    let mut sequence_for_input: Vec<Vec<i32>> = Vec::new();
    for (seq, num) in input_nums.iter().enumerate() {
        sequence_for_input.insert(seq, vec![*num % 10]);
        let mut result = *num;
        for _ in 0..2000 {
            result = get_next_prn(result);
            sequence_for_input[seq].push(result % 10);
        }
    }
    let mut pattern_for_sequence: Vec<HashMap<(i32, i32, i32, i32), i32>> = Vec::new();
    for sequence in sequence_for_input.iter() {
        let mut results_for_patterns: HashMap<(i32, i32, i32, i32), i32> = HashMap::new();
        sequence
            .iter()
            .tuple_windows()
            .for_each(|(a, b, c, d, e)| {
                let key: (i32, i32, i32, i32) = (b - a, c - b, d - c, e - d);
                results_for_patterns.entry(key).or_insert(*e);
            });
        pattern_for_sequence.push(results_for_patterns);
    }

    let best_total = (-9..=9).collect::<Vec<_>>().par_iter().map(|a| {
        (-9..=9).map(|b| {
            (-9..=9).map(|c| {
                (-9..=9).map(|d| {
                    let key: (i32, i32, i32, i32) = (*a, b, c, d);
                    let mut pattern_total = 0;
                    for pattern in &pattern_for_sequence {
                        if pattern.contains_key(&key) {
                            pattern_total += pattern.get(&key).unwrap();
                        }
                    }
                    pattern_total
                }).max().unwrap()
            }).max().unwrap()
        }).max().unwrap()
    }).max().unwrap();

    Some(best_total as usize)
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
        assert_eq!(result, Some(24));
    }
}

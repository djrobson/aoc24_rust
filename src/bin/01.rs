advent_of_code::solution!(1);
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    // read lines from a file, split the line into 2 integers, push the first into list2 and the second into list2
    for line in input.lines() {
        let mut split = line.split("   ");
        let first = split.next().unwrap().parse::<u32>().unwrap();
        let second = split.next().unwrap().parse::<u32>().unwrap();
        list1.push(first);
        list2.push(second);
    }

    list1.sort();
    list2.sort();
    let sum: u32 = list1
        .iter()
        .zip(list2.iter())
        .map(|(a, b)| a.abs_diff(*b))
        .sum();
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut list1 = Vec::new();
    let mut hash2: HashMap<u32, u32> = HashMap::new();

    // read lines from a file, split the line into 2 integers, push the first into list2 and the second into list2
    for line in input.lines() {
        let mut split = line.split("   ");
        let first = split.next().unwrap().parse::<u32>().unwrap();
        let second = split.next().unwrap().parse::<u32>().unwrap();

        list1.push(first);
        *hash2.entry(second).or_insert(0) += 1
    }
    let sum = list1.iter().map(|n| n * hash2.get(n).unwrap_or(&0)).sum();
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

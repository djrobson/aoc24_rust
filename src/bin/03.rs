advent_of_code::solution!(3);
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    // parse all lines of input]
    // look for the exaxt pattern "mul([0-9]{1,3},[0-9]{1,3})" and extract the numbers
    // multiply the numbers and add them to a sum
    // return the sum
    let mut sum = 0;
    for line in input.lines() {
        let mut iter = line.match_indices("mul");
        while let Some((offset, _)) = iter.next() {
            let start = offset + 4;
            let end = line[start..].find(')').unwrap();
            if let Some((a, b)) = line[start..start + end]
                .split(',')
                .map(|x| x.parse::<u32>())
                .collect_tuple()
            {
                sum += a.unwrap_or(0) * b.unwrap_or(0);
            }
        }
    }
    Some(sum)
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
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

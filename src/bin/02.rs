advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let mut valid = 0;
    for line in input.lines() {
        let report: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        //reports.push(report);
        if report.len() < 2 {
            continue;
        }

        let mut prev = report[0];
        let mut count = 1;
        // look for ascending
        if report[0] < report[1] {
            for rec in report.iter().skip(1) {
                if prev >= *rec || *rec - prev > 3 {
                    count = 0;
                    break;
                }
                prev = *rec;
            }
            valid += count;
        }
        // look for decending order
        else if report[0] > report[1] {
            for rec in report.iter().skip(1) {
                if prev <= *rec || prev - *rec > 3 {
                    count = 0;
                    break;
                }
                prev = *rec;
            }
            valid += count;
        }
        // skip if elements are equal
    }
    Some(valid)
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
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

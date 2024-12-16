advent_of_code::solution!(2);

pub fn is_valid(report: &[u32]) -> bool {
    if report.len() < 2 {
        return false;
    }
    let mut prev = report[0];
    // look for ascending
    #[allow(clippy::comparison_chain)]
    if report[0] < report[1] {
        for rec in report.iter().skip(1) {
            if prev >= *rec || *rec - prev > 3 {
                return false;
            }
            prev = *rec;
        }
        return true;
    }
    // look for decending order
    else if report[0] > report[1] {
        for rec in report.iter().skip(1) {
            if prev <= *rec || prev - *rec > 3 {
                return false;
            }
            prev = *rec;
        }
        return true;
    }
    // skip if elements are equal
    false
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut valid = 0;
    for line in input.lines() {
        let report: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        // filter elements in report according to is_valid()
        if is_valid(&report) {
            valid += 1;
        }
    }
    Some(valid)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut valid = 0;
    for line in input.lines() {
        let report: Vec<u32> = line
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let mut any_valid = false;
        for i in 0..report.len() {
            let mut report_drop = report.clone();
            report_drop.remove(i);
            if is_valid(&report_drop) {
                any_valid = true;
                break;
            }
        }
        if any_valid {
            valid += 1;
        }
    }
    Some(valid)
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
        assert_eq!(result, Some(4));
    }
}

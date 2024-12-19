advent_of_code::solution!(19);

fn parse_input(input: &str) -> (Vec<&str>, Vec<&str>) {
    let top_bottom = input.split("\n\n").collect::<Vec<&str>>();

    // split r, wr, b, g, bwu, rb, gb, br into towels, remove the ',' and ' '
    let towels: Vec<&str> = top_bottom[0]
        .split_ascii_whitespace()
        .map(|towel| towel.trim_end_matches(','))
        .collect();
    let messages: Vec<&str> = top_bottom[1].lines().collect();
    (towels, messages)
}

fn check_message(towels: &Vec<&str>, message: &str) -> bool {
    if message.is_empty() {
        return true;
    }
    towels.iter().any(|towel| {
        if message.starts_with(towel) {
            return check_message(towels, &message[towel.len()..]);
        }
        false
    })
}

pub fn part_one(input: &str) -> Option<u32> {
    let (towels, messages) = parse_input(input);

    let count = messages
        .iter()
        .filter(|message| check_message(&towels, message))
        .count();

    if count > 0 {
        Some(count as u32)
    } else {
        None
    }
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
        assert_eq!(result, Some(6));
    }
    #[test]
    fn test_part_one_1() {
        let result = part_one(
            "r, wr, b, g, bwu, rb, gb, br

brwrr",
        );
        assert_eq!(result, Some(1));
    }
    #[test]
    fn test_part_one_2() {
        let result = part_one(
            "r, wr, b, g, bwu, rb, gb, br

ubwu",
        );
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

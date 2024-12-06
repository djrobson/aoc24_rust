advent_of_code::solution!(3);
use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    // parse all lines of input]
    // look for the exaxt pattern "mul([0-9]{1,3},[0-9]{1,3})" and extract the numbers
    // multiply the numbers and add them to a sum
    // return the sum
    let mut sum = 0;
    for line in input.lines() {
        let mut iter = line.match_indices("mul(");
        while let Some((offset, _)) = iter.next() {
            let start = offset + 4;
            let end = line[start..].find(')').unwrap();
            if let Some((a, b)) = line[start..start + end]
                .split(',')
                .map(|x| x.parse::<u32>())
                .collect_tuple()
            {
                match (a, b) {
                    (Ok(a), Ok(b)) => sum += a * b,
                    _ => (),
                }
            }
        }
    }
    Some(sum)
}

fn get_mul_instructions(input: &str) -> Vec<(usize, u32)> {
    // collect all the valid mul(x,y) pairs and store them in a vector
    let mut multiplies: Vec<(usize, u32)> = Vec::new();
    let mut iter = input.match_indices("mul(");
    while let Some((offset, _)) = iter.next() {
        let start = offset + 4;
        let end = input[start..].find(')').unwrap(); // TODO: handle the case where ')' is not found
        let values = &input[start..start + end];
        if let Some((a, b)) = values.split(',').map(|x| x.parse::<u32>()).collect_tuple() {
            match (a, b) {
                (Ok(a), Ok(b)) => {
                    //println!("Good: offset: {}, a: {}, b: {}", offset, a, b);
                    multiplies.push((offset, a * b));
                }
                _ => {
                    //println!("#### failed to parse: {}", values);
                }
            }
        }
    }
    multiplies
}

fn get_exclusion_ranges(input: &str) -> Vec<(usize, usize)> {
    // scan through input for "don't()", record the index, then scan through for "do()" and record the index
    // repeat until all the input is consumed
    let mut exclude_ranges: Vec<(usize, usize)> = Vec::new();
    let mut toggle_offset = 0;
    let mut iter;
    let mut exclude_range_start;
    let mut exclude_range_end;
    while toggle_offset < input.len() {
        iter = input[toggle_offset..].match_indices("don't()");
        if let Some((offset, _)) = iter.next() {
            toggle_offset += offset + 7;
            exclude_range_start = toggle_offset;
        } else {
            break;
        }

        iter = input[toggle_offset..].match_indices("do()");
        if let Some((offset, _)) = iter.next() {
            toggle_offset += offset + 4;
            exclude_range_end = toggle_offset;
        } else {
            // we start a don't() and never say another do, so it excludes until the end of the input
            toggle_offset = input.len();
            exclude_range_end = toggle_offset;
        }
        exclude_ranges.push((exclude_range_start, exclude_range_end));
    }
    //dbg!(&exclude_ranges);
    exclude_ranges
}

pub fn part_two(input: &str) -> Option<u32> {
    let multiplies = get_mul_instructions(input);
    let exclude_ranges = get_exclusion_ranges(input);

    let mut sum = 0;
    let mut excl_range_index = 0;
    let mut cur_excl_range = exclude_ranges[excl_range_index];
    excl_range_index += 1;
    for (offset, value) in multiplies {
        // skip if the current offset is within the current exclude range
        if offset > cur_excl_range.0 && offset < cur_excl_range.1 {
            /*println!(
                "skipping offset: {} because {} {}",
                offset, cur_excl_range.0, cur_excl_range.1
            );*/
            continue;
        }

        // if the current offset is greater than the current exclude range, move to the next exclude range
        while offset > cur_excl_range.1 {
            /*println!(
                "advancing exclusion range: offset {} > exclude end {}",
                offset, cur_excl_range.1
            );*/
            if excl_range_index < exclude_ranges.len() {
                cur_excl_range = exclude_ranges[excl_range_index];
                excl_range_index += 1;
            } else {
                break;
            }
        }

        if offset < cur_excl_range.0 || offset > cur_excl_range.1 {
            /*println!(
                "including offset: {}, exclude start: {}, end: {}, value: {}",
                offset, cur_excl_range.0, cur_excl_range.1, value
            );*/
            sum += value;
        } else {
            /*println!(
                "excluding offset: {}, exclude start: {}, end: {}, value: {}",
                offset, cur_excl_range.0, cur_excl_range.1, value
            );*/
        }
    }
    Some(sum)
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
        let result =
            part_two("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, Some(48));
    }
}

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

fn build_list_from_seed(seed: usize, depth: usize) -> Vec<usize> {
    let mut this_input = vec![seed];
    for _ in 0..depth {
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
    }
    this_input
}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct Node {
    value: usize,
    depth: usize,
}

impl Node {
    fn new(value: usize, depth: usize) -> Self {
        Self { value, depth }
    }
}

fn count_nodes_at_depth(
    node: &Node,
    count_cache: &mut std::collections::HashMap<Node, usize>,
) -> usize {
    if node.depth == 0 {
        return 1;
    }

    if let Some(count) = count_cache.get(node) {
        return *count;
    }

    let mut count = 0;
    if node.value == 0 {
        count += count_nodes_at_depth(&Node::new(1, node.depth - 1), count_cache);
    } else if get_num_digits(&node.value) % 2 == 0 {
        let printed_num = node.value.to_string();
        let half = printed_num.len() / 2;

        count += count_nodes_at_depth(
            &Node::new(printed_num[..half].parse().unwrap(), node.depth - 1),
            count_cache,
        );
        count += count_nodes_at_depth(
            &Node::new(printed_num[half..].parse().unwrap(), node.depth - 1),
            count_cache,
        );
    } else {
        count += count_nodes_at_depth(&Node::new(node.value * 2024, node.depth - 1), count_cache);
    }

    count_cache.insert(*node, count);
    count
}

fn part_two_with_depth(input: &str, depth: usize) -> Option<usize> {
    let this_input: Vec<Node> = input
        .split_ascii_whitespace()
        .map(|num| Node {
            value: num.parse().expect("Invalid number"),
            depth,
        })
        .collect::<Vec<_>>();

    let mut output_len: usize = 0;
    let mut count_cache = std::collections::HashMap::new();
    for node in this_input.iter() {
        output_len += count_nodes_at_depth(node, &mut count_cache);
    }

    Some(output_len)
}

pub fn part_two(input: &str) -> Option<usize> {
    let max_depth = 75;
    return part_two_with_depth(input, max_depth);
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
    fn test_part_two_with_depth() {
        let result = part_two_with_depth(&advent_of_code::template::read_file("examples", DAY), 25);
        assert_eq!(result, Some(55312));
    }
}

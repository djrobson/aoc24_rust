advent_of_code::solution!(5);

use std::collections::HashMap;

fn parse_input1(input: &str) -> (HashMap<u8, Vec<u8>>, HashMap<u8, Vec<u8>>, Vec<Vec<u8>>) {
    let mut pre_rules: HashMap<u8, Vec<u8>> = HashMap::new();
    let mut post_rules: HashMap<u8, Vec<u8>> = HashMap::new();

    /* parse input like:
        53|13

        75,47,61,53,29
        splitting at the blank line
    */
    let mut rules_and_orders = input.split("\n\n");
    let rules = rules_and_orders.next().unwrap();

    for line in rules.lines() {
        let mut parts = line.split("|");
        let pre = parts.next().unwrap().parse().unwrap();
        let post = parts.next().unwrap().parse().unwrap();
        if pre_rules.contains_key(&post) {
            pre_rules.get_mut(&post).unwrap().push(pre);
        } else {
            pre_rules.insert(post, vec![pre]);
        }
        if post_rules.contains_key(&pre) {
            post_rules.get_mut(&pre).unwrap().push(post);
        } else {
            post_rules.insert(pre, vec![post]);
        }
    }

    let orders = rules_and_orders
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(",").map(|num| num.parse().unwrap()).collect())
        .collect();

    //dbg!(&pre_rules);
    //dbg!(&post_rules);
    //dbg!(&orders);
    (pre_rules, post_rules, orders)
}

pub fn part_one(input: &str) -> Option<u32> {
    let (pre_rules, post_rules, orders) = parse_input1(input);
    let mut result: u32 = 0;
    for order in orders {
        let mut valid = true;
        // for each value in order
        // if the value is in pre_rules,
        //  then make sure all the values in pre_rules are after the value in order
        // if the value is in post_rules,
        //  then make sure all the values in post_rules are before the value in order
        for i in 0..order.len() {
            let value = order[i];
            if pre_rules.contains_key(&value) {
                let my_rules = pre_rules.get(&value).unwrap();
                for rule in my_rules {
                    if let Some(pos) = order.iter().position(|&x| x == *rule) {
                        if pos > i {
                            valid = false;
                            break;
                        }
                    }
                }
            }
            if post_rules.contains_key(&value) {
                let my_rules = post_rules.get(&value).unwrap();
                for rule in my_rules {
                    if let Some(pos) = order.iter().position(|&x| x == *rule) {
                        if pos < i {
                            valid = false;
                            break;
                        }
                    }
                }
            }
        }
        if valid {
            let middle_value = order[order.len() / 2];
            //println!("middle value is: {}, {:?}", middle_value, order);
            result += middle_value as u32;
        }
    }
    Some(result)
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
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

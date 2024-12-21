advent_of_code::solution!(5);

use std::collections::HashMap;

struct Rules {
    pre_rules: HashMap<u8, Vec<u8>>,
    post_rules: HashMap<u8, Vec<u8>>,
    orders: Vec<Vec<u8>>,
}

fn parse_input1(input: &str) -> Rules {
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
        let mut parts = line.split('|');
        let pre = parts.next().unwrap().parse().unwrap();
        let post = parts.next().unwrap().parse().unwrap();
        if let std::collections::hash_map::Entry::Vacant(e) = pre_rules.entry(post) {
            e.insert(vec![pre]);
        } else {
            pre_rules.get_mut(&post).unwrap().push(pre);
        }
        if let std::collections::hash_map::Entry::Vacant(e) = post_rules.entry(pre) {
            e.insert(vec![post]);
        } else {
            post_rules.get_mut(&pre).unwrap().push(post);
        }
    }

    let orders = rules_and_orders
        .next()
        .unwrap()
        .lines()
        .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
        .collect();

    //dbg!(&pre_rules);
    //dbg!(&post_rules);
    //dbg!(&orders);
    Rules {
        pre_rules,
        post_rules,
        orders,
    }
}

fn split_valid_orders(
    pre_rules: &HashMap<u8, Vec<u8>>,
    post_rules: &HashMap<u8, Vec<u8>>,
    orders: &[Vec<u8>],
) -> (Vec<usize>, Vec<usize>) {
    let mut valid_orders = Vec::new();
    let mut invalid_orders = Vec::new();
    for i in 0..orders.len() {
        let order = orders.get(i).unwrap();
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
            valid_orders.push(i);
        } else {
            invalid_orders.push(i);
        }
    }
    (valid_orders, invalid_orders)
}

pub fn part_one(input: &str) -> Option<u32> {
    let rules = parse_input1(input);
    let mut result: u32 = 0;
    let (valid_orders, _) = split_valid_orders(&rules.pre_rules, &rules.post_rules, &rules.orders);
    for valid_order in valid_orders {
        let order = rules.orders.get(valid_order).unwrap();
        let middle_value = order[order.len() / 2];
        //println!("middle value is: {}, {:?}", middle_value, order);
        result += middle_value as u32;
    }

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let rules = parse_input1(input);
    let mut result: u32 = 0;
    let (_, invalid_orders) =
        split_valid_orders(&rules.pre_rules, &rules.post_rules, &rules.orders);

    // TODO: consider sorting all numbers 0-99 once then re-sorting each list as a lookup table in linear time
    for invalid_order in invalid_orders {
        let order = rules.orders.get(invalid_order).unwrap();
        // sort order where
        //  if the value is a key in pre_rules, then this value comes before every value in that pre-rules key
        //  if the value is a key in post_rules, then this value comes after every value in that pre-rules key
        let mut order = order.clone();
        let mut changed = true;
        while changed {
            changed = false;
            for i in 0..order.len() {
                let value = order[i];
                if rules.pre_rules.contains_key(&value) {
                    let my_rules = rules.pre_rules.get(&value).unwrap();
                    for rule in my_rules {
                        if let Some(pos) = order.iter().position(|&x| x == *rule) {
                            if pos > i {
                                order.swap(i, pos);
                                changed = true;
                            }
                        }
                    }
                }
                if rules.post_rules.contains_key(&value) {
                    let my_rules = rules.post_rules.get(&value).unwrap();
                    for rule in my_rules {
                        if let Some(pos) = order.iter().position(|&x| x == *rule) {
                            if pos < i {
                                order.swap(i, pos);
                                changed = true;
                            }
                        }
                    }
                }
            }
        }

        let middle_value = order[order.len() / 2];
        //println!("middle value is: {}, {:?}", middle_value, order);
        result += middle_value as u32;
    }
    Some(result)
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
        assert_eq!(result, Some(123));
    }
}

advent_of_code::solution!(25);

fn parse_input(input: &str) -> (Vec<(u8, u8, u8, u8, u8)>, Vec<(u8, u8, u8, u8, u8)>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for lock_key in input.split("\n\n") {
        let details = lock_key
            .split("\n")
            .map(|line| {
                line.bytes()
                    .map(|byte| match byte {
                        b'#' => 1,
                        b'.' => 0,
                        _ => panic!("unexpected byte"),
                    })
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();

        let mut lock_key = vec![0; 5];
        for col in 0..5 {
            for row in 0..7 {
                lock_key[col] += details[row][col];
            }
        }
        if details[0][0] == 0 {
            locks.push((
                lock_key[0],
                lock_key[1],
                lock_key[2],
                lock_key[3],
                lock_key[4],
            ));
        } else {
            keys.push((
                lock_key[0],
                lock_key[1],
                lock_key[2],
                lock_key[3],
                lock_key[4],
            ));
        }
    }
    (locks, keys)
}

pub fn part_one(input: &str) -> Option<usize> {
    let (locks, keys) = parse_input(input);
    /*println!("locks: {}, keys: {}", locks.len(), keys.len());
    for lock in &locks {
        println!("lock: {:?}", lock);
    }
    for key in &keys {
        println!("key: {:?}", key);
    }*/
    let mut possible_matches: usize = 0;
    for (lock_num, lock) in locks.iter().enumerate() {
        for (key_num, key) in keys.iter().enumerate() {
            if (lock.0 + key.0) <= 7
                && (lock.1 + key.1) <= 7
                && (lock.2 + key.2) <= 7
                && (lock.3 + key.3) <= 7
                && (lock.4 + key.4) <= 7
            {
                /*println!(
                    "lock: {} {:?} matches key: {} {:?}",
                    lock_num, lock, key_num, key
                );*/
                possible_matches += 1;
            }
        }
    }

    Some(possible_matches)
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
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

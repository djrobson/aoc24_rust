advent_of_code::solution!(4);
fn get_vec_from_input(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

fn get_directions() -> Vec<(i32, i32)> {
    vec![
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, 0),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ]
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = get_vec_from_input(input);
    let max_col = grid[0].len();
    let max_row = grid.len();
    let word: &[u8] = "XMAS".as_bytes();
    let mut count = 0;

    // for all the starting points
    for row in 0..max_row {
        for col in 0..max_col {
            // if the starting point is the first letter of the word
            if grid[row][col] == word[0] {
                // for all the directions
                for (dx, dy) in get_directions() {
                    let mut found = true;
                    for i in 1..word.len() {
                        let new_row = row as i32 + i as i32 * dx;
                        let new_col = col as i32 + i as i32 * dy;
                        if new_row < 0
                            || new_row >= max_row as i32
                            || new_col < 0
                            || new_col >= max_col as i32
                        {
                            found = false;
                            break;
                        }
                        if grid[new_row as usize][new_col as usize] != word[i] {
                            found = false;
                            break;
                        }
                    }
                    if found {
                        count += 1;
                    }
                }
            }
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = get_vec_from_input(input);
    let max_col = grid[0].len();
    let max_row = grid.len();
    let word: &[u8] = "MAS".as_bytes();
    let mut count = 0;

    // for all the starting points that have diagonal neighbors
    for row in 1..max_row - 1 {
        for col in 1..max_col - 1 {
            // if the starting point is the first letter of the word
            if grid[row][col] == word[1] {
                // check top left and bottom right
                if (grid[row - 1][col - 1] == word[0] && grid[row + 1][col + 1] == word[2])
                    || (grid[row - 1][col - 1] == word[2] && grid[row + 1][col + 1] == word[0])
                {
                    // check top right and bottom left
                    if (grid[row - 1][col + 1] == word[0] && grid[row + 1][col - 1] == word[2])
                        || (grid[row - 1][col + 1] == word[2] && grid[row + 1][col - 1] == word[0])
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_small() {
        let input = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";
        let result = part_one(&input);
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two_small() {
        let input = "M.S
.A.
M.S";
        let result = part_two(&input);
        assert_eq!(result, Some(1));
    }

    #[test]
    fn test_part_two() {
        let input = ".M.S......
..A..MSMS.
.M.S.MAA..
..A.ASMSM.
.M.S.M....
..........
S.S.S.S.S.
.A.A.A.A..
M.M.M.M.M.
..........";
        let result = part_two(input);
        assert_eq!(result, Some(9));
    }
}

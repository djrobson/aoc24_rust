advent_of_code::solution!(9);
use std::fmt;

#[derive(Debug, Copy, Clone)]
enum Block {
    F(File),
    E(Empty),
}

#[derive(Debug, Copy, Clone)]
struct File {
    size: u32,
    file_index: u32,
}

#[derive(Debug, Copy, Clone)]
struct Empty {
    size: u32,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Block::F(file) => {
                let f_str = (file.file_index % 10).to_string();
                write!(f, "{}", f_str.repeat(file.size as usize))
            }
            Block::E(empty) => write!(f, "{}", ".".repeat(empty.size as usize)),
        }
    }
}

fn get_checksum(blocks: &Vec<Block>) -> u64 {
    let mut acc = 0;
    let mut index = 0;
    for block in blocks.iter() {
        match block {
            Block::F(f) => {
                let mut new_sum = 0;
                for i in index..index + f.size {
                    //println!("{} {}", i, f.file_index);
                    new_sum += i as u64 * f.file_index as u64;
                }
                index += f.size;
                acc += new_sum;
            }
            Block::E(_) => break,
        }
    }
    acc
}

pub fn part_one(input: &str) -> Option<u64> {
    // parse the input into an alternating sequence of file and empty blocks
    let mut original_blocks: Vec<Block> = Vec::new();
    let mut is_file = true;
    let mut file_index = 0;
    for record in input.bytes() {
        let size = record as u32 - '0' as u32;
        if size > 0 {
            if is_file {
                original_blocks.push(Block::F(File { size, file_index }));
                file_index += 1;
            } else {
                original_blocks.push(Block::E(Empty { size }));
            }
        }
        is_file = !is_file;
    }

    /*for block in original_blocks.iter() {
        print!("{}", block);
    }
    println!("");*/

    let mut compact_blocks: Vec<Block> = Vec::new();
    let mut head_cursor = 0;
    let mut tail_cursor = original_blocks.len() - 1;
    loop {
        match original_blocks[head_cursor] {
            Block::F(head_file) => {
                compact_blocks.push(Block::F(head_file));
                head_cursor += 1;
            }
            Block::E(head_empty) => {
                // skip all the empty blocks at the end
                while let Block::E(_) = original_blocks[tail_cursor] {
                    tail_cursor -= 1;
                }
                if let Block::F(tail_file) = original_blocks[tail_cursor] {
                    if tail_file.size == head_empty.size {
                        compact_blocks.push(Block::F(tail_file));
                        tail_cursor -= 1;
                        head_cursor += 1;
                    } else if tail_file.size < head_empty.size {
                        compact_blocks.push(Block::F(tail_file));

                        //replace the empty block at the head with a smaller one
                        original_blocks[head_cursor] = Block::E(Empty {
                            size: head_empty.size - tail_file.size,
                        });
                        tail_cursor -= 1;
                    } else {
                        let mut smaller_tail_file = tail_file.clone();
                        let size_left = tail_file.size - head_empty.size;
                        smaller_tail_file.size = head_empty.size;
                        compact_blocks.push(Block::F(File {
                            size: head_empty.size,
                            file_index: tail_file.file_index,
                        })); // replace the file block at the tail with a smaller one
                        original_blocks[tail_cursor] = Block::F(File {
                            size: size_left,
                            file_index: tail_file.file_index,
                        });
                        head_cursor += 1;
                    }
                } else {
                    panic!("unexpected  block type");
                }
            }
        }
        if head_cursor >= tail_cursor {
            compact_blocks.push(original_blocks[tail_cursor]);
            break;
        }
    }

    /*for block in compact_blocks.iter() {
        print!("{}", block);
    }
    println!("");
    */

    let checksum = get_checksum(&compact_blocks);
    Some(checksum)
}

#[derive(Debug, Copy, Clone)]
enum Block2 {
    F(File),
    E(Empty),
    D,
}

pub fn part_two(input: &str) -> Option<u64> {
    // parse the input into an alternating sequence of file and empty blocks
    let mut original_blocks: Vec<Block2> = Vec::new();
    let mut is_file = true;
    let mut file_index = 0;
    for record in input.bytes() {
        let size = record as u32 - '0' as u32;
        if size > 0 {
            if is_file {
                original_blocks.push(Block2::F(File { size, file_index }));
                file_index += 1;
            } else {
                original_blocks.push(Block2::E(Empty { size }));
            }
        }
        is_file = !is_file;
    }

    /*for block in original_blocks.iter() {
        print!("{}", block);
    }
    println!("");*/

    let mut compact_blocks: Vec<Block2> = Vec::new();
    let mut head_cursor = 0;
    let mut tail_cursor = original_blocks.len() - 1;
    loop {
        match original_blocks[head_cursor] {
            Block2::F(head_file) => {
                compact_blocks.push(Block2::F(head_file));
                head_cursor += 1;
            }
            Block2::E(head_empty) => {
                // skip all the empty blocks at the end
                while let Block2::E(_) = original_blocks[tail_cursor] {
                    tail_cursor -= 1;
                }
                if let Block2::F(tail_file) = original_blocks[tail_cursor] {
                    if tail_file.size == head_empty.size {
                        compact_blocks.push(Block2::F(tail_file));
                        tail_cursor -= 1;
                        head_cursor += 1;
                    } else if tail_file.size < head_empty.size {
                        compact_blocks.push(Block2::F(tail_file));

                        //replace the empty block at the head with a smaller one
                        original_blocks[head_cursor] = Block2::E(Empty {
                            size: head_empty.size - tail_file.size,
                        });
                        tail_cursor -= 1;
                    } else {
                        let mut smaller_tail_file = tail_file.clone();
                        let size_left = tail_file.size - head_empty.size;
                        smaller_tail_file.size = head_empty.size;
                        compact_blocks.push(Block2::F(File {
                            size: head_empty.size,
                            file_index: tail_file.file_index,
                        })); // replace the file block at the tail with a smaller one
                        original_blocks[tail_cursor] = Block2::F(File {
                            size: size_left,
                            file_index: tail_file.file_index,
                        });
                        head_cursor += 1;
                    }
                } else {
                    panic!("unexpected  block type");
                }
            }
            Block2::D => {
                todo!("implement the delted block");
            }
        }
        if head_cursor >= tail_cursor {
            compact_blocks.push(original_blocks[tail_cursor]);
            break;
        }
    }

    /*for block in compact_blocks.iter() {
        print!("{}", block);
    }
    println!("");
    */

    let checksum = get_checksum(&compact_blocks);
    Some(checksum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}

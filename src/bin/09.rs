advent_of_code::solution!(9);

#[derive(Debug, Copy,Clone)]
enum Block {
    F(File),
    E(Empty),
}

#[derive(Debug, Copy,Clone)]
struct File {
    size: u32,
    file_index: u32,
}

#[derive(Debug, Copy,Clone)]
struct Empty {
    size: u32,
}
pub fn part_one(input: &str) -> Option<u32> {

    // parse the input into an alternating sequence of file and empty blocks
    let mut original_blocks: Vec<Block> = Vec::new();
    let mut is_file = true;
    let mut file_index = 0;
    for record in input.bytes() {
        let size = record as u32;
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

    let mut compact_blocks: Vec<Block> = Vec::new();

    let mut head_cursor = 0;
    let mut tail_cursor = original_blocks.len() - 1;
    loop {
        match original_blocks[head_cursor] {
            Block::F(head_file) => {
                compact_blocks.push(Block::F(head_file));
            }
            Block::E(head_empty) => {
                let mut blocks_to_fill = head_empty.size;
                // skip all the empty blocks at the end
                while let Block::E(_) = original_blocks[tail_cursor] {
                    tail_cursor -= 1;
                }
                match original_blocks[tail_cursor] {
                    Block::F(tail_file) => {
                        // if the empty block at head_cursor is the same size as the file at tail_cursor
                        //  the move the while thing into the compact_blocks
                        // else if the empty block at head_cursor is larger than the file at tail_cursor
                        //  then move the file at tail_cursor into the compact_blocks and decrement blocks_to_fill
                        //  add more file blocks to the compact_blocks until blocks_to_fill is 0
                        // else if the empty block at head_cursor is smaller than the file at tail_cursor
                        //  then move some of the file at tail_cursor into the compact_blocks and decrement tail file size by the amount moved
                        if tail_file.size == head_empty.size {
                            compact_blocks.push(Block::F(tail_file));
                            tail_cursor -= 1;
                        } else if tail_file.size < blocks_to_fill {
                            compact_blocks.push(Block::F(tail_file));
                            blocks_to_fill -= tail_file.size;
                            while blocks_to_fill > 0 {
                                match original_blocks[tail_cursor] {
                                    Block::F(f) => {
                                        compact_blocks.push(Block::F(f));
                                        blocks_to_fill -= 1;
                                    }
                                    Block::E(_) => {
                                        panic!("we just skipped all these");
                                    }
                                }
                            }
                        } else {
                            let mut smaller_tail_file = tail_file.clone();
                            let size_left = tail_file.size - blocks_to_fill;
                            smaller_tail_file.size = blocks_to_fill;
                            blocks_to_fill = 0;
                            compact_blocks.push(Block::F(smaller_tail_file));
                            //replace the tail file with the smaller tail file
                            // TODO loop around and stick the rest of the file someplace?
                            original_blocks[tail_cursor] = Block::F(File { size: size_left, file_index: tail_file.file_index });
                        }
                    }
                    Block::E(_) => {
                        panic!("we just skipped all these");
                    }
                }
            }
        }
        head_cursor += 1;
        if head_cursor >= tail_cursor {
            break;
        }
    }
    let checksum = compact_blocks.iter().fold(0, |acc, block| {
        match block {
            Block::F(f) => {
                let mut new_sum = 0;
                for i in f.file_index..f.file_index + f.size {
                    new_sum += i * f.file_index
                }
                acc + new_sum},
            Block::E(_) => acc,
        }
    });
    Some(checksum)
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
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

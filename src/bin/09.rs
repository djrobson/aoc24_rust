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

fn get_checksum(blocks: &[Block]) -> u64 {
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
                    #[allow(clippy::comparison_chain)]
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
                        let mut smaller_tail_file = tail_file;
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

struct FileSpan {
    size: u32,
    file_index: u32,
    trailing_files: Vec<File>,
    trailing_space: u32,
    leading_space: u32,
}

impl fmt::Display for FileSpan {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut f_str = (self.file_index % 10).to_string();
        write!(f, "{}", f_str.repeat(self.size as usize))?;
        write!(f, "{}", ".".repeat(self.leading_space as usize))?;
        for trailing_file in self.trailing_files.iter() {
            f_str = (trailing_file.file_index % 10).to_string();
            write!(f, "{}", f_str.repeat(trailing_file.size as usize))?;
        }
        write!(f, "{}", ".".repeat(self.trailing_space as usize))
    }
}

fn get_checksum_from_file(offset: u32, file_index: u32, file_size: u32) -> u64 {
    let mut acc = 0;
    for i in 0..file_size {
        acc += (i + offset) as u64 * file_index as u64;
    }
    acc
}

fn get_checksum2(blocks: &[FileSpan]) -> u64 {
    let mut acc = 0;
    let mut offset = 0;
    for block in blocks.iter() {
        acc += get_checksum_from_file(offset, block.file_index, block.size);
        offset += block.size;
        offset += block.leading_space;
        for trailing_file in block.trailing_files.iter() {
            acc += get_checksum_from_file(offset, trailing_file.file_index, trailing_file.size);
            offset += trailing_file.size;
        }
        offset += block.trailing_space;
    }
    acc
}

/*fn print_blocks(blocks: &Vec<FileSpan>) {
    for block in blocks.iter() {
        print!("{}", block);
    }
    println!("");
}*/
#[allow(dead_code)]
fn print_spans(spans: &[FileSpan]) {
    for span in spans.iter() {
        print!("{}", span);
    }
    println!();
}

pub fn part_two(input: &str) -> Option<u64> {
    // parse the input into an alternating sequence of file and empty blocks
    let mut original_blocks: Vec<FileSpan> = Vec::new();
    let mut file_index = 0;
    let input_bytes = input.bytes().collect::<Vec<u8>>();

    #[allow(clippy::explicit_counter_loop)]
    for record in input_bytes.chunks(2) {
        let fsize = record[0] as u32 - '0' as u32;
        if fsize == 0 {
            panic!("unexpected empty block");
        }
        let esize = *record.get(1).unwrap_or(&b'0') as u32 - '0' as u32;
        original_blocks.push(FileSpan {
            size: fsize,
            file_index,
            trailing_files: Vec::new(),
            trailing_space: esize,
            leading_space: 0,
        });
        file_index += 1;
    }

    //print_spans(&original_blocks);

    // for each block at the tail
    let mut tail_cursor = original_blocks.len() - 1;
    while tail_cursor > 1 {
        // look for the first block, with index lower than tail, with enough empty space
        for head_index in 0..tail_cursor {
            if original_blocks[head_index].trailing_space >= original_blocks[tail_cursor].size {
                /*println!(
                    "moving tail: {} to be after head {}",
                    original_blocks[tail_cursor].file_index, original_blocks[head_index].file_index
                );*/
                // append the tail block to the trailing files of the front block
                let moved_file_index = original_blocks[tail_cursor].file_index;
                let moved_file_size = original_blocks[tail_cursor].size;
                original_blocks[head_index].trailing_files.push(File {
                    size: moved_file_size,
                    file_index: moved_file_index,
                });
                // decrement the front blocks empty space by the size of the tail block
                original_blocks[head_index].trailing_space -= original_blocks[tail_cursor].size;

                // clear out the tail block
                original_blocks[tail_cursor].leading_space += original_blocks[tail_cursor].size;
                original_blocks[tail_cursor].size = 0;

                //print_spans(&original_blocks);
                break;
            }
        }
        tail_cursor -= 1;
    }

    //print_spans(&original_blocks);

    let checksum = get_checksum2(&original_blocks);
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

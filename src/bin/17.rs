use core::panic;
use rayon::prelude::*;

advent_of_code::solution!(17);

#[allow(dead_code)]
enum Combo {
    Lit0,
    Lit1,
    Lit2,
    Lit3,
    RegA,
    RegB,
    RegC,
    Invalid17,
}

#[allow(dead_code)]
enum Inst {
    Adv0, // a = a div (2^combo)
    Bxl1, // b = b xor literal
    Bst2, // b = combo mod 8
    Jnz3, // if a != 0 then jump literal
    Bxc4, // b = b xor c, no operand
    Out5, // output = combo operand mod 8
    Bdv6, // b = a div (2^combo)
    Cdv7, // c = a div (2^combo)
}

#[allow(dead_code)]
fn parse_inst(inst: usize) -> Inst {
    match inst {
        0 => Inst::Adv0,
        1 => Inst::Bxl1,
        2 => Inst::Bst2,
        3 => Inst::Jnz3,
        4 => Inst::Bxc4,
        5 => Inst::Out5,
        6 => Inst::Bdv6,
        7 => Inst::Cdv7,
        _ => panic!("Invalid instruction"),
    }
}

#[derive(Debug, PartialEq, Clone)]
struct MachineState {
    a: usize,
    b: usize,
    c: usize,
    pc: usize,
    memory: Vec<usize>,
    output: Vec<usize>,
}

fn parse_input(input: &str) -> MachineState {
    // read input like:
    // Register A: 729
    // Register B: 0
    // Register C: 0
    //
    // Program: 0,1,5,4,3,0
    let mut lines = input.lines();
    let a = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let b = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let c = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    let _ = lines.next(); // skip blank line
    let memory = lines
        .next()
        .unwrap()
        .split_whitespace()
        .last()
        .unwrap()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();
    MachineState {
        a,
        b,
        c,
        pc: 0,
        memory,
        output: vec![],
    }
}

fn get_combo_value(combo: usize, machine: &MachineState) -> usize {
    match combo {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => machine.a,
        5 => machine.b,
        6 => machine.c,
        _ => panic!("Invalid combo value"),
    }
}

fn run_machine(machine: &mut MachineState) {
    loop {
        if machine.pc >= machine.memory.len() {
            break;
        }
        let inst = machine.memory[machine.pc];
        let operand = machine.memory[machine.pc + 1];
        match inst {
            0 => {
                machine.a /= (2 as usize).pow(get_combo_value(operand, machine) as u32);
                machine.pc += 2;
            }
            1 => {
                machine.b ^= operand;
                machine.pc += 2;
            }
            2 => {
                machine.b = get_combo_value(operand, machine) % 8;
                machine.pc += 2;
            }
            3 => {
                if machine.a != 0 {
                    machine.pc = operand;
                    continue;
                }
                machine.pc += 2;
            }
            4 => {
                machine.b ^= machine.c;
                machine.pc += 2;
            }
            5 => {
                machine.output.push(get_combo_value(operand, machine) % 8);
                machine.pc += 2;
            }
            6 => {
                machine.b = machine.a / (2 as usize).pow(get_combo_value(operand, machine) as u32);
                machine.pc += 2;
            }
            7 => {
                machine.c = machine.a / (2 as usize).pow(get_combo_value(operand, machine) as u32);
                machine.pc += 2;
            }
            _ => {
                panic!("Invalid instruction");
            }
        }
    }
}

fn run_machine_2(a_reg: usize, output: &[u8]) -> bool {
    let mut a_reg = a_reg;
    let mut b_reg;
    let mut c_reg;
    let mut output_cursor = 0;

    while a_reg != 0 {
        b_reg = a_reg & 0b111;
        b_reg ^= 3;
        c_reg = a_reg >> b_reg;
        a_reg = a_reg >> 1;
        b_reg = b_reg ^ 1;
        b_reg = b_reg ^ c_reg;
        if output[output_cursor] != (b_reg & 0b111) as u8 {
            return false;
        }
        output_cursor += 1;
    }
    output_cursor == 16
}

pub fn part_one(input: &str) -> Option<String> {
    let mut machine = parse_input(input);
    run_machine(&mut machine);
    Some(
        machine
            .output
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(","),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let area_size = std::thread::available_parallelism().unwrap().get() * 100_000_000;
    //let area_size = std::thread::available_parallelism().unwrap().get() * 118000;
    let output: &[u8; 16] = &[2, 4, 1, 3, 7, 5, 0, 3, 1, 5, 4, 4, 5, 5, 3, 0];

    for area in 0..((2 as usize).pow(59)) {
        // get precise time
        let start_time = std::time::Instant::now();
        let area_start = area * area_size;
        let area_end = (area + 1) * area_size;
        print!("Trying area {}", area_start);
        let result = (area_start..area_end)
            .into_par_iter()
            .find_first(|reg_a| run_machine_2(*reg_a, output));
        if result.is_some() {
            println!(
                " found {} in {}",
                result.unwrap(),
                start_time.elapsed().as_secs_f32()
            );
            return result;
        } else {
            println!(
                " failed: {} checks/sec",
                (area_size as f32) / start_time.elapsed().as_secs_f32()
            );
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_1() {
        // If register C contains 9, the program 2,6 would set register B to 1.
        let mut machine = MachineState {
            a: 0,
            b: 0,
            c: 9,
            pc: 0,
            memory: vec![2, 6],
            output: vec![],
        };
        run_machine(&mut machine);
        assert_eq!(
            machine,
            MachineState {
                a: 0,
                b: 1,
                c: 9,
                pc: 2,
                memory: vec![2, 6],
                output: vec![],
            }
        );
    }
    #[test]
    fn test_part_one_2() {
        // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
        let mut machine = MachineState {
            a: 10,
            b: 0,
            c: 0,
            pc: 0,
            memory: vec![5, 0, 5, 1, 5, 4],
            output: vec![],
        };
        run_machine(&mut machine);
        assert_eq!(
            machine,
            MachineState {
                a: 10,
                b: 0,
                c: 0,
                pc: 6,
                memory: vec![5, 0, 5, 1, 5, 4],
                output: vec![0, 1, 2],
            }
        );
    }
    #[test]
    fn test_part_one_3() {
        // If register A contains 2024, the program 0,1,5,4,3,0
        // would output v and leave 0 in register A
        let mut machine = MachineState {
            a: 2024,
            b: 0,
            c: 0,
            pc: 0,
            memory: vec![0, 1, 5, 4, 3, 0],
            output: vec![],
        };
        run_machine(&mut machine);
        assert_eq!(
            machine,
            MachineState {
                a: 0,
                b: 0,
                c: 0,
                pc: 6,
                memory: vec![0, 1, 5, 4, 3, 0],
                output: vec![4, 2, 5, 6, 7, 7, 7, 7, 3, 1, 0],
            }
        );
    }
    #[test]
    fn test_part_one_4() {
        // If register B contains 29, the program 1,7 would set register B to 26
        let mut machine = MachineState {
            a: 0,
            b: 29,
            c: 0,
            pc: 0,
            memory: vec![1, 7],
            output: vec![],
        };
        run_machine(&mut machine);
        assert_eq!(
            machine,
            MachineState {
                a: 0,
                b: 26,
                c: 0,
                pc: 2,
                memory: vec![1, 7],
                output: vec![],
            }
        );
    }
    #[test]
    fn test_part_one_5() {
        // If register B contains 2024 and register C contains 43690,
        // the program 4,0 would set register B to 44354.
        let mut machine = MachineState {
            a: 0,
            b: 2024,
            c: 43690,
            pc: 0,
            memory: vec![4, 0],
            output: vec![],
        };
        run_machine(&mut machine);
        assert_eq!(
            machine,
            MachineState {
                a: 0,
                b: 44354,
                c: 43690,
                pc: 2,
                memory: vec![4, 0],
                output: vec![],
            }
        );
    }
    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("4,6,3,5,6,3,5,2,1,0".to_string()));
    }

    #[ignore]
    #[test]
    fn test_part_two() {
        let result = part_two(
            "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0",
        );
        println!("{:?}", result);
        assert_eq!(result, Some(117440));
    }

    #[ignore]
    #[test]
    fn test_part_two_1() {
        let a_reg: usize = 0xea0304aa258b;
        println!("a_reg: {}", a_reg);
        assert!(run_machine_2(
            a_reg,
            &[2, 4, 1, 3, 7, 5, 0, 3, 1, 5, 4, 4, 5, 5, 3, 0]
        ));
    }
}

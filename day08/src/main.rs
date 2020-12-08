use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashSet;

fn main() {
    let mut input: Vec<String> = Vec::new();
    if let Ok(file) = File::open("input.dat") {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            input.push(line.unwrap());
        }
    }

    let mut codes:Vec<Code> = Vec::new();
    for line in input {
        let mut line_split = line.split(" ");
        let instruction = line_split.next().unwrap();
        let value = line_split.next().unwrap().parse::<i32>().unwrap();
        codes.push(Code {instruction: Instruction::from_string(instruction), value });
    }

    part1(&codes);
    part2(&codes);
}

#[derive(Clone, Debug)]
enum Instruction {
    ACC,
    JMP,
    NOP
}

impl Instruction {
    fn from_string(value: &str) -> Self {
        match value {
            "acc" => Instruction::ACC,
            "jmp" => Instruction::JMP,
            "nop" => Instruction::NOP,
            _ => Instruction::NOP
        }
    }
}

struct Code {
    instruction: Instruction,
    value: i32
}

fn process(codes: &Vec<Code>, return_zero_on_duplicate: bool) -> i64 {
    let mut accumulator:i64 = 0;
    let mut index:i32 = 0;
    let mut seen_indices:HashSet<i32> = HashSet::new();
    loop {
        if index == codes.len() as i32 {
            return accumulator;
        } else if index > codes.len() as i32 {
            return 0;
        }
        let code = codes.get(index as usize).unwrap();
        if seen_indices.contains(&index) {
            if return_zero_on_duplicate {
                return 0;
            }
            return accumulator;
        }
        seen_indices.insert(index.clone());
        // println!("{:?} {}", code.instruction, code.value);
        match code.instruction {
            Instruction::ACC => {
                accumulator += code.value as i64;
                index += 1;
            }
            Instruction::JMP => {
                index += code.value;
            }
            Instruction::NOP => {
                index += 1;
            }
        }
    }
}

fn part1(codes: &Vec<Code>) {
    println!("Accumulator value for part 1: {}", process(codes, false));
}


fn part2(codes: &Vec<Code>) {
    for index in 0..codes.len() {
        let mut codes_copy:Vec<Code> = Vec::new();
        let code = codes.get(index).unwrap();
        match code.instruction {
            Instruction::JMP => {
                for clone_index in 0..codes.len() {
                    if clone_index == index {
                        codes_copy.push(Code { instruction: Instruction::NOP, value: code.value });
                    } else {
                        let clone_code = codes.get(clone_index).unwrap();
                        let instruction = clone_code.instruction.clone();
                        codes_copy.push(Code {instruction, value: clone_code.value});
                    }
                }
            },
            _ => {}
        }
        let result = process(&codes_copy, true);
        if result > 0 {
            println!("Accumulator value (changing index {}) for part 2: {}", index, result);

        }
    }
}
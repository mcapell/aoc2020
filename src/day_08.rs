use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use lazy_static::lazy_static;
use regex::Regex;

const FILEPATH: &str = "data/08/input.txt";

pub fn first_solution() {
    let instructions = parse_input();
    let total = match execute_instructions(&instructions) {
        Ok(value) => value,
        Err(value) => value,
    };
    println!("Solution: {}", total);
}

pub fn second_solution() {
    let mut instructions = parse_input();
    for i in 0..instructions.len() {
        if instructions[i].operation == Operation::ACC {
            continue;
        }
        instructions[i] = switch_instruction(&instructions[i]);
        match execute_instructions(&instructions) {
            Ok(value) => {
                println!("Solution: {}", value);
                break;
            }
            Err(_) => {
                // Switch back
                instructions[i] = switch_instruction(&instructions[i]);
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum Operation {
    NOP,
    ACC,
    JMP,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    operation: Operation,
    value: i32,
}

fn parse_input() -> Vec<Instruction> {
    let file = File::open(FILEPATH).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|v| parse_instruction(v.expect("Unable to read line")))
        .collect()
}

fn execute_instructions(instructions: &[Instruction]) -> Result<i64, i64> {
    let mut executed = HashSet::new();
    let mut pointer: i32 = 0;
    let mut acc: i64 = 0;
    while !executed.contains(&pointer) && pointer < instructions.len() as i32 {
        executed.insert(pointer);
        let instruction = &instructions[pointer as usize];
        match instruction.operation {
            Operation::NOP => pointer += 1,
            Operation::ACC => {
                acc += instruction.value as i64;
                pointer += 1
            }
            Operation::JMP => pointer += instruction.value,
        };
    }
    if !executed.contains(&pointer) {
        Ok(acc)
    } else {
        Err(acc)
    }
}

fn switch_instruction(instruction: &Instruction) -> Instruction {
    Instruction {
        operation: match instruction.operation {
            Operation::NOP => Operation::JMP,
            Operation::JMP => Operation::NOP,
            Operation::ACC => Operation::ACC,
        },
        value: instruction.value,
    }
}

lazy_static! {
    static ref RE: Regex = Regex::new(r"^(\w+)\s(.)(\d+)").expect("Invalid regex");
}

fn parse_instruction(instruction: String) -> Instruction {
    let res = RE
        .captures(instruction.as_str())
        .expect("Invalid instruction");
    let value = res[3].parse::<i32>().expect("Expected a numeric value");
    Instruction {
        operation: match &res[1] {
            "jmp" => Operation::JMP,
            "nop" => Operation::NOP,
            "acc" => Operation::ACC,
            op => panic!("Invalid operation: {}", op),
        },
        value: value
            * match &res[2] {
                "-" => -1,
                "+" => 1,
                _ => panic!("Expected +/-"),
            },
    }
}

#[test]
fn test_execute_instructions_until_loop() {
    let instructions = vec![
        Instruction {
            operation: Operation::NOP,
            value: 0,
        },
        Instruction {
            operation: Operation::ACC,
            value: 1,
        },
        Instruction {
            operation: Operation::JMP,
            value: 4,
        },
        Instruction {
            operation: Operation::ACC,
            value: 3,
        },
        Instruction {
            operation: Operation::JMP,
            value: -3,
        },
        Instruction {
            operation: Operation::ACC,
            value: -99,
        },
        Instruction {
            operation: Operation::ACC,
            value: 1,
        },
        Instruction {
            operation: Operation::JMP,
            value: -4,
        },
        Instruction {
            operation: Operation::ACC,
            value: 6,
        },
    ];
    assert_eq!(true, execute_instructions(&instructions).is_err());
    assert_eq!(
        5,
        match execute_instructions(&instructions) {
            Ok(value) => value,
            Err(value) => value,
        }
    );
}

#[test]
fn test_parse_instruction() {
    let expected = Instruction {
        operation: Operation::JMP,
        value: -99,
    };
    assert_eq!(expected, parse_instruction("jmp -99".to_string()));
}

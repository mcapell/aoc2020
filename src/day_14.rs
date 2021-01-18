use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use lazy_static::lazy_static;
use regex::Regex;

const FILEPATH: &str = "data/14/input.txt";

pub fn first_solution() {
    println!("{}", run_program_v1(parse_lines()));
}

pub fn second_solution() {
    println!("{}", run_program_v2(parse_lines()));
}

fn parse_lines() -> Vec<String> {
    let file = File::open(FILEPATH).expect("File not found");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("error reading line"))
        .collect()
}

type Mask = Vec<char>;

struct Memory {
    address: u64,
    value: u64,
}

fn parse_mask(input: String) -> Mask {
    input.chars().skip(7).collect()
}

lazy_static! {
    static ref RE_MEM: Regex = Regex::new(r"^mem\[(\d+)\]\s=\s(\d+)$").expect("Invalid regex");
}

fn parse_memory(input: String) -> Memory {
    let groups = RE_MEM.captures(&input).expect("Invalid input");
    Memory {
        address: groups[1].parse().expect("invalid numeric value"),
        value: groups[2].parse().expect("invalid numeric value"),
    }
}

fn run_program_v1(lines: Vec<String>) -> u64 {
    let mut mask: Mask = vec![];
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for line in lines {
        if line.starts_with("mask") {
            mask = parse_mask(line);
            continue;
        }
        let mem = parse_memory(line);

        let value = compute_decoder_v1(&mask, mem.value);

        memory.insert(mem.address, value);
    }
    memory.iter().map(|(_, v)| v).sum()
}

fn compute_decoder_v1(mask: &Mask, value: u64) -> u64 {
    let bitvalue: Vec<u64> = format!("{:#038b}", value)
        .chars()
        .skip(2)
        .map(|c| c.to_digit(10).expect("expected a numeric value") as u64)
        .collect();
    let mut result: Vec<u64> = vec![0; 36];
    for i in 0..=35 {
        result[35 - i] = if mask[35 - i] == 'X' {
            bitvalue[35 - i]
        } else {
            mask[35 - i].to_digit(10).expect("expected numeric value") as u64
        }
    }
    result.iter().fold(0, |res, bit| (res << 1) ^ bit)
}

fn run_program_v2(lines: Vec<String>) -> u64 {
    let mut mask: Mask = vec![];
    let mut memory: HashMap<u64, u64> = HashMap::new();
    for line in lines {
        if line.starts_with("mask") {
            mask = parse_mask(line);
            continue;
        }
        let mem = parse_memory(line);

        for addr in compute_addresses(&mask, mem.address) {
            memory.insert(addr, mem.value);
        }
    }
    memory.iter().map(|(_, v)| v).sum()
}

fn compute_addresses(mask: &Mask, value: u64) -> Vec<u64> {
    let bitvalue: Vec<char> = format!("{:#038b}", value).chars().skip(2).collect();
    let mut result: Vec<char> = vec!['0'; 36];
    for i in 0..=35 {
        result[35 - i] = if mask[35 - i] == 'X' {
            'X'
        } else if mask[35 - i] == '1' {
            '1'
        } else {
            bitvalue[35 - i]
        }
    }

    let mut results: Vec<u64> = vec![];
    for perm in perms(
        result.iter().filter(|e| **e == 'X').count(),
        vec!["0".to_string(), "1".to_string()],
    ) {
        let mut chars = perm.chars();
        results.push(
            result
                .iter()
                .map(|c| {
                    if *c == 'X' {
                        chars.next().expect("expected permutation")
                    } else {
                        *c
                    }
                })
                .map(|c| c.to_digit(10).expect("expected numeric value") as u64)
                .fold(0, |res, bit| (res << 1) ^ bit),
        );
    }

    results
}

fn perms(num: usize, nums: Vec<String>) -> Vec<String> {
    if num == 1 {
        return nums;
    }
    let mut new_nums = vec![];
    for n in nums {
        new_nums.push(format!("0{}", n));
        new_nums.push(format!("1{}", n));
    }
    perms(num - 1, new_nums)
}

#[test]
fn test_parse_mask() {
    let input = "mask = X101011X011X10101011000001X00XX0X000".to_string();
    assert_eq!('X', parse_mask(input)[0]);
}

#[test]
fn test_parse_memory() {
    let input = "mem[54849] = 40771927".to_string();
    let expected = parse_memory(input);
    assert_eq!(54849, expected.address);
    assert_eq!(40771927, expected.value);
}

#[test]
fn test_compute_value() {
    let mask = parse_mask("mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string());
    assert_eq!(101, compute_decoder_v1(&mask, 101));
    assert_eq!(73, compute_decoder_v1(&mask, 11));
}

#[test]
fn test_run_program_v1() {
    let input = vec![
        "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X".to_string(),
        "mem[8] = 11".to_string(),
        "mem[7] = 101".to_string(),
        "mem[8] = 0".to_string(),
    ];
    assert_eq!(165, run_program_v1(input));
}

#[test]
fn test_run_program_v2() {
    let input = vec![
        "mask = 000000000000000000000000000000X1001X".to_string(),
        "mem[42] = 100".to_string(),
        "mask = 00000000000000000000000000000000X0XX".to_string(),
        "mem[26] = 1".to_string(),
    ];
    assert_eq!(208, run_program_v2(input));
}

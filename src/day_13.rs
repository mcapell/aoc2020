use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILEPATH: &str = "data/13/input.txt";

pub fn first_solution() {
    let (timestamp, buses) = parse_timestamp_and_buses();
    let bus = earliest_bus(timestamp, buses);
    println!("Solution: {}", bus.0 * bus.1);
}

pub fn second_solution() {
    let (_, buses) = parse_timestamp_and_buses();
    println!("Solution: {}", earliest_timestamp_fast(&buses));
}

fn parse_timestamp_and_buses() -> (u32, String) {
    let file = File::open(FILEPATH).expect("File not found");
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .map(|l| l.expect("expected lines from input"))
        .collect();
    (
        lines[0].parse::<u32>().expect("numeric timestamp expected"),
        lines[1].clone(),
    )
}

fn earliest_bus(timestamp: u32, buses: String) -> (u32, u32) {
    let mut next_buses: Vec<(u32, u32)> = buses
        .split(',')
        .filter(|c| *c != "x")
        .map(|n| n.parse::<u32>().expect("numeric expected"))
        .map(|n| (n, timestamp - timestamp % n + n))
        .collect();
    next_buses.sort_by(|a, b| a.1.cmp(&b.1));
    (next_buses[0].0, next_buses[0].1 - timestamp)
}

#[allow(dead_code)]
fn earliest_timestamp_slow(input: &str) -> u64 {
    let buses: Vec<(usize, u64)> = input
        .split(',')
        .enumerate()
        .filter(|(_, c)| *c != "x")
        .map(|(n, c)| (n, c.parse::<u64>().expect("numeric expected")))
        .collect();
    let num_buses = buses.iter().count();
    let mut start = 0;
    loop {
        start += 1;
        if num_buses
            == buses
                .iter()
                .map(|(n, b)| (n, b, start - start % b))
                .map(|(n, b, t)| (n, if t < start { t + *b } else { t }))
                .filter(|(n, t)| start + (**n as u64) == *t)
                .count()
        {
            break;
        }
    }

    start
}

fn earliest_timestamp_fast(input: &str) -> i64 {
    let buses: Vec<(i64, i64)> = input
        .split(',')
        .enumerate()
        .filter(|(_, c)| *c != "x")
        .map(|(n, c)| (n as i64, c.parse::<i64>().expect("numeric expected")))
        .collect();

    chinese_remainder(
        &buses.iter().map(|&(n, b)| b - n).collect::<Vec<_>>(),
        &buses.iter().map(|&(_, b)| b).collect::<Vec<_>>(),
    )
    .expect("No solution found")
}

// From https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
#[allow(clippy::many_single_char_names)]
fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

// From https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn mod_inv(x: i64, n: i64) -> Option<i64> {
    let (g, x, _) = egcd(x, n);
    if g == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

// From https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
fn chinese_remainder(residues: &[i64], modulii: &[i64]) -> Option<i64> {
    let prod = modulii.iter().product::<i64>();
    let mut sum = 0;
    for (&residue, &modulus) in residues.iter().zip(modulii) {
        let p = prod / modulus;
        sum += residue * mod_inv(p, modulus)? * p
    }

    Some(sum % prod)
}

#[test]
fn test_get_earliest_bus() {
    let input = "7,13,x,x,59,x,31,19".to_string();
    assert_eq!((59, 5), earliest_bus(939, input));
}

#[test]
fn test_find_timestamp() {
    let input = "17,x,13,19".to_string();
    assert_eq!(3417, earliest_timestamp_slow(&input));
    assert_eq!(3417, earliest_timestamp_fast(&input));
}

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILEPATH: &str = "data/06/input.txt";

pub fn first_solution() {
    let file = File::open(FILEPATH).unwrap();
    let reader = BufReader::new(file);

    let mut group: HashSet<char> = HashSet::new();
    let mut count: i32 = 0;
    for line in reader.lines().map(|l| l.expect("Unable to read line")) {
        if line.is_empty() {
            count += group.len() as i32;
            group = HashSet::new();
            continue;
        }
        for ch in line.chars() {
            group.insert(ch);
        }
    }
    count += group.len() as i32;
    println!("Solution: {}", count);
}

pub fn second_solution() {
    let file = File::open(FILEPATH).unwrap();
    let reader = BufReader::new(file);

    let mut group: HashMap<char, usize> = HashMap::new();
    let mut num_in_group: usize = 0;
    let mut count: i32 = 0;
    for line in reader.lines().map(|l| l.expect("Unable to read line")) {
        if line.is_empty() {
            count += group.values().filter(|v| **v == num_in_group).count() as i32;
            group = HashMap::new();
            num_in_group = 0;
            continue;
        }
        num_in_group += 1;
        for ch in line.chars() {
            match group.get_mut(&ch) {
                Some(v) => { *v += 1; },
                _ => { group.insert(ch, 1); },
            }
        }
    }
    count += group.values().filter(|v| **v == num_in_group).count() as i32;

    println!("Solution: {}", count);
}

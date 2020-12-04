use itertools::Itertools;
use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILEPATH: &str = "data/01/input.txt";

pub fn first_solution() {
    let file = File::open(FILEPATH).unwrap();
    let reader = BufReader::new(file);

    let numbers = reader
        .lines()
        .map(|w| w.unwrap().parse::<i32>().unwrap())
        .collect::<HashSet<i32>>();

    for num in numbers.iter() {
        let candidate = 2020 - num;
        if numbers.contains(&candidate) {
            println!("Solution: {}", num * candidate);
            return;
        }
    }
}

pub fn second_solution() {
    let file = File::open(FILEPATH).unwrap();
    let reader = BufReader::new(file);

    let numbers = reader
        .lines()
        .map(|w| w.unwrap().parse::<i32>().unwrap())
        .collect::<HashSet<i32>>();

    for pair in numbers.iter().combinations(2) {
        let candidate = 2020 - (pair[0] + pair[1]);
        if numbers.contains(&candidate) {
            println!("Solution: {}", candidate * pair[0] * pair[1]);
            return;
        }
    }
}

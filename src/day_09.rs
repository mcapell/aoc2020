use itertools::Itertools;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILEPATH: &str = "data/09/input.txt";

pub fn first_solution() {
    let file = File::open(FILEPATH).unwrap();
    let reader = BufReader::new(file);

    let numbers = reader
        .lines()
        .map(|w| w.unwrap().parse::<u64>().expect("expect numeric values"))
        .collect();
    println!("Solution: {}", find_wrong_number(25, numbers).unwrap());
}

pub fn second_solution() {
    let file = File::open(FILEPATH).unwrap();
    let reader = BufReader::new(file);

    let numbers = reader
        .lines()
        .map(|w| w.unwrap().parse::<u64>().expect("expect numeric values"))
        .collect();
    println!(
        "Solution: {}",
        find_contiguous_numbers(23278925, numbers).unwrap()
    );
}

fn find_wrong_number(window_size: usize, values: Vec<u64>) -> Option<u64> {
    for window in values.windows(window_size + 1) {
        let last = *(window.last().expect("expecting non-empty window"));
        if window
            .iter()
            .combinations(2)
            .filter(|pair| pair[0] + pair[1] == last)
            .count()
            == 0
        {
            return Some(last);
        }
    }
    None
}

fn find_contiguous_numbers(invalid_number: u64, values: Vec<u64>) -> Option<u64> {
    for comb_length in 2..values.len() {
        for combination in values.windows(comb_length) {
            if invalid_number == combination.iter().copied().sum() {
                return Some(
                    combination.iter().min().expect("unexpected empty values")
                        + combination.iter().max().expect("unexpected empty values"),
                );
            }
        }
    }
    None
}

#[test]
fn test_find_wrong_number() {
    let input = vec![
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    assert_eq!(127, find_wrong_number(5, input).unwrap());
}

#[test]
fn test_find_contiguous_numbers() {
    let input = vec![
        35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309, 576,
    ];
    assert_eq!(62, find_contiguous_numbers(127, input).unwrap());
}

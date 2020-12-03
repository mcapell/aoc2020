use itertools::Itertools;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILEPATH: &str = "data/01/input.txt";

pub fn first_solution() {
    let file = File::open(FILEPATH).unwrap();
    let reader = BufReader::new(file);

    for pair in reader
        .lines()
        .map(|w| w.unwrap().parse::<i32>().unwrap())
        .combinations(2)
    {
        if is_list_valid(&pair) {
            println!("Solution: {}", pair.iter().fold(1, |a, n| a * n));
            return;
        }
    }
}

pub fn second_solution() {
    let file = File::open(FILEPATH).unwrap();
    let reader = BufReader::new(file);

    for combination in reader
        .lines()
        .map(|w| w.unwrap().parse::<i32>().unwrap())
        .combinations(3)
    {
        if is_list_valid(&combination) {
            println!("Solution: {}", combination.iter().fold(1, |a, n| a * n));
            return;
        }
    }
}

fn is_list_valid(list: &Vec<i32>) -> bool {
    list.iter().sum::<i32>() == 2020
}

#[test]
fn test_check_list_is_valid() {
    assert_eq!(true, is_list_valid(&vec![2020, 0]));
    assert_eq!(true, is_list_valid(&vec![2019, 1, 0]));
}

#[test]
fn test_check_list_is_invalid() {
    assert_eq!(false, is_list_valid(&vec![100, 200]));
}

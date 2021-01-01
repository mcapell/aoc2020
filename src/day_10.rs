use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILEPATH: &str = "data/10/input.txt";

pub fn first_solution() {
    let numbers = get_sorted_input();
    let distribution = get_jolt_distribution(numbers);
    println!("Solution: {}", distribution[0] * distribution[2]);
}

pub fn second_solution() {
    let numbers = get_sorted_input();
    let mut cache = HashMap::new();
    println!(
        "Solution: {}",
        count_valid_arrangements(&get_adapters(numbers), 0, &mut cache)
    );
}

fn get_sorted_input() -> Vec<i32> {
    let file = File::open(FILEPATH).unwrap();
    let reader = BufReader::new(file);
    let mut numbers = reader
        .lines()
        .map(|w| w.unwrap().parse::<i32>().expect("expect numeric value"))
        .collect::<Vec<i32>>();
    numbers.push(0);
    numbers.push(*numbers.iter().max().expect("unexpected empty input") + 3);
    numbers.sort();
    numbers
}

type Distribution = [i32; 3];

fn get_jolt_distribution(values: Vec<i32>) -> Distribution {
    let mut current_joltage = 0;
    let mut distribution = [0, 0, 0];
    for jolt in values.iter().skip(1) {
        let diff = jolt - current_joltage;
        if diff > 3 {
            break;
        }
        current_joltage = *jolt;
        distribution[diff as usize - 1] += 1;
    }
    distribution
}

fn get_adapters(values: Vec<i32>) -> Vec<i32> {
    let mut adapters = vec![];
    let mut current_joltage = 0;
    for jolt in values {
        if jolt - current_joltage > 3 {
            break;
        }
        current_joltage = jolt;
        adapters.push(jolt);
    }
    adapters
}

fn count_valid_arrangements(
    adapters: &[i32],
    position: usize,
    cache: &mut HashMap<usize, u64>,
) -> u64 {
    if let Some(value) = cache.get(&position) {
        return *value;
    }
    if position == adapters.len() - 1 {
        return 1;
    }
    let mut count = 0;
    for i in position + 1..=position + 3 {
        if i < adapters.len() && adapters[i] - adapters[position] <= 3 {
            count += count_valid_arrangements(adapters, i, cache);
        }
    }
    cache.insert(position, count);
    count
}

#[test]
fn test_get_jolt_distribution() {
    let mut input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4, 0, 22];
    input.sort();
    assert_eq!([7, 0, 5], get_jolt_distribution(input));
}

#[test]
fn test_get_adapters() {
    let mut input = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4, 0, 22];
    input.sort();
    let expected = vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22];
    assert_eq!(expected, get_adapters(input));
}

#[test]
fn test_get_num_valid_arrangements() {
    let input = vec![0, 1, 4, 5, 6, 7, 10, 11, 12, 15, 16, 19, 22];
    let mut cache = HashMap::new();
    assert_eq!(8, count_valid_arrangements(&input, 0, &mut cache));
}

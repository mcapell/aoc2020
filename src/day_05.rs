use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILEPATH: &str = "data/05/input.txt";

pub fn first_solution() {
    let file = File::open(FILEPATH).expect("File not found");
    let reader = BufReader::new(file);
    let max = reader
        .lines()
        .map(|bp| seat_id(bp.expect("unable to parse line")))
        .max();
    println!("Solution: {}", max.expect("unable to get max value"));
}

pub fn second_solution() {
    let file = File::open(FILEPATH).expect("File not found");
    let reader = BufReader::new(file);
    let seats: HashSet<i32> = reader
        .lines()
        .map(|bp| seat_id(bp.expect("unable to parse line")))
        .collect();
    // max seat
    for i in 1..127 * 8 + 7 {
        if !seats.contains(&i) && seats.contains(&(i - 1)) && seats.contains(&(i + 1)) {
            println!("Solution: {}", i);
            break;
        }
    }
}

fn seat_id(boarding_pass: String) -> i32 {
    i32::from_str_radix(
        boarding_pass
            .replace("F", "0")
            .replace("B", "1")
            .replace("L", "0")
            .replace("R", "1")
            .as_str(),
        2,
    )
    .expect("invalid boarding pass")
}

#[test]
fn test_seat_ids() {
    assert_eq!(567, seat_id("BFFFBBFRRR".to_string()));
    assert_eq!(119, seat_id("FFFBBBFRRR".to_string()));
    assert_eq!(820, seat_id("BBFFBBFRLL".to_string()));
}

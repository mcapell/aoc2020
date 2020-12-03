use std::fs::File;
use std::io::{prelude::*, BufReader};

use regex::Regex;

const FILEPATH: &str = "data/02/input.txt";

pub fn first_solution() {
    let file = File::open(FILEPATH).expect("File not found");
    let reader = BufReader::new(file);

    let count = reader
        .lines()
        .map(|l| password_db_parser(l.unwrap()))
        .filter(|pw| pw.first_rule_is_valid())
        .count();
    println!("Solution: {}", count);
}

pub fn second_solution() {
    let file = File::open(FILEPATH).expect("File not found");
    let reader = BufReader::new(file);

    let count = reader
        .lines()
        .map(|l| password_db_parser(l.unwrap()))
        .filter(|pw| pw.second_rule_is_valid())
        .count();
    println!("Solution: {}", count);
}

struct PasswordRule {
    first: usize,
    second: usize,
    ch: char,
    password: String,
}

impl PasswordRule {
    fn first_rule_is_valid(&self) -> bool {
        let count = self.password.chars().filter(|c| c == &self.ch).count();
        self.first <= count && count <= self.second
    }

    fn second_rule_is_valid(&self) -> bool {
        // Only valid input is expected
        let first = self
            .password
            .chars()
            .nth(self.first - 1)
            .expect("index error: invalid input")
            == self.ch;
        let second = self
            .password
            .chars()
            .nth(self.second - 1)
            .expect("index error: invalid input")
            == self.ch;
        (first || second) && first != second
    }
}

/*
 * Password rule input format:
 * 1-3 a: abcde
 */
fn password_db_parser(line: String) -> PasswordRule {
    let re = Regex::new(r"^(\d+)\-(\d+) (\w): (\w+)$").expect("Invalid regex");
    let groups = re.captures(&line).expect("Invalid input");
    PasswordRule {
        first: groups[1].parse().expect("expected `first` to be numeric"),
        second: groups[2].parse().expect("expected `second` to be numeric"),
        ch: groups[3].parse().expect("expected `ch` to be a char"),
        password: groups[4]
            .parse()
            .expect("expected `password` to be a string"),
    }
}

#[test]
fn test_password_db_parser() {
    let exemple = String::from("1-3 a: abcde");
    let pw_rule = password_db_parser(exemple);
    assert_eq!(1, pw_rule.first);
    assert_eq!(3, pw_rule.second);
    assert_eq!('a', pw_rule.ch);
    assert_eq!(String::from("abcde"), pw_rule.password);
}

#[test]
fn test_password_first_rule_is_valid() {
    let pw_rule = password_db_parser(String::from("1-3 b: aabbbcd"));
    assert_eq!(true, pw_rule.first_rule_is_valid());
}

#[test]
fn test_password_first_rule_is_invalid_missing_character() {
    let pw_rule = password_db_parser(String::from("1-3 b: aacd"));
    assert_eq!(false, pw_rule.first_rule_is_valid());
}

#[test]
fn test_password_first_rule_is_invalid_by_max() {
    let pw_rule = password_db_parser(String::from("1-2 b: aabbbcd"));
    assert_eq!(false, pw_rule.first_rule_is_valid());
}

#[test]
fn test_password_first_rule_is_invalid_by_min() {
    let pw_rule = password_db_parser(String::from("2-3 b: aabcd"));
    assert_eq!(false, pw_rule.first_rule_is_valid());
}

#[test]
fn test_password_second_rule_is_valid() {
    let pw_rule = password_db_parser(String::from("1-3 a: abcd"));
    assert_eq!(true, pw_rule.second_rule_is_valid());
}

#[test]
fn test_password_second_rule_is_invalid_missing_char() {
    let pw_rule = password_db_parser(String::from("1-3 b: abcd"));
    assert_eq!(false, pw_rule.second_rule_is_valid());
}

#[test]
fn test_password_second_rule_is_invalid_missing_duplicated() {
    let pw_rule = password_db_parser(String::from("1-3 a: aaa"));
    assert_eq!(false, pw_rule.second_rule_is_valid());
}

use std::collections::HashSet;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use regex::Regex;

const FILEPATH: &str = "data/04/input.txt";

pub fn first_solution() {
    let passports = parse_passports();
    println!(
        "Solution: {}",
        passports.iter().filter(|p| p.first_is_valid()).count()
    );
}

pub fn second_solution() {
    let passports = parse_passports();
    println!(
        "Solution: {}",
        passports.iter().filter(|p| p.second_is_valid()).count()
    );
}

struct Height {
    value: i32,
    unit: String,
}

struct Passport {
    byr: Option<i32>,    // (Birth Year)
    iyr: Option<i32>,    // (Issue Year)
    eyr: Option<i32>,    // (Expiration Year)
    hgt: Option<Height>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<String>, // (Passport ID)
    cid: Option<i32>,    // (Country ID)
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn first_is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn second_is_valid(&self) -> bool {
        if self.byr.is_none() || self.byr.unwrap() < 1920 || self.byr.unwrap() > 2002 {
            return false;
        }
        if self.iyr.is_none() || self.iyr.unwrap() < 2010 || self.iyr.unwrap() > 2020 {
            return false;
        }
        if self.eyr.is_none() || self.eyr.unwrap() < 2020 || self.eyr.unwrap() > 2030 {
            return false;
        }
        match &self.hgt {
            Some(height) => {
                if (height.unit == "cm" && (height.value < 150 || height.value > 193))
                    || (height.unit == "in" && (height.value < 59 || height.value > 76))
                    || (height.unit != "cm" && height.unit != "in")
                {
                    return false;
                }
            }
            None => return false,
        }
        match &self.hcl {
            Some(hcl) => {
                let re = Regex::new(r"^#[a-f0-9]{6}$").expect("Invalid regex");
                if !re.is_match(hcl) {
                    return false;
                }
            }
            None => return false,
        }

        let eye_colours: HashSet<String> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
            .into_iter()
            .map(|s| s.to_string())
            .collect();
        if self.ecl.is_none() || !eye_colours.contains(self.ecl.as_ref().unwrap()) {
            return false;
        }
        match &self.pid {
            Some(value) => {
                if value.len() != 9 {
                    return false;
                }
                if value.parse::<i32>().is_err() {
                    return false;
                }
            }
            None => return false,
        }

        true
    }
}

/*
 * ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
 * byr:1937 iyr:2017 cid:147 hgt:183cm
 */
fn parse_passports() -> Vec<Passport> {
    let file = File::open(FILEPATH).expect("File not found");
    let reader = BufReader::new(file);

    let mut passports = vec![];
    let mut passport = Passport::new();
    for line in reader
        .lines()
        .map(|l| l.expect("Unable to read line from file"))
    {
        if line.is_empty() {
            passports.push(passport);
            passport = Passport::new();
            continue;
        }
        for block in line.split_whitespace().map(|t| t.trim()) {
            let token: Vec<&str> = block.split(':').collect();
            match token[0] {
                "byr" => passport.byr = Some(token[1].parse::<i32>().expect("numeric value")),
                "iyr" => passport.iyr = Some(token[1].parse::<i32>().expect("numeric value")),
                "eyr" => passport.eyr = Some(token[1].parse::<i32>().expect("numeric value")),
                "cid" => passport.cid = Some(token[1].parse::<i32>().expect("numeric value")),
                "pid" => passport.pid = Some(token[1].to_string()),
                "hgt" => {
                    let (mut value, mut unit) = token[1].split_at(token[1].len() - 2);
                    if !token[1].ends_with("cm") && !token[1].ends_with("in") {
                        value = token[1];
                        unit = "";
                    }
                    passport.hgt = Some(Height {
                        value: value.parse::<i32>().expect("numeric value"),
                        unit: unit.to_string(),
                    })
                }
                "hcl" => passport.hcl = Some(token[1].to_string()),
                "ecl" => passport.ecl = Some(token[1].to_string()),
                _ => panic!("Unexpected value"),
            }
        }
    }
    passports.push(passport);
    passports
}

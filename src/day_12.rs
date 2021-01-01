use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILEPATH: &str = "data/12/input.txt";

pub fn first_solution() {
    let instructions = read_instructions_from_file();
    println!("Solution: {}", count_distance(navigate(&instructions)));
}

pub fn second_solution() {
    let instructions = read_instructions_from_file();
    println!(
        "Solution: {}",
        count_distance(navigate_waypoint(&instructions, &mut (10, 1)))
    );
}

fn read_instructions_from_file() -> Vec<Instruction> {
    let file = File::open(FILEPATH).expect("File not found");
    let reader = BufReader::new(file);

    let instructions: Vec<String> = reader
        .lines()
        .map(|l| l.expect("expected lines from input"))
        .collect();
    parse_instructions(&instructions)
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
    Right,
    Left,
    Forward,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    value: i32,
}

fn parse_instructions(instructions: &[String]) -> Vec<Instruction> {
    instructions
        .iter()
        .map(|s| s.split_at(1))
        .map(|(d, v)| Instruction {
            direction: match d {
                "N" => Direction::North,
                "S" => Direction::South,
                "E" => Direction::East,
                "W" => Direction::West,
                "R" => Direction::Right,
                "L" => Direction::Left,
                "F" => Direction::Forward,
                _ => unreachable!(),
            },
            value: v.parse::<i32>().expect("numeric value expected"),
        })
        .collect()
}

fn navigate(instructions: &[Instruction]) -> (i32, i32) {
    let mut pos = (0, 0);
    let mut dir = 0;
    for instruction in instructions.iter() {
        match instruction.direction {
            Direction::North => pos.1 += instruction.value,
            Direction::South => pos.1 -= instruction.value,
            Direction::East => pos.0 += instruction.value,
            Direction::West => pos.0 -= instruction.value,
            Direction::Right => {
                dir -= instruction.value;
                dir += 3600;
                dir %= 360
            }
            Direction::Left => {
                dir += instruction.value;
                dir %= 360
            }
            Direction::Forward => match dir {
                0 => pos.0 += instruction.value,
                90 => pos.1 += instruction.value,
                180 => pos.0 -= instruction.value,
                270 => pos.1 -= instruction.value,
                _ => unreachable!(),
            },
        }
    }
    pos
}

fn navigate_waypoint(instructions: &[Instruction], waypoint: &mut (i32, i32)) -> (i32, i32) {
    let mut pos = (0, 0);
    for instruction in instructions.iter() {
        let x = waypoint.0 * instruction.value;
        let y = waypoint.1 * instruction.value;
        match instruction.direction {
            Direction::North => waypoint.1 += instruction.value,
            Direction::South => waypoint.1 -= instruction.value,
            Direction::East => waypoint.0 += instruction.value,
            Direction::West => waypoint.0 -= instruction.value,
            Direction::Right => match instruction.value % 360 {
                0 => {}
                90 => {
                    let (a, b) = *waypoint;
                    waypoint.0 = b;
                    waypoint.1 = -a;
                }
                180 => {
                    waypoint.0 *= -1;
                    waypoint.1 *= -1;
                }
                270 => {
                    let (a, b) = *waypoint;
                    waypoint.0 = -b;
                    waypoint.1 = a;
                }
                _ => unreachable!(),
            },
            Direction::Left => match instruction.value % 360 {
                0 => {}
                90 => {
                    let (a, b) = *waypoint;
                    waypoint.0 = -b;
                    waypoint.1 = a;
                }
                180 => {
                    waypoint.0 *= -1;
                    waypoint.1 *= -1;
                }
                270 => {
                    let (a, b) = *waypoint;
                    waypoint.0 = b;
                    waypoint.1 = -a;
                }
                _ => unreachable!(),
            },
            Direction::Forward => {
                pos.0 += x;
                pos.1 += y;
            }
        }
    }
    pos
}

fn count_distance(position: (i32, i32)) -> i32 {
    position.0.abs() + position.1.abs()
}

#[test]
fn test_count_distance() {
    let instructions = vec![
        "F10".to_string(),
        "N3".to_string(),
        "F7".to_string(),
        "R90".to_string(),
        "F11".to_string(),
    ];
    assert_eq!(
        25,
        count_distance(navigate(&parse_instructions(&instructions)))
    );
}

#[test]
fn test_count_distance_waypoint() {
    let instructions = vec![
        "F10".to_string(),
        "N3".to_string(),
        "F7".to_string(),
        "R90".to_string(),
        "F11".to_string(),
    ];
    assert_eq!(
        286,
        count_distance(navigate_waypoint(
            &parse_instructions(&instructions),
            &mut (10, 1)
        ))
    );
}

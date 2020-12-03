use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILEPATH: &str = "data/03/input.txt";

pub fn first_solution() {
    let map = create_map_from_file();
    let count = count_trees_from_slope(&map, (3, 1));
    println!("{}", count);
}

pub fn second_solution() {
    let map = create_map_from_file();
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let total = slopes
        .iter()
        .map(|s| count_trees_from_slope(&map, *s) as i64)
        .fold(1, |a, r| a * r);
    println!("{}", total);
}

fn create_map_from_file() -> Map {
    let file = File::open(FILEPATH).expect("File not found");
    let reader = BufReader::new(file);

    let mut map = Map::new();
    for line in reader.lines() {
        map.import_row(line.expect("Unable to read line from file"));
    }
    map
}

fn count_trees_from_slope(map: &Map, slope: (usize, usize)) -> i32 {
    let mut count = 0;
    for y in 1..map.height / slope.1 {
        match map.get(y * slope.0, y * slope.1) {
            Square::Tree => {
                count += 1;
            }
            _ => {}
        }
    }
    count
}

#[derive(Debug, PartialEq, Clone)]
enum Square {
    Open,
    Tree,
}

struct Map {
    height: usize,
    width: usize,
    grid: Vec<Vec<Square>>,
}

impl Map {
    fn new() -> Map {
        Map {
            height: 0,
            width: 0,
            grid: vec![],
        }
    }

    fn parse_row(row: String) -> Vec<Square> {
        row.chars()
            .map(|c| match c {
                '#' => Square::Tree,
                _ => Square::Open,
            })
            .collect()
    }

    fn import_row(&mut self, row: String) {
        self.height += 1;
        self.width = row.len();
        self.grid.push(Map::parse_row(row));
    }

    fn get(&self, x: usize, y: usize) -> Square {
        self.grid[y][x % self.width].clone()
    }
}

#[test]
fn test_parse_row() {
    let row = Map::parse_row(String::from("..#"));
    assert_eq!(Square::Open, row[0]);
    assert_eq!(Square::Open, row[1]);
    assert_eq!(Square::Tree, row[2]);
}

#[test]
fn test_map_import_row() {
    let mut map = Map::new();
    map.import_row(String::from(".#"));
    assert_eq!(1, map.height);
    assert_eq!(2, map.width);
}

#[test]
fn test_get_square() {
    let mut map = Map::new();
    map.import_row(String::from(".#"));
    map.import_row(String::from(".#"));
    assert_eq!(Square::Tree, map.get(3, 1));
}

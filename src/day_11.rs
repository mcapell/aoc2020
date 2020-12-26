use std::fs::File;
use std::io::{prelude::*, BufReader};

const FILEPATH: &str = "data/11/input.txt";

pub fn first_solution() {
    let seats = run_simulation(&read_seats_from_file());
    println!(
        "{}",
        seats
            .iter()
            .map(|r| r.iter().filter(|s| **s == SeatStatus::Occupied).count())
            .sum::<usize>()
    );
}

pub fn second_solution() {}

#[derive(Clone, Debug, PartialEq, Copy)]
enum SeatStatus {
    Floor,
    Empty,
    Occupied,
}

type SeatLayout = Vec<Vec<SeatStatus>>;

fn read_seats_from_file() -> SeatLayout {
    let file = File::open(FILEPATH).expect("File not found");
    let reader = BufReader::new(file);

    parse_layout(
        reader
            .lines()
            .map(|l| l.expect("expected lines from input"))
            .collect(),
    )
}

fn parse_layout(rows: Vec<String>) -> SeatLayout {
    rows.iter()
        .map(|row| {
            row.chars()
                .map(|ch| match ch {
                    '.' => SeatStatus::Floor,
                    '#' => SeatStatus::Occupied,
                    'L' => SeatStatus::Empty,
                    _ => unreachable!(),
                })
                .collect()
        })
        .collect()
}

fn run_simulation(seats: &SeatLayout) -> SeatLayout {
    let new_layout = &move_seats_expected(seats);
    if new_layout == seats {
        return new_layout.clone();
    }
    return run_simulation(new_layout);
}

fn move_seats_expected(seats: &SeatLayout) -> SeatLayout {
    let mut layout = vec![];
    for (y, row) in seats.iter().enumerate() {
        let mut new_row = vec![];
        for (x, seat) in row.iter().enumerate() {
            let count_occupied = count_adjacent(x as i8, y as i8, &seats);
            if *seat == SeatStatus::Empty && count_occupied == 0 {
                new_row.push(SeatStatus::Occupied);
            } else if *seat == SeatStatus::Occupied && count_occupied >= 4 {
                new_row.push(SeatStatus::Empty);
            } else {
                new_row.push(*seat);
            }
        }
        layout.push(new_row);
    }
    layout
}

fn count_adjacent(x: i8, y: i8, seats: &SeatLayout) -> usize {
    let x_max = seats[0].len() as i8;
    let y_max = seats.len() as i8;
    let positions: Vec<(i8, i8)> = vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .iter()
    .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < x_max && *y < y_max)
    .map(|t| *t)
    .collect();

    positions
        .iter()
        .filter(|(x, y)| seats[*y as usize][*x as usize] == SeatStatus::Occupied)
        .count()
}

fn move_seats_observed(seats: &SeatLayout) -> SeatLayout {
    let directions = vec![
        (-1, -1), (0, -1), (1, -1),
        (-1, 0), (1, 0),
        (-1, 1), (0, 1), (1, 1),
    ];
    seats.clone()
}

#[test]
fn test_parse_layout() {
    let rows = vec!["#.L".to_string(), "#.L".to_string()];

    let expected: SeatLayout = vec![
        vec![SeatStatus::Occupied, SeatStatus::Floor, SeatStatus::Empty],
        vec![SeatStatus::Occupied, SeatStatus::Floor, SeatStatus::Empty],
    ];

    assert_eq!(expected, parse_layout(rows));
}

#[test]
fn test_count_adjacent() {
    let seats: SeatLayout = vec![
        vec![
            SeatStatus::Occupied,
            SeatStatus::Floor,
            SeatStatus::Occupied,
            SeatStatus::Occupied,
        ],
        vec![
            SeatStatus::Occupied,
            SeatStatus::Occupied,
            SeatStatus::Occupied,
            SeatStatus::Occupied,
        ],
    ];
    assert_eq!(4, count_adjacent(2, 0, &seats));
}

#[test]
fn test_move_seats_top_row() {
    let input: SeatLayout = vec![
        vec![
            SeatStatus::Occupied,
            SeatStatus::Floor,
            SeatStatus::Occupied,
            SeatStatus::Occupied,
            SeatStatus::Floor,
        ],
        vec![
            SeatStatus::Occupied,
            SeatStatus::Occupied,
            SeatStatus::Occupied,
            SeatStatus::Occupied,
            SeatStatus::Occupied,
        ],
    ];

    let expected_top_row = vec![
        SeatStatus::Occupied,
        SeatStatus::Floor,
        SeatStatus::Empty,
        SeatStatus::Empty,
        SeatStatus::Floor,
    ];

    assert_eq!(expected_top_row, move_seats_expected(&input)[0]);
}

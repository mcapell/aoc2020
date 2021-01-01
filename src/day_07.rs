use std::collections::HashMap;
use std::fs::File;
use std::io::{prelude::*, BufReader};

use lazy_static::lazy_static;
use regex::Regex;

const FILEPATH: &str = "data/07/input.txt";

pub fn first_solution() {
    let file = File::open(FILEPATH).unwrap();
    let reader = BufReader::new(file);
    let graph = create_graph(
        reader
            .lines()
            .map(|l| l.expect("Unable to read line"))
            .collect(),
    );
    let total = graph
        .keys()
        .filter(|k| holds_shiny_gold_bag(&graph, k.to_string()))
        .count();
    println!("Solution: {}", total);
}

pub fn second_solution() {
    let file = File::open(FILEPATH).unwrap();
    let reader = BufReader::new(file);
    let graph = create_graph(
        reader
            .lines()
            .map(|l| l.expect("Unable to read line"))
            .collect(),
    );
    let total = count_individual_bags(&graph, "shiny gold".to_string()) - 1;
    println!("Solution: {}", total);
}

lazy_static! {
    static ref RE_SPLIT: Regex = Regex::new(r"^(\w+\s\w+).*contain\s(.*)").expect("Invalid regex");
    static ref RE_GROUPS: Regex =
        Regex::new(r"(\d+|no)\s(\w+\s?\w+?)\sbags?").expect("Invalid regex");
}

type BagGraph = HashMap<String, Vec<(usize, String)>>;

fn create_graph(lines: Vec<String>) -> BagGraph {
    let mut graph: BagGraph = BagGraph::new();
    for line in lines.iter() {
        let split = RE_SPLIT.captures(line).expect("Invalid Bag rule");
        for group in RE_GROUPS.captures_iter(&split[2]) {
            let num = (&group[1]).to_string().parse::<usize>().unwrap_or(0);
            match graph.get_mut(&split[1].to_string()) {
                Some(val) => {
                    val.push((num, (&group[2]).to_string()));
                }
                _ => {
                    let bags = if num == 0 {
                        vec![]
                    } else {
                        vec![(num, (&group[2]).to_string())]
                    };
                    graph.insert((&split[1]).to_string(), bags);
                }
            }
        }
    }

    graph
}

fn holds_shiny_gold_bag(graph: &BagGraph, bag: String) -> bool {
    for (_, b) in graph[&bag].iter() {
        if *b == "shiny gold".to_string() {
            return true;
        }
        if holds_shiny_gold_bag(&graph, b.clone()) {
            return true;
        }
    }
    false
}

fn count_individual_bags(graph: &BagGraph, bag: String) -> u32 {
    graph
        .get(&bag)
        .unwrap()
        .iter()
        .map(|(n, b)| count_individual_bags(&graph, b.clone()) * (*n) as u32)
        .sum::<u32>()
        + 1
}

#[test]
fn test_create_graph() {
    let lines: Vec<String> = vec![
        "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
        "bright white bags contain 1 shiny gold bag.".to_string(),
        "muted yellow bags contain no other bags.".to_string(),
    ];
    let graph = create_graph(lines);
    println!("{:?}", graph);
    assert_eq!(3, graph.len());
    let light_red: Vec<(usize, String)> = vec![
        (1, "bright white".to_string()),
        (2, "muted yellow".to_string()),
    ];
    assert_eq!(light_red, *graph.get(&"light red".to_string()).unwrap());
}

#[test]
fn test_contain_shiny_gold_bags() {
    let lines: Vec<String> = vec![
        "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
        "bright white bags contain 1 shiny gold bag.".to_string(),
        "muted yellow bags contain 9 faded blue bags, 2 shiny gold bags.".to_string(),
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
        "faded blue bags contain no other bags.".to_string(),
        "dotted black bags contain no other bags.".to_string(),
    ];
    let graph = create_graph(lines);
    assert_eq!(true, holds_shiny_gold_bag(&graph, "light red".to_string()));
    assert_eq!(
        4,
        graph
            .keys()
            .filter(|k| holds_shiny_gold_bag(&graph, k.to_string()))
            .count()
    );
}

#[test]
fn test_count_individual_bags() {
    let lines: Vec<String> = vec![
        "light red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
        "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
        "bright white bags contain 1 shiny gold bag.".to_string(),
        "muted yellow bags contain 9 faded blue bags, 2 shiny gold bags.".to_string(),
        "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
        "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
        "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
        "faded blue bags contain no other bags.".to_string(),
        "dotted black bags contain no other bags.".to_string(),
    ];
    let graph = create_graph(lines);
    assert_eq!(
        33, // shiny gold is counted here
        count_individual_bags(&graph, "shiny gold".to_string())
    );
}

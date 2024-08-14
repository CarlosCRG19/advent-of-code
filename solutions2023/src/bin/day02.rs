use std::{collections::HashMap, env, fs};

use regex::Regex;

fn part_1(input: &str) -> u32 {
    let bag: HashMap<&str, u32> = ([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]).into_iter().collect();

    let id_regex = Regex::new(r"Game (\d+): ").unwrap();
    let set_regex = Regex::new(r"(?<n>\d+) (?<color>red|blue|green)").unwrap();

    let sum: u32 = input.lines()
        .filter_map(|game| {
            let game_id = id_regex.captures(game)
                .and_then(|caps| caps.get(1)?.as_str().parse::<u32>().ok())?;

            let is_valid = set_regex.captures_iter(game)
                .all(|caps| {
                    let count = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
                    let color = caps.get(2).unwrap().as_str();
                    count <= bag[color]
                });

            is_valid.then_some(game_id)
        })
        .sum();

    sum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).expect("Error reading file");

    println!("the result for part 1 is {}", part_1(&file));
}
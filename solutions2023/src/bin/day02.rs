use std::{collections::HashMap, env, fs};

use regex::Regex;

fn part_1(input: &str) -> u32 {
    let bag: HashMap<&str, u32> = ([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]).into_iter().collect();

    let id_regex = Regex::new(r"Game (\d+): ").unwrap();
    let set_regex = Regex::new(r"(?<count>\d+) (?<color>red|blue|green)").unwrap();

    let sum: u32 = input.lines()
        .filter_map(|game| {
            let game_id = id_regex.captures(game)
                .and_then(|caps| caps.get(1)?.as_str().parse::<u32>().ok())?;

            let is_valid = set_regex.captures_iter(game)
                .all(|caps| {
                    let count = caps["count"].parse::<u32>().unwrap();
                    let color = &caps["color"];
                    count <= bag[color]
                });

            is_valid.then_some(game_id)
        })
        .sum();

    sum
}

fn part_2(input: &str) -> u32 {
    let set_regex = Regex::new(r"(?<count>\d+) (?<color>red|blue|green)").unwrap();

    input.lines()
        .map(|game| {
            let mut max_values: HashMap<String, u32> = HashMap::new();

            for caps in set_regex.captures_iter(game) {
                let count = caps["count"].parse::<u32>().unwrap();
                let color = caps["color"].to_string();

                max_values.entry(color)
                    .and_modify(|e| *e = (*e).max(count))
                    .or_insert(count);
            }

            max_values.values().product::<u32>()
        })
        .sum()
} 

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).expect("Error reading file");

    println!("the result for part 1 is {}", part_1(&file));
    println!("the result for part 2 is {}", part_2(&file));
}
use std::{collections::HashSet, env, fs};

use regex::Regex;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn parse_numbers(s: &str) -> HashSet<u32> {
    s.split_whitespace()
        .filter_map(|n| n.parse().ok())
        .collect()
}

fn get_matching_numbers_count(card: &str) -> usize {
    let card_regex = Regex::new(r"(?:\w\s+\d+:)(?<winning_numbers>[ \d]+)([|])(?<numbers_you_have>[ \d]+)").unwrap();
    let caps = card_regex.captures(card).unwrap();
    let winning_numbers = parse_numbers(&caps["winning_numbers"]);
    let numbers_you_have = parse_numbers(&caps["numbers_you_have"]);
    winning_numbers.intersection(&numbers_you_have).count()
}

fn part_1(input: &str) -> Result<u32> {
    Ok(input
        .lines()
        .map(|card| {
            let matching_count = get_matching_numbers_count(card);
            if matching_count > 0 { 1 << matching_count - 1 } else { 0 }
        })
        .sum())
}

fn part_2(input: &str) -> Result<u32> {
    let matching_numbers_by_card: Vec<_> = input
        .lines()
        .map(get_matching_numbers_count)
        .collect();

    let mut instances = 0;
    for i in 0..matching_numbers_by_card.len() {
        let mut queue = vec![i];
        while let Some(card_number) = queue.pop() {
            instances += 1;
            if let Some(matching_count) = matching_numbers_by_card.get(card_number) {
                if *matching_count > 0 {
                    queue.extend(card_number + 1..=card_number + *matching_count)
                }
            }
        }
    }

    Ok(instances)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1])?;

    println!("the result for part 1 is {}", part_1(&file)?);
    println!("the result for part 2 is {}", part_2(&file)?);

    Ok(())
}
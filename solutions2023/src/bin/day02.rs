use std::{collections::HashMap, env, fs};

use regex::Regex;

fn part_1(input: &str) -> u32 {
    let bag = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    let mut sum = 0;
    'games: for game in input.lines() {
        // extract game id
        let re = Regex::new(r"Game (\d+): ").expect("Error creating regex");
        let caps = re
            .captures(game)
            .expect("Error finding ID");
        let game_id = &caps[1].parse::<u32>().unwrap();

        // extract sets
        let re = Regex::new(r"(?<n>\d+) (?<color>red|blue|green)").unwrap();
        for (_, [n, color]) in re.captures_iter(game).map(|c| c.extract()) {
            if n.parse::<u32>().unwrap() > bag[color] {
                continue 'games
            }
        }

        sum += game_id;
    }

    sum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).expect("Error reading file");

    println!("the result for part 1 is {}", part_1(&file));
}
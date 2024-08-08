use std::{collections::HashMap, env, fs};

#[derive(Debug)]
struct Digit {
    name: &'static str,
    number: u32,
}

const DIGITS: [Digit; 9] = [
    Digit { name: "one" , number: 1},
    Digit { name: "two" , number: 2},
    Digit { name: "three" , number: 3},
    Digit { name: "four" , number: 4},
    Digit { name: "five" , number: 5},
    Digit { name: "six" , number: 6},
    Digit { name: "seven" , number: 7},
    Digit { name: "eight" , number: 8},
    Digit { name: "nine" , number: 9},
];

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1]).expect("Error reading file");

    let mut forward_digit_hash = HashMap::with_capacity(9);
    let mut backward_digit_hash = HashMap::with_capacity(9);

    for digit in &DIGITS {
        forward_digit_hash.entry(digit.name.chars().next().unwrap())
            .or_insert_with(Vec::new).push(digit);
        backward_digit_hash.entry(digit.name.chars().rev().next().unwrap())
            .or_insert_with(Vec::new).push(digit);
    }

    let sum: u32 = file.lines()
        .map(|line| {
            let first_digit = get_first_digit(line, &forward_digit_hash).expect("Error finding first digit");
            let last_digit = get_last_digit(line, &backward_digit_hash).expect("Error finding last digit");
            first_digit * 10 + last_digit
        })
        .sum();

    println!("the result is {sum}");
}

fn get_first_digit(s: &str, digit_hash: &HashMap<char, Vec<&Digit>>) -> Option<u32> {
    for (i, c) in s.chars().enumerate() {
        match c.to_digit(10) {
            Some(number) => return Some(number),
            None => {
                match digit_hash.get(&c) {
                    Some(digits) => {
                        for digit in digits.iter() {
                            let can_contain = i + digit.name.len() <= s.len();
                            if can_contain && s[i..i + digit.name.len()].contains(digit.name) {
                                return Some(digit.number);
                            }
                        }
                    },
                    None => continue
                }
            }
        }

    }

    None
}

fn get_last_digit(s: &str, digit_hash: &HashMap<char, Vec<&Digit>>) -> Option<u32> {
    let s_rev: String = s.chars().rev().collect();
    for (i, c) in s_rev.chars().enumerate() {
        match c.to_digit(10) {
            Some(number) => return Some(number),
            None => {
                match digit_hash.get(&c) {
                    Some(digits) => {
                        for digit in digits.iter() {
                            let rev_name: String = digit.name.chars().rev().collect();
                            let can_contain = i + digit.name.len() <= s_rev.len();
                            if can_contain && s_rev[i..i + digit.name.len()].contains(&rev_name) {
                                return Some(digit.number);
                            }
                        }
                    },
                    None => continue
                }
            }
        }

    }

    None
}

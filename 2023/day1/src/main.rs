use std::{collections::HashMap, fs};

fn main() {
    let file = fs::read_to_string("input.txt").expect("Error reading file");
    let digit_hash = digit_hash();
    let mut line_digits = Vec::new();

    for line in file.lines() {
        let line = line.trim();
        let digits = get_numbers(line, &digit_hash);

        let combined = digits[0] * 10 + digits[digits.len() - 1];
        line_digits.push(combined);
    }

    let sum: u32 = line_digits.iter().sum();
    println!("the result is {sum}");
}

fn get_numbers(s: &str, digit_hash: &HashMap<char, Vec<String>>) -> Vec<u32> {
    let mut numbers: Vec<u32> = Vec::new();

    println!("the digit hash is {:?}", digit_hash);

    for (i, char) in s.chars().enumerate() {
        match char.to_digit(10) {
            Some(digit) => numbers.push(digit),
            None => {
                if let Some(digits) = digit_hash.get(&char) {
                    for digit in digits {
                        if i + digit.len() <= s.len() && s[i..i+digit.len()] == digit[..] {
                            let number = match &digit[..] {
                                "one" => 1,
                                "two" => 2,
                                "three" => 3,
                                "four" => 4,
                                "five" => 5,
                                "six" => 6,
                                "seven" => 7,
                                "eight" => 8,
                                "nine" => 9,
                                _ => 0
                            };

                            numbers.push(number);
                        }
                    }
                }
            }
        }
    }

    numbers
}

const DIGITS: [&str; 9]= [
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
];

fn digit_hash() -> HashMap<char, Vec<String>> {
    let mut hash: HashMap<char, Vec<String>> = HashMap::new();

    for digit in DIGITS {
        let all_chars: Vec<char> = digit.chars().collect();
        let first_char = all_chars[0];

        if let Some(vector) = hash.get_mut(&first_char) {
            vector.push(digit.to_string());
        } else {
            hash.insert(first_char, vec![digit.to_string()]);
        }
    }

    hash
}
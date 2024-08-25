use std::{env, fs};
use regex::Regex;
use once_cell::sync::Lazy;

static NUMBER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\d]+").unwrap());
static SYMBOL_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[^\d\.]").unwrap());

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug)]
struct Position { row: i32, col: i32 }

#[derive(Debug)]
struct Number {
    value: u32,
    start: Position,
    end: Position,
}

fn is_adjacent(num: &Number, symbol_pos: &Position) -> bool {
    (symbol_pos.row - num.start.row).abs() <= 1 &&
    symbol_pos.col >= num.start.col - 1 &&
    symbol_pos.col <= num.end.col
}

fn extract_numbers_and_symbols(input: &str) -> (Vec<Number>, Vec<Position>) {
    let mut numbers: Vec<Number> = vec![];
    let mut symbol_positions: Vec<Position> = vec![];

    for (row, line) in input.lines().enumerate() {
        let row = row as i32;

        for number_match in NUMBER_REGEX.find_iter(line) {
            numbers.push(Number {
                value: number_match.as_str().parse::<u32>().unwrap(),
                start: Position {
                    row,
                    col: number_match.start() as i32
                },
                end: Position {
                    row,
                    col: number_match.end() as i32
                }
            });
        }

        for symbol_match in SYMBOL_REGEX.find_iter(line) {
            symbol_positions.push(Position {
                row,
                col: symbol_match.start() as i32
            });
        }
    }

    (numbers, symbol_positions)
}

fn part_1(input: &str) -> Result<u32> {
    let (numbers, symbol_positions) = extract_numbers_and_symbols(input);
    Ok(numbers
        .iter()
        .filter(|&number| symbol_positions.iter().any(|pos| is_adjacent(number, pos)))
        .map(|number| number.value)
        .sum())
}

fn part_2(input: &str) -> Result<u32> {
    let (numbers, symbol_positions) = extract_numbers_and_symbols(input);
    Ok(symbol_positions
        .iter()
        .filter_map(|pos| {
            let adjacent_numbers: Vec<_> = numbers
                .iter()
                .filter(|num| is_adjacent(num, pos))
                .map(|num| num.value)
                .collect();

            if adjacent_numbers.len() == 2 {
                Some(adjacent_numbers[0] * adjacent_numbers[1])
            } else {
                None
            }
        })
        .sum())
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let file = fs::read_to_string(&args[1])?;

    println!("the result for part 1 is {}", part_1(&file)?);
    println!("the result for part 2 is {}", part_2(&file)?);

    Ok(())
}
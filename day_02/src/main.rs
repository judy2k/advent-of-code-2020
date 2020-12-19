use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::{env, str::Chars};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let count = BufReader::new(File::open(filename)?)
        .lines()
        .map(|r| r.unwrap())
        .filter(is_valid_1)
        .count();

    println!("Valid Passwords: {}", count);

    let count = BufReader::new(File::open(filename)?)
        .lines()
        .map(|r| r.unwrap())
        .filter(is_valid_2)
        .count();

    println!("Properly valid Passwords: {}", count);

    Ok(())
}

fn parse_line(line: &String) -> (usize, usize, char, String) {
    let (prefix, password) = split_once(&line, ": ").unwrap();
    let (range, letter) = split_once(prefix, " ").unwrap();
    let (lower, upper) = split_once(range, "-").unwrap();
    let letter = letter.chars().next().unwrap();
    let lower = lower.parse::<usize>().unwrap();
    let upper = upper.parse::<usize>().unwrap();

    (lower, upper, letter, password.to_owned())
}

fn is_valid_1(line: &String) -> bool {
    let (lower, upper, letter, password) = parse_line(line);

    let letter_count = password.chars().filter(|c| c == &letter).count();

    lower <= letter_count && upper >= letter_count
}

fn is_valid_2(line: &String) -> bool {
    let (lower, upper, letter, password) = parse_line(line);
    let password_chars = password.chars().collect::<Vec<_>>();
    let lower_match = password_chars[lower - 1] == letter;
    let upper_match = password_chars[upper - 1] == letter;
    (lower_match || upper_match) && !(lower_match && upper_match)
}

fn split_once<'a>(source: &'a str, pat: &str) -> Option<(&'a str, &'a str)> {
    if let Some(index) = source.find(pat) {
        return Some((&source[..index], &source[index + pat.len()..]));
    }
    None
}

use std::collections::{HashMap, HashSet};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut complete_count = 0;
    let mut valid_count = 0;
    let mut passport = HashMap::new();
    for line in BufReader::new(File::open(filename)?)
        .lines()
        .map(|r| r.unwrap())
    {
        if !parse_pairs(&line, &mut passport) {
            if test_passport_complete(&passport) {
                complete_count += 1;
                if test_passport_valid(&passport) {
                    valid_count += 1;
                }
            }
            passport.clear();
        }
    }
    if test_passport_complete(&passport) {
        complete_count += 1;
        if test_passport_valid(&passport) {
            valid_count += 1;
        }
    }

    println!("Complete passports: {}", complete_count);
    println!("Valid passports: {}", valid_count);

    Ok(())
}

fn parse_pairs(line: &String, map: &mut HashMap<String, String>) -> bool {
    if line == "" {
        return false;
    }
    for pair in line.split(" ") {
        let (key, value) = split_once(pair, ":").unwrap();
        map.insert(key.to_owned(), value.to_owned());
    }
    return true;
}

fn split_once<'a>(source: &'a str, pat: &str) -> Option<(&'a str, &'a str)> {
    if let Some(index) = source.find(pat) {
        return Some((&source[..index], &source[index + pat.len()..]));
    }
    None
}

fn test_passport_complete(passport: &HashMap<String, String>) -> bool {
    let expected_keys: HashSet<_> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid", "cid"]
        .into_iter()
        .collect();
    let actual_keys: HashSet<&str> = passport.keys().map(|s| s.as_ref()).collect();
    let missing_keys: Vec<&&str> = expected_keys.difference(&actual_keys).collect();
    if missing_keys.len() > 1 || missing_keys.len() == 1 && **missing_keys.get(0).unwrap() != "cid"
    {
        return false;
    }
    true
}

fn test_passport_valid(passport: &HashMap<String, String>) -> bool {
    if !validate_year(passport.get(&"byr".to_owned()).unwrap(), 1920, 2002) {
        return false;
    }
    if !validate_year(passport.get(&"iyr".to_owned()).unwrap(), 2010, 2020) {
        return false;
    }
    if !validate_year(passport.get(&"eyr".to_owned()).unwrap(), 2020, 2030) {
        return false;
    }
    if !validate_height(passport.get(&"hgt".to_owned()).unwrap()) {
        return false;
    }
    if !validate_hair_color(passport.get(&"hcl".to_owned()).unwrap()) {
        return false;
    }
    if !validate_eye_color(passport.get(&"ecl".to_owned()).unwrap()) {
        return false;
    }
    if !validate_passport_number(passport.get(&"pid".to_owned()).unwrap()) {
        return false;
    }

    true
}

fn validate_year(value: &str, lower: i32, upper: i32) -> bool {
    let value = value.parse::<i32>().unwrap_or(-1);
    value >= lower && value <= upper
}

fn validate_height(s: &str) -> bool {
    let unit = &s[s.len() - 2..];
    let measure = s[..s.len() - 2].parse::<i32>().unwrap_or(-1);
    if unit == "cm" {
        return measure >= 150 && measure <= 193;
    } else if unit == "in" {
        return measure >= 59 && measure <= 76;
    }
    false
}

fn validate_hair_color(s: &str) -> bool {
    let valid_chars: HashSet<_> = vec![
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ]
    .into_iter()
    .collect();
    let chars: Vec<_> = s.chars().collect();

    if chars.len() != 7 {
        return false;
    }

    if chars.get(0) != Some(&'#') {
        return false;
    }
    for c in chars[1..].iter() {
        if !valid_chars.contains(c) {
            return false;
        }
    }

    true
}

fn validate_eye_color(s: &str) -> bool {
    let valid_values: HashSet<_> = vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"]
        .into_iter()
        .collect();
    valid_values.contains(s)
}

fn validate_passport_number(s: &str) -> bool {
    let valid_chars: HashSet<_> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9']
        .into_iter()
        .collect();
    let chars: Vec<_> = s.chars().collect();

    if chars.len() != 9 {
        return false;
    }

    for c in chars.iter() {
        if !valid_chars.contains(c) {
            return false;
        }
    }

    true
}

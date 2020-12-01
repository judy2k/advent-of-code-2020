use itertools::Itertools;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let ints = BufReader::new(File::open(filename)?)
        .lines()
        .map(|r| r.unwrap().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    for (a, b) in ints.iter().tuple_combinations() {
        if a + b == 2020 {
            println!("Answer 1: {}", a * b);
            break;
        }
    }

    for (a, b, c) in ints.iter().tuple_combinations() {
        if a + b + c == 2020 {
            println!("Answer 2: {}", a * b * c);
            break;
        }
    }

    Ok(())
}

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // Part 1:
    let mut count = 0;
    let mut current_pos = 0;
    for line in BufReader::new(File::open(filename)?)
        .lines()
        .map(|r| r.unwrap().chars().collect::<Vec<char>>())
    {
        // println("Row: {}, Pos: {}, ")
        if line[current_pos] == '#' {
            count += 1;
        }
        current_pos = (current_pos + 3) % line.len();
    }
    println!("Count: {}", count);

    // Part 2:
    let mut counts = vec![0, 0, 0, 0, 0];
    let mut poss = vec![0, 0, 0, 0, 0];
    let diffs = vec![1, 3, 5, 7, 1];

    for (line_no, line) in BufReader::new(File::open(filename)?)
        .lines()
        .map(|r| r.unwrap().chars().collect::<Vec<char>>())
        .enumerate()
    {
        for i in 0..counts.len() {
            if i < counts.len() - 1 || line_no % 2 == 0 {
                if line[poss[i]] == '#' {
                    counts[i] += 1;
                }

                poss[i] = (poss[i] + diffs[i]) % line.len();
            }
        }
    }
    println!("Sum: {}", counts.iter().fold(1, |acc, i| acc * i));

    Ok(())
}

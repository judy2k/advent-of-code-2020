use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let mut tally: usize = 0;
    let mut tally2: usize = 0;
    let mut shared_answers: Option<HashSet<char>> = None;
    let mut yes_answers = HashSet::new();
    for line in BufReader::new(File::open(filename)?)
        .lines()
        .map(|r| r.unwrap())
    {
        if line.len() > 0 {
            for c in line.chars() {
                yes_answers.insert(c);
            }

            let ys: HashSet<char> = line.chars().collect();
            shared_answers = Some(match shared_answers {
                None => ys,
                Some(sa) => sa.intersection(&ys).map(|r| r.to_owned()).collect(),
            });
        } else {
            let sa = &shared_answers.unwrap();
            tally += yes_answers.len();
            tally2 += &sa.len();
            yes_answers.clear();
            shared_answers = None;
        }
    }

    tally += yes_answers.len();
    tally2 += &shared_answers.unwrap().len();

    println!("Yes Sum: {}", tally);
    println!("Shared Sum: {}", tally2);

    Ok(())
}

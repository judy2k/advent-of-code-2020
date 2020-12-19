use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::{collections::HashSet, str::Chars};

struct Range {
    lower: u32,
    upper: u32,
}

impl Range {
    fn rows() -> Self {
        Self {
            lower: 0,
            upper: 127,
        }
    }

    fn cols() -> Self {
        Self { lower: 0, upper: 7 }
    }

    fn partition_lower(&self) -> Self {
        Self {
            lower: self.lower,
            upper: (self.upper - self.lower) / 2 + self.lower,
        }
    }

    fn partition_upper(&self) -> Self {
        Self {
            lower: (self.upper - self.lower + 1) / 2 + self.lower,
            upper: self.upper,
        }
    }

    fn partition(&self, letter: char) -> Self {
        if letter == 'F' || letter == 'L' {
            return self.partition_lower();
        } else if letter == 'B' || letter == 'R' {
            return self.partition_upper();
        }
        panic!("Invalid letter passed to partition");
    }

    fn solve(self, cs: Chars) -> u32 {
        let mut result = self;
        for c in cs {
            result = result.partition(c)
        }
        if result.lower == result.upper {
            return result.lower;
        }
        panic!(
            "Lower {} and upper {} did not match!",
            &result.lower, &result.upper,
        );
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let ids: Vec<u32> = BufReader::new(File::open(filename)?)
        .lines()
        .map(|r| seat_id(r.unwrap().as_ref()))
        .collect();
    let id_set: HashSet<_> = ids.iter().collect();
    let max_seat_id = ids.iter().max().unwrap().to_owned();
    println!("Max Seat ID: {}", max_seat_id);
    let min_seat_id = ids.iter().min().unwrap().to_owned();

    for i in min_seat_id..max_seat_id {
        if !id_set.contains(&i) {
            println!("My Seat ID: {}", i);
            break;
        }
    }

    Ok(())
}

fn seat_id(s: &str) -> u32 {
    return Range::rows().solve(s[..s.len() - 3].chars()) * 8
        + Range::cols().solve(s[s.len() - 3..].chars());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn partition_lower() {
        let r = Range {
            lower: 0,
            upper: 127,
        }
        .partition_lower();
        assert_eq!(r.lower, 0);
        assert_eq!(r.upper, 63);

        let r = Range {
            lower: 32,
            upper: 63,
        }
        .partition_lower();
        assert_eq!(r.lower, 32);
        assert_eq!(r.upper, 47);

        let r = Range {
            lower: 44,
            upper: 47,
        }
        .partition_lower();
        assert_eq!(r.lower, 44);
        assert_eq!(r.upper, 45);

        let r = Range {
            lower: 44,
            upper: 45,
        }
        .partition_lower();
        assert_eq!(r.lower, 44);
        assert_eq!(r.upper, 44);
    }

    #[test]
    fn partition_upper() {
        let r = Range {
            lower: 0,
            upper: 63,
        }
        .partition_upper();
        assert_eq!(r.lower, 32);
        assert_eq!(r.upper, 63);

        let r = Range {
            lower: 32,
            upper: 47,
        }
        .partition_upper();
        assert_eq!(r.lower, 40);
        assert_eq!(r.upper, 47);

        let r = Range {
            lower: 40,
            upper: 47,
        }
        .partition_upper();
        assert_eq!(r.lower, 44);
        assert_eq!(r.upper, 47);
    }

    #[test]
    fn solve() {
        assert_eq!(
            Range {
                lower: 0,
                upper: 127
            }
            .solve("FBFBBFF".chars()),
            44
        );

        assert_eq!(Range { lower: 0, upper: 7 }.solve("RLR".chars()), 5);
    }

    #[test]
    fn test_seat_id() {
        assert_eq!(seat_id("FBFBBFFRLR"), 357);
    }
}

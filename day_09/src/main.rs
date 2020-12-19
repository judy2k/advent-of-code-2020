use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::ops::Add;
use std::{collections::VecDeque, error::Error};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let preamble_length: usize = args.get(2).map_or(Ok(25), |s| s.parse())?;

    let program: Vec<i64> = BufReader::new(File::open(filename)?)
        .lines()
        .map(|r| r.unwrap().parse())
        .collect::<Result<Vec<_>, _>>()?;

    let answer_1 = solve_01(&program, preamble_length);
    println!("Answer 1: {}", answer_1);
    println!("Answer 2: {}", solve_02(&program, answer_1)?);

    Ok(())
}

fn find_pair<'a, T: 'a>(seq: impl IntoIterator<Item = &'a T>, sum: T) -> bool
where
    T: Add<Output = T> + Ord + Eq + Copy,
{
    let mut sorted: Vec<T> = seq.into_iter().copied().collect();
    sorted.sort_unstable();
    sorted.dedup();

    let mut lower = 0;
    let mut upper = sorted.len() - 1;

    // Optimization: The following could actually bisect recursively.
    while lower < upper {
        let this_sum = sorted[lower] + sorted[upper];
        if this_sum == sum {
            return true;
        } else if this_sum < sum {
            lower += 1;
        } else {
            upper -= 1;
        }
    }
    false
}

fn solve_01(program: &Vec<i64>, preamble_length: usize) -> i64 {
    let mut preamble: VecDeque<i64> = program[0..preamble_length].iter().copied().collect();

    for i in &program[preamble_length..] {
        if !find_pair(&preamble, *i) {
            return *i;
        }
        preamble.pop_front();
        preamble.push_back(*i);
    }

    panic!("No matching numbers in sequence!");
}

fn solve_02(program: &[i64], total: i64) -> Result<i64, String> {
    let mut numbers: Vec<i64> = Vec::new();
    let mut tally: i64;
    for start in 0..program.len() - 1 {
        numbers.clear();
        tally = 0;
        for i in &program[start..] {
            tally += i;
            numbers.push(*i);
            if tally == total {
                numbers.sort();
                return Ok(numbers[0] + numbers[numbers.len() - 1]);
            } else if tally > total {
                break;
            }
        }
    }
    return Err("No matching range found.".to_string());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_find_pair() {
        for seq in vec![
            &[1, 2, 3, 4, 5],
            &[5, 4, 3, 2, 1],
            &[5, 2, 3, 4, 1],
            &[1, 3, 2, 4, 5],
            &[1, 3, 4, 2, 5],
            &[2, 3, 1, 4, 5],
            &[4, 3, 1, 2, 5],
            &[3, 2, 1, 4, 5],
        ] {
            assert_eq!(find_pair(seq, 10), false);
            assert!(find_pair(seq, 9));
            assert!(find_pair(seq, 8));
            assert!(find_pair(seq, 7));
            assert!(find_pair(seq, 6));
            assert!(find_pair(seq, 5));
            assert!(find_pair(seq, 4));
            assert!(find_pair(seq, 3));
            assert_eq!(find_pair(seq, 2), false);
            assert_eq!(find_pair(seq, 1), false);
        }

        assert_eq!(find_pair(&[5, 5, 1, 5, 5], 10), false);
        assert_eq!(find_pair(&[5, 5, 1, 5, 5], 6), true);
    }
}

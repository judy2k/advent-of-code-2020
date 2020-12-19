use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug, Clone, Copy)]
enum Op {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

impl Op {
    fn patched(&self) -> Op {
        match self {
            Op::Jmp(i) => Op::Nop(i.to_owned()),
            Op::Nop(i) => Op::Jmp(i.to_owned()),
            o => o.clone(),
        }
    }
}

impl From<String> for Op {
    fn from(s: String) -> Self {
        let parts: Vec<String> = s.split(" ").map(str::to_owned).collect();
        match parts[0].as_ref() {
            "acc" => Self::Acc(parts[1].parse::<isize>().unwrap()),
            "jmp" => Self::Jmp(parts[1].parse::<isize>().unwrap()),
            "nop" => Self::Nop(parts[1].parse::<isize>().unwrap()),
            _ => panic!(),
        }
    }
}

struct Process {
    program: Vec<Op>,
    visited: Vec<bool>,
    ptr: usize,
    acc: isize,
    patched: Option<usize>,
}

impl Process {
    fn new(program: Vec<Op>) -> Self {
        let program_len = program.len();
        Self {
            program: program,
            visited: vec![false; program_len],
            ptr: 0,
            acc: 0,
            patched: None,
        }
    }

    fn init(&mut self) {
        self.visited = vec![false; self.program.len()];
        self.ptr = 0;
        self.acc = 0;
        self.patched = None;
    }

    fn patch(&mut self, idx: usize) {
        self.patched = Some(idx);
    }

    fn op(&self, idx: usize) -> Op {
        match self.patched {
            Some(patched_idx) if patched_idx == idx => self.program[idx].patched(),
            _ => self.program[idx].clone(),
        }
    }

    fn step(&mut self) -> Option<isize> {
        let op = self.op(self.ptr);

        self.visited[self.ptr] = true;

        match op {
            Op::Acc(amount) => {
                self.acc += amount;
                self.ptr += 1;
            }
            Op::Jmp(rel) => {
                self.ptr = (self.ptr as isize + rel) as usize;
            }
            Op::Nop(_) => self.ptr += 1,
        };

        None
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let program = BufReader::new(File::open(filename)?)
        .lines()
        .map(|r| r.unwrap().into())
        .collect::<Vec<Op>>();
    let mut process = Process::new(program);

    println!("Answer 1: {}", solve_01(&mut process));
    println!("Answer 2: {}", solve_02(&mut process));

    Ok(())
}

fn solve_01(process: &mut Process) -> isize {
    loop {
        process.step();
        if process.visited[process.ptr] {
            return process.acc;
        }
    }
}

fn solve_02(process: &mut Process) -> isize {
    // Part 2:
    let program_len = process.program.len();
    for patched_idx in 0..program_len {
        process.init();
        process.patch(patched_idx);
        loop {
            process.step();

            if process.ptr == program_len {
                return process.acc;
            } else if process.ptr > program_len || process.visited[process.ptr] {
                break;
            }
        }
    }
    panic!("No solution found!");
}

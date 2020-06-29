#[macro_use]
extern crate text_io;
use std::env;
use std::fs;

#[derive(Debug)]
pub enum Instruction {
    Left,
    Right,
    Increment,
    Decrement,
    Write,
    Read,
    LoopStart,
    LoopEnd,
    Unknown,
}

impl From<char> for Instruction {
    fn from(data: char) -> Self {
        match data {
            '<' => Instruction::Left,
            '>' => Instruction::Right,
            '+' => Instruction::Increment,
            '-' => Instruction::Decrement,
            '.' => Instruction::Write,
            ',' => Instruction::Read,
            '[' => Instruction::LoopStart,
            ']' => Instruction::LoopEnd,
            _ => Instruction::Unknown,
        }
    }
}

enum EvalResult {
    Exec,
    Begin,
    End,
}

fn eval(data: Instruction, ptr: &mut usize, memory: &mut Vec<isize>) -> EvalResult {
    if *ptr == memory.len() {
        memory.push(0);
    }
    match data {
        Instruction::Left => *ptr -= 1,
        Instruction::Right => *ptr += 1,
        Instruction::Increment => memory[*ptr] += 1,
        Instruction::Decrement => memory[*ptr] -= 1,
        Instruction::Read => memory[*ptr] = read!("{}"),
        Instruction::Write => print!("{}", std::char::from_u32(memory[*ptr] as u32).unwrap()),
        Instruction::LoopStart => return EvalResult::Begin,
        Instruction::LoopEnd => return EvalResult::End,
        _ => {}
    }
    EvalResult::Exec
}

fn find_next(idx: &mut usize, data: &[char]) -> usize {
    let mut end = *idx + 1;
    let mut bal: usize = 1;
    loop {
        if data[end] == '[' {
            bal += 1;
        }
        if data[end] == ']' {
            bal -= 1;
            if bal == 0 {
                return end;
            }
        }
        end += 1;
        if end == data.len() {
            return 0;
        }
    }
}

fn find_prev(idx: &mut usize, data: &[char]) -> usize {
    let mut end = *idx - 1;
    let mut bal: usize = 1;
    loop {
        if data[end] == ']' {
            bal += 1;
        }
        if data[end] == '[' {
            bal -= 1;
            if bal == 0 {
                return end;
            }
        }
        end -= 1;
        if end == data.len() {
            return 0;
        }
    }
}

fn process(
    depth: &mut usize,
    idx: &mut usize,
    data: &[char],
    ptr: &mut usize,
    memory: &mut Vec<isize>,
) {
    *depth += 1;
    loop {
        if *idx == data.len() {
            break;
        }
        let cur = Instruction::from(data[*idx]);
        match eval(cur, ptr, memory) {
            EvalResult::Begin => {
                if memory[*ptr] == 0 {
                    *idx = find_next(idx, data) + 1;
                    continue;
                }
            }
            EvalResult::End => {
                if memory[*ptr] == 0 {
                    *idx += 1;
                    continue;
                } else {
                    *idx = find_prev(idx, data) + 1;
                    continue;
                }
            }
            EvalResult::Exec => {}
        }
        *idx += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Not enough arguments!");
        std::process::exit(1);
    }
    let data = fs::read_to_string(&args[1])
        .unwrap()
        .replace(" ", "")
        .replace("\n", "")
        .replace("\t", "");
    let data: Vec<char> = data.chars().collect();
    let mut idx: usize = 0;
    let mut memory: Vec<isize> = vec![0; 8];
    let mut ptr: usize = 0;
    let mut depth: usize = 0;
    process(&mut depth, &mut idx, &data[..], &mut ptr, &mut memory);
}

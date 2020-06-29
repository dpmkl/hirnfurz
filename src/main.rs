#[macro_use]
extern crate text_io;
use std::env;
use std::fmt::Display;
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

fn eval(data: Instruction, ptr: &mut usize, memory: &mut Vec<u8>) -> EvalResult {
    if *ptr == memory.len() {
        memory.push(0);
    }
    match data {
        Instruction::Left => *ptr -= 1,
        Instruction::Right => *ptr += 1,
        Instruction::Increment => memory[*ptr] += 1,
        Instruction::Decrement => memory[*ptr] -= 1,
        Instruction::Read => memory[*ptr] = read!("{}"),
        Instruction::Write => print!("{}", memory[*ptr] as char),
        Instruction::LoopStart => return EvalResult::Begin,
        Instruction::LoopEnd => return EvalResult::End,
        _ => {}
    }
    EvalResult::Exec
}

fn process(
    depth: &mut usize,
    idx: &mut usize,
    data: &[char],
    ptr: &mut usize,
    memory: &mut Vec<u8>,
) {
    let start: usize = *idx;
    *depth += 1;

    'proc: loop {
        let cur = Instruction::from(data[*idx]);
        match eval(cur, ptr, memory) {
            EvalResult::Begin => {
                if memory[*ptr] == 0 {
                    break 'proc;
                }
                *idx += 1;
                process(depth, idx, data, ptr, memory);
            }
            EvalResult::End => {
                if memory[*ptr] == 0 {
                    break 'proc;
                } else {
                    *idx = start;
                    continue;
                }
            }
            EvalResult::Exec => {}
        }
        *idx += 1;
        if *idx == data.len() {
            break;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Not enough arguments!");
        std::process::exit(1);
    }
    let data = fs::read_to_string(&args[1]).unwrap();
    let data: Vec<char> = data.chars().collect();
    let mut idx: usize = 0;
    let mut memory: Vec<u8> = vec![0; 8];
    let mut ptr: usize = 0;
    let mut depth: usize = 0;
    process(&mut depth, &mut idx, &data[..], &mut ptr, &mut memory);
}

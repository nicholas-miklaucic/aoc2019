use std::fs::File;
use std::io::prelude::*;
use std::io;
use std::path::Path;
use std::convert::{TryFrom, TryInto};

fn read_int() -> isize {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    
    let trimmed = input_text.trim();
    match trimmed.parse::<isize>() {
        Ok(i) => i,
        Err(..) => panic!("this was not an integer: {}", trimmed),
    }
}

fn get_digits(n: isize) -> Vec<isize> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
        .collect()
}

fn get_5_digits(n: isize) -> Vec<isize> {
    let mut digits = get_digits(n);
    while digits.len() < 5 {
        digits.insert(0, 0);
    }
    digits
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum ParameterMode {
    Position,
    Immediate,
}

impl ParameterMode {
    fn value(&self) -> isize {
        match *self {
            ParameterMode::Position => 0,
            ParameterMode::Immediate => 1,
        }
    }
}

impl TryFrom<isize> for ParameterMode {
    type Error = &'static str;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            _ => Err("Invalid parameter mode")
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum Opcode {
    Add,
    Multiply,
    Input,
    Output,
    JumpTrue,
    JumpFalse,
    LessThan,
    Equals,
    Halt,
}

impl Opcode {
    fn value(&self) -> isize {
        match *self {
            Opcode::Add => 1,
            Opcode::Multiply => 2,
            Opcode::Input => 3,
            Opcode::Output => 4,
            Opcode::JumpTrue => 5,
            Opcode::JumpFalse => 6,
            Opcode::LessThan => 7,
            Opcode::Equals => 8,
            Opcode::Halt => 99,
        }
    }
    // Gets the number of input parameters (defined as ones that can use either parameter mode)
    fn num_inputs(&self) -> usize {
        match *self {
            Opcode::Add => 2,
            Opcode::Multiply => 2,
            Opcode::Input => 0,
            Opcode::Output => 0,
            Opcode::JumpTrue => 2,
            Opcode::JumpFalse => 2,
            Opcode::LessThan => 2,
            Opcode::Equals => 2,
            Opcode::Halt => 0,
        }
    }
}

impl TryFrom<isize> for Opcode {
    type Error = String;

    fn try_from(value: isize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Opcode::Add),
            2 => Ok(Opcode::Multiply),
            3 => Ok(Opcode::Input),
            4 => Ok(Opcode::Output),
            5 => Ok(Opcode::JumpTrue),
            6 => Ok(Opcode::JumpFalse),
            7 => Ok(Opcode::LessThan),
            8 => Ok(Opcode::Equals),
            99 => Ok(Opcode::Halt),
            _ => Err(format!("Invalid opcode {}", value)),
        }
    }
}

// Gets the modes for the input parameters and the opcode
fn parse_opcode(n: isize) -> (Opcode, Vec<ParameterMode>) {
    let opcode: Opcode = (n % 100).try_into().unwrap();
    let mut modes = vec![];
    for digit in get_5_digits(n).drain(..3).rev() {
        modes.push(ParameterMode::try_from(digit).unwrap());
    }
    (opcode, modes)
}

// If position mode, reads the given position from memory. If immediate mode, reads in the value
// directly. 
fn read_value(mode: ParameterMode, value: isize, memory: &Vec<isize>) -> isize {
    match mode {
        ParameterMode::Position => memory[usize::try_from(value).unwrap()],
        ParameterMode::Immediate => value,
    }
}

// Executes the program, taking values from the program stack and writing to memory and stdout as
// appropriate.
fn run(mut program: Vec<isize>) {
    let mut curr_ind: usize = 0;
    while curr_ind < program.len() {
        let (opcode, modes) = parse_opcode(program[curr_ind]);
        curr_ind += 1;
        let mut inputs = vec![];
        for i in 0..opcode.num_inputs() {
            inputs.push(read_value(modes[i], program[curr_ind], &program));
            curr_ind += 1;
        }
        // we clone memory here because we can't write values at the same time we read them
        let memory = program.clone();
        match opcode {
            Opcode::Add => {
                // next value must be position mode, write the output to it
                program[usize::try_from(memory[curr_ind]).unwrap()] = inputs[0] + inputs[1];
                curr_ind += 1;
            },
            Opcode::Multiply => {
                // next value must be position mode, write the output to it
                program[usize::try_from(memory[curr_ind]).unwrap()] = inputs[0] * inputs[1];
                curr_ind += 1;
            },
            Opcode::Input => {
                // read in value from standard input, write to position
                program[usize::try_from(memory[curr_ind]).unwrap()] = read_int();
                curr_ind += 1;
            }
            Opcode::Output => {
                // read in value at position given
                println!("{}", program[usize::try_from(memory[curr_ind]).unwrap()]);
                curr_ind += 1;
            },
            Opcode::JumpTrue => {
                if inputs[0] != 0 {
                    curr_ind = usize::try_from(inputs[1]).unwrap();
                }
            }
            Opcode::JumpFalse => {
                if inputs[0] == 0 {
                    curr_ind = usize::try_from(inputs[1]).unwrap();
                }
            }
            Opcode::LessThan => {
                let val = if inputs[0] < inputs[1] { 1 } else { 0 };
                program[usize::try_from(memory[curr_ind]).unwrap()] = val;
                curr_ind += 1;
            }
            Opcode::Equals => {
                let val = if inputs[0] == inputs[1] { 1 } else { 0 };
                program[usize::try_from(memory[curr_ind]).unwrap()] = val;
                curr_ind += 1;
            }
            Opcode::Halt => {
                // halt execution
                return;
            },
        }
    }
}

fn main() {
    let path = Path::new("input");

    let mut file = File::open(&path).unwrap();
    let mut filestr: String = String::new();
    file.read_to_string(&mut filestr).unwrap();

    let program: Vec<isize> = filestr.trim_end()
        .split(',')
        .map(|x| x.parse().unwrap()).collect();

    run(program);
}

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

/// Runs the program with the given two values at positions 1 and 2, returning the value at address
/// 0 after the program halts.
fn run(mut code: Vec<usize>, noun: usize, verb: usize) -> usize {
    code[1] = noun;
    code[2] = verb;
    let mut curr_pos: usize = 0;
    while curr_pos + 3 < code.len() {
        let opcode = code[curr_pos];
        let in1 = code[curr_pos + 1];
        let in2 = code[curr_pos + 2];
        let out = code[curr_pos + 3];
        if opcode == 1 {
            code[out] = code[in1] + code[in2];
        } else if opcode == 2 {
            code[out] = code[in1] * code[in2];
        } else if opcode == 99 {
            break;
        } else {
            panic!("Invalid opcode!");
        }
        curr_pos += 4;
    }
    code[0]
}

fn main() {
    let path = Path::new("input");

    let mut file = File::open(&path).unwrap();
    let mut filestr: String = String::new();
    file.read_to_string(&mut filestr).unwrap();

    let code: Vec<usize> = filestr.trim_end().split(',').map(|x| x.parse().unwrap()).collect();
    println!("The answer to part 1 is {}", run(code.clone(), 12, 2));

    let part2_target = 19690720;
    for noun in 0..100 {
        for verb in 0..100 {
            let result = run(code.clone(), noun, verb);
            if result == part2_target {
                println!("The answer to part 2 is {}", 100 * noun + verb);
            } else {
                // println!("{} {} => {}", noun, verb, result);
            }
        }
    }
}

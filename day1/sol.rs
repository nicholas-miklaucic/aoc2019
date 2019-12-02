use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

/// Given a mass, gets the fuel required to carry this mass, the fuel required to carry that fuel,
/// and so on.
fn compute_part2_fuel(mass: u64) -> u64 {
    let mut fuel: u64 = 0;
    let mut curr_fuel: u64;
    let mut curr_mass = mass;
    // 9 is the smallest mass that requires fuel
    while curr_mass >= 9 {
        curr_fuel = (curr_mass / 3) - 2;
        fuel += curr_fuel;
        curr_mass = curr_fuel;
    }
    fuel
}

fn main() {
    let path = Path::new("input");

    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut masses: Vec<u64> = vec![];
    for l in lines {
        masses.push(l.unwrap().parse().unwrap());
    }
    let part1_fuel: u64 = masses.iter().map(|x| (x / 3) - 2).sum();
    let part2_fuel: u64 = masses.iter().map(|&x| compute_part2_fuel(x)).sum();

    println!("The total fuel required in part 1 is {}\n", part1_fuel);

    println!("The total fuel required in part 2 is {}\n", part2_fuel);
}

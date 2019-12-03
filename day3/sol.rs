use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::str::FromStr;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl FromStr for Direction {
    type Err = std::io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            "L" => Ok(Direction::Left),
            "R" => Ok(Direction::Right),
            _ => Err(std::io::Error::new(std::io::ErrorKind::Other, "RIP")),
        }
    }
}

struct Movement {
    dir: Direction,
    length: i64,
}

impl FromStr for Movement {
    type Err = std::io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (dir, len) = input.split_at(1);
        Ok(Movement{
            dir: dir.parse().unwrap(),
            length: len.parse().unwrap(),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i64,
    y: i64
}

impl Point {
    fn go_in_dir(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
    }

    fn norm(&self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

fn get_point_set(wire: &Vec<Movement>) -> HashSet<Point> {
    let mut curr_point = Point{x: 0, y: 0};
    let mut points = HashSet::new();
    for path in wire {
        for _step in 0..path.length {
            curr_point.go_in_dir(&path.dir);
            points.insert(curr_point.clone());
        }
    }
    points
}

fn get_point_steps_map(wire: &Vec<Movement>) -> HashMap<Point, usize> {
    let mut curr_point = Point{x: 0, y: 0};
    let mut points = HashMap::new();
    let mut curr_ind = 0;
    for path in wire {
        for _step in 0..path.length {
            curr_ind += 1;
            curr_point.go_in_dir(&path.dir);
            points.insert(curr_point.clone(), curr_ind);
        }
    }
    points
}

fn closest_crossing_dist(wire1: &Vec<Movement>, wire2: &Vec<Movement>) -> usize {
    let pts1 = get_point_set(wire1);
    let pts2 = get_point_set(wire2);
    pts1.into_iter().filter(
        |p| pts2.contains(p)
    ).map(
        |p| p.norm()
    ).min().unwrap().try_into().unwrap()
}

fn least_interference_crossing_dist(wire1: &Vec<Movement>, wire2: &Vec<Movement>) -> usize {
    let pts1 = get_point_steps_map(wire1);
    let pts2 = get_point_steps_map(wire2);
    pts1.into_iter().filter(
        |(p, _v)| pts2.contains_key(p)
    ).map(|(p, v)| v + pts2.get(&p).unwrap()
    ).min().unwrap().try_into().unwrap()
}

fn main() {
    let path = Path::new("input");

    let mut file = File::open(&path).unwrap();
    let mut filestr: String = String::new();
    file.read_to_string(&mut filestr).unwrap();

    let mut wirestrs = filestr.trim_end().split('\n');
    let wire1_str = wirestrs.next().unwrap();
    let wire2_str = wirestrs.next().unwrap();
    let wire1: Vec<Movement> = wire1_str.split(',').map(|x| x.parse().unwrap()).collect();
    let wire2: Vec<Movement> = wire2_str.split(',').map(|x| x.parse().unwrap()).collect();

    println!("The answer for part 1 is {}", closest_crossing_dist(&wire1, &wire2));
    println!("The answer for part 2 is {}", least_interference_crossing_dist(&wire1, &wire2));
}

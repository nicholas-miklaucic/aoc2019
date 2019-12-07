use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::collections::HashMap;
use std::convert::TryInto;

// Gets the total number of direct and indirect orbits: the sum total of the length of the path to
// COM in the DAG of orbits.
fn total_orbits(orbits: &HashMap<String, String>) -> u64 {
    let mut num_orbits = HashMap::new();
    num_orbits.insert("COM".to_string(), 0_u64);
    let num_to_check = orbits.len();
    let mut curr_nodes_tracked = 0;
    while curr_nodes_tracked < num_to_check {
        for (key, val) in orbits.iter() {
            // if orbiting planet not in current checked nodes but orbited planet is, update
            if !num_orbits.contains_key(key) && num_orbits.contains_key(val) {
                num_orbits.insert(key.to_string(), num_orbits.get(val).unwrap() + 1);
                curr_nodes_tracked += 1;
            }
        }
    }
    num_orbits.iter().map(|(_k, v)| v).sum()
}

/// Gets the path to COM in reverse order, starting with COM and ending with the node itself.
fn path_to_com(orbits: &HashMap<String, String>, node: String) -> Vec<String> {
    if &node == "COM" {
        vec![]
    } else {
        let mut rest_path = path_to_com(&orbits, orbits.get(&node).unwrap().to_string());
        rest_path.push(node.clone());
        rest_path
    }
}

// Gets the length of the path through the lowest common ancestor of each Vec. Returns 0 otherwise.
fn lca(vec1: Vec<String>, vec2: Vec<String>) -> u64 {
    let mut ans = 0_u64;
    for (i, node) in vec1.iter().enumerate().rev() {
        if vec2.contains(&node) {
            ans = (i + vec2.iter().position(|x| x == node).unwrap() + 1).try_into().unwrap();
        }
    }
    ans
}

// Gets the total number of nodes in the path between the nodes that both the source and destination
// strings orbit, assuming the paths start at the left.
fn transfers_required(orbits: HashMap<String, String>, src: String, dest: String) -> u64 {
    let mut src_path = path_to_com(&orbits, src);
    // this removes the node itself
    src_path.pop();
    src_path.reverse();
    let mut dest_path = path_to_com(&orbits, dest);
    dest_path.pop();
    dest_path.reverse();
    lca(src_path, dest_path) - 1
}

fn main() {
    let path = Path::new("input");

    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut orbits = HashMap::new();
    
    for line in lines {
        let l = line.unwrap();
        let mut objs = l.split(')');
        let obj1 = objs.next().unwrap();
        let obj2 = objs.next().unwrap();
        orbits.insert(obj2.to_owned(), obj1.to_owned());
    }
    println!("The answer to part 1 is {}", total_orbits(&orbits));
    println!("The answer to part 2 is {}", transfers_required(orbits, "YOU".to_string(), "SAN".to_string()));
}

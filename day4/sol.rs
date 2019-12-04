use std::convert::TryInto;

fn get_digits(n: u64) -> Vec<u64> {
    n.to_string()
        .chars()
        .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
        .collect()
}


// assumes password in correct range
// for part 1
fn is_valid(password: u64) -> bool {
    let digits = get_digits(password);
    let mut is_same = false;
    for i in 0..digits.len() - 1 {
        is_same = is_same || (digits[i] == digits[i+1]);
        if digits[i] > digits[i+1] {
            return false;
        }
    }
    return is_same;    
}

fn single_pair(password: u64) -> bool {
    let digits = get_digits(password);
    let mut groups = vec![];
    let mut curr_group = 1;
    for i in 1..digits.len() {
        if digits[i-1] == digits[i] {
            curr_group += 1;
        } else {
            groups.push(curr_group);
            curr_group = 1;
        }
    }
    groups.push(curr_group);
    return groups.contains(&2);
}

fn main() {
    let low: u64 = 152085;
    let high: u64 = 670283;

    let mut part1_ans = 0;
    for num in low..high {
        if is_valid(num) {
            part1_ans += 1;
        }
    }
    println!("The answer to part 1 is {}", part1_ans);

    let mut part2_ans = 0;
    for num in low..high {
        if is_valid(num) && single_pair(num) {
            part2_ans += 1;
        }
    }
    println!("The answer to part 2 is {}", part2_ans);
}

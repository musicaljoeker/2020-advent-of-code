use std::fs;
use std::collections::HashSet;
use std::process;

// solved in O(n)
fn problem_one() {
    // reading in the file -- O(n)
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the input file");

    // converting file to an array of i32s -- O(n)
    let entries_int: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    // converting i32 array into a hashset for quick lookup -- O(n)
    let entries_hs: HashSet<i32> = entries_int
        .clone()
        .into_iter()
        .collect();

    // find the complement and compute the answer -- O(n)
    for e in entries_int {
        let complement = 2020 - e;
        if entries_hs.contains(&complement) {
            println!("Answer: {}", e * complement);
            process::exit(0); // done!
        }
    }
}

fn main() {
    problem_one()
}

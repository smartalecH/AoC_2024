use std::fs::File;
use std::io::{BufRead, BufReader};
pub fn run() {
    let file = File::open("input/day1.txt").expect("Failed to open file");
    let reader = BufReader::new(file);
    // Read input and solve puzzle here
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        // Process line
    }
    // Print solution
    println!("Day 1 solution: ");
}
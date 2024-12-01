use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run(input: String) {
    part1(input.clone());
    part2(input.clone());
}

fn part1(input: String) {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();

    let mut sum_of_differences = 0;

    for (a, b) in left.iter().zip(right.iter()) {
        let difference = *a - *b;
        sum_of_differences += difference.abs();
    }
    println!("Day 1, part 1 solution: {}", sum_of_differences);
}

fn part2(input:String) {
    let (mut left, mut right) = parse_input(input);
    left.sort();
    right.sort();

    let mut similarity_score = 0;
    for &num in &left {
        let count = right.iter().filter(|&&x| x == num).count();
        similarity_score += count * (num as usize);
    }
    println!("Day 1, part 2 solution: {}", similarity_score);
}

fn parse_input(input: String) -> (Vec<i32>, Vec<i32>) {
    let filename = format!("input/day_01_{}.txt", input);

    match input.as_str() {
        "demo" | "test" => (), // Do nothing on success
        _ => panic!("Invalid input state"),
    }

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    // Read input and solve puzzle here
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() != 2 {
            panic!("Invalid input format");
        }
        left.push(parts[0].parse::<i32>().expect("Failed to parse integer"));
        right.push(parts[1].parse::<i32>().expect("Failed to parse integer"));
    }

    (left, right)
}
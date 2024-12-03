use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;

pub fn run(input: String) {
    part1(input.clone());
    part2(input.clone());
}

fn part1(input: String) {
    let filename = format!("input/day_03_{}.txt", input);

    match input.as_str() {
        "demo" | "test" => (), // Do nothing on success
        _ => panic!("Invalid input state"),
    }

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut running_total = 0;

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        
        // Iterate over all matches in the text
        for cap in re.captures_iter(line.as_str()) {
            // Parse the captured groups as integers
            if let (Ok(first), Ok(second)) = (cap[1].parse::<i32>(), cap[2].parse::<i32>()) {
                running_total += first * second;
            }
        }
    }

    println!("Day 1, part 1 solution: {}", running_total);
}

fn part2(input:String) {
    let filename = format!("input/day_03_{}.txt", input);

    match input.as_str() {
        "demo" | "test" => (), // Do nothing on success
        _ => panic!("Invalid input state"),
    }

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mul_re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    let mut running_total = 0;
    let mut flag = true;
    for line in reader.lines() {
        let line = line.expect("Failed to read line");

        let mut left = 0;
        let mut right = 1;
        while right <= line.len() {
            let current_string = &line[left..right];
            if let Some(mul_cap) = mul_re.captures(current_string) {
                // If a mul pattern is found
                if flag {
                    if let (Ok(first), Ok(second)) = (mul_cap[1].parse::<i32>(), mul_cap[2].parse::<i32>()) {
                        running_total += first * second;
                    }
                }
                left = right;
                right = left + 1;
            } else if let Some(_do_match) = do_re.find(current_string) {
                // If a do pattern is found, set the flag to true
                flag = true;
                left = right;
                right = left + 1;
            } else if let Some(_dont_match) = dont_re.find(current_string) {
                // If a don't pattern is found, set the flag to false
                flag = false;
                left = right;
                right = left + 1;
            } else {
                // If no pattern is found, move to the next character
                right += 1;
            }
        }
    }

    println!("Day 1, part 2 solution: {}", running_total);
}
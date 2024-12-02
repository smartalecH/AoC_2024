use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run(input: String) {
    part1(input.clone());
    part2(input.clone());
}

/// Enum representing the state of a value.
#[derive(Debug, PartialEq)]
pub enum ReportState {
    Increasing,
    Decreasing,
}

fn part1(input: String) {
    let filename = format!("input/day_02_{}.txt", input);

    match input.as_str() {
        "demo" | "test" => (), // Do nothing on success
        _ => panic!("Invalid input state"),
    }

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut valid_report = 0;
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split_whitespace().collect();

        let mut prev_num = parts[0].parse::<i32>().unwrap();
        let mut current_num = parts[1].parse::<i32>().unwrap();
        let mut difference = current_num - prev_num;      

        // Initalize the state
        let state = match difference {
            x if x > 0 => ReportState::Increasing,
            _ => ReportState::Decreasing,
        };

        let mut is_valid = true;  

        for i in 1..parts.len() {
            current_num = parts[i].parse::<i32>().unwrap();
            difference = current_num - prev_num;
            // Check if we are increasing, and meant to be increasing
            if (state == ReportState::Increasing) && (difference < 0) {
                is_valid = false;
            }
            // Check if we are decreasing, and meant to be decreasing
            if (state == ReportState::Decreasing) && (difference > 0) {
                is_valid = false;
            }
            // Check if we are minimum 1 apart
            if difference.abs() < 1 {
                is_valid = false;
            }
            // Check if we are maximum 3 apart
            if difference.abs() > 3 {
                is_valid = false;
            }

            prev_num = current_num;
        }
        if is_valid {
            valid_report += 1;
        }
    }

    println!("Day 1, part 1 solution: {}", valid_report);
}

fn part2(input: String) {
    let filename = format!("input/day_02_{}.txt", input);

    match input.as_str() {
        "demo" | "test" => (), // Do nothing on success
        _ => panic!("Invalid input state"),
    }

    let file = File::open(filename).expect("Failed to open file");
    let reader = BufReader::new(file);

    let mut valid_report = 0;
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split_whitespace().collect();

        let mut is_valid = false;

        for dampener in 0..parts.len(){
            let mut numbers: Vec<i32> = parts.iter()
                .flat_map(|x| x.parse::<i32>().ok())
                .collect();
            
            // remove the current dampener
            numbers.remove(dampener);

            // Initalize the state
            let mut prev_num = numbers[0];
            let mut current_num = numbers[1];
            let mut difference = current_num - prev_num;
            let state = match difference {
                x if x > 0 => ReportState::Increasing,
                _ => ReportState::Decreasing,
            };

            // Check the current iteration omitting the dampener
            let mut current_attempt_is_valid = true;
            for i in 1..numbers.len() {
                current_num = numbers[i];
                difference = current_num - prev_num;
                // Check if we are increasing, and meant to be increasing
                if (state == ReportState::Increasing) && (difference < 0) {
                    current_attempt_is_valid = false;
                    continue;
                }
                // Check if we are decreasing, and meant to be decreasing
                if (state == ReportState::Decreasing) && (difference > 0) {
                    current_attempt_is_valid = false;
                    continue;
                }
                // Check if we are minimum 1 apart
                if difference.abs() < 1 {
                    current_attempt_is_valid = false;
                    continue;
                }
                // Check if we are maximum 3 apart
                if difference.abs() > 3 {
                    current_attempt_is_valid = false;
                    continue;
                }
                prev_num = current_num;
            }
            if current_attempt_is_valid {
                is_valid = true;
                continue;
            }

        }
        if is_valid {
            valid_report += 1;
        }
    }

    println!("Day 1, part 2 solution: {}", valid_report);
}
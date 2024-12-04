use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn run(input: String) {
    part1(input.clone());
    part2(input.clone());
}

fn part1(input: String) {
    let char_2d_array = get_array_from_text(input);
    let mut count = 0;
    for row in 0..char_2d_array.len() {
        // Loop over each column index
        for col in 0..char_2d_array[row].len() {
            // Pass the array, row, column index, and count to the function
            process_element(&mut count, &char_2d_array, row, col,);
        }
    }
    println!("Day 4, part 1 solution: {}", count);

}

fn part2(input: String) {
    let char_2d_array = get_array_from_text(input);
    let mut count = 0;
    // Loop over each row index, avoiding the first and last row
    for row in 1..char_2d_array.len() - 1 {
        // Loop over each column index, avoiding the first and last column
        for col in 1..char_2d_array[row].len() - 1 {
            // Pass the array, row, column index, and count to the function
            process_cross(&mut count, &char_2d_array, row, col,);
        }
    }
    println!("Day 4, part 2 solution: {}", count);

}

fn process_element(count: &mut i32, array: &Vec<Vec<char>>, row: usize, col: usize) {
    let key_vec: Vec<char> = "XMAS".chars().collect();
    // North
    if row >= 3 {
        let slice: Vec<char> = (0..=3).map(|i| array[row - i][col]).collect();
        if slice == key_vec {
            //println!("North slice at column {} starting from row {}: {:?}", col, row, slice);
            *count += 1;
        }
    }
    // South
    if row <= array.len() - 4 {
        let slice: Vec<char> = (0..=3).map(|i| array[row + i][col]).collect();
        if slice == key_vec {
            //println!("South slice at column {} starting from row {}: {:?}", col, row, slice);
            *count += 1;
        }
    }
    // East
    if col <= array[row].len() - 4 {
        let slice: Vec<char> = (0..=3).map(|i| array[row][col + i]).collect();
        if slice == key_vec {
            //println!("East slice at row {} starting from column {}: {:?}", row, col, slice);
            *count += 1;
        }
    }
    // West
    if col >= 3 {
        let slice: Vec<char> = (0..=3).map(|i| array[row][col - i]).collect();
        if slice == key_vec {
            //println!("West slice at row {} starting from column {}: {:?}", row, col, slice);
            *count += 1;
        }
    }

    // Northeast
    if row >= 3 && col <= array[row].len() - 4 {
        let slice: Vec<char> = (0..=3).map(|i| array[row - i][col + i]).collect();
        if slice == key_vec {
            //println!("Northeast slice starting at ({}, {}): {:?}", row, col, slice);
            *count += 1;
        }
    }
    // Northwest
    if row >= 3 && col >= 3 {
        let slice: Vec<char> = (0..=3).map(|i| array[row - i][col - i]).collect();
        if slice == key_vec {
            //println!("Northwest slice starting at ({}, {}): {:?}", row, col, slice);
            *count += 1;
        }
    }
    // Southeast
    if row <= array.len() - 4 && col <= array[row].len() - 4 {
        let slice: Vec<char> = (0..=3).map(|i| array[row + i][col + i]).collect();
        if slice == key_vec {
            //println!("Southeast slice starting at ({}, {}): {:?}", row, col, slice);
            *count += 1;
        }
    }
    // Southwest
    if row <= array.len() - 4 && col >= 3 {
        let slice: Vec<char> = (0..=3).map(|i| array[row + i][col - i]).collect();
        if slice == key_vec {
            //println!("Southwest slice starting at ({}, {}): {:?}", row, col, slice);
            *count += 1;
        }
    }

    return;
}

fn process_cross(count: &mut i32, array: &Vec<Vec<char>>, row: usize, col: usize) {
    if array[row][col] == 'A' {
        if (array[row - 1][col - 1] == 'M' && array[row + 1][col + 1] == 'S' && array[row - 1][col + 1] == 'M' && array[row + 1][col - 1] == 'S') ||
            (array[row - 1][col - 1] == 'S' && array[row + 1][col + 1] == 'M' && array[row - 1][col + 1] == 'M' && array[row + 1][col - 1] == 'S') ||
            (array[row - 1][col - 1] == 'S' && array[row + 1][col + 1] == 'M' && array[row - 1][col + 1] == 'S' && array[row + 1][col - 1] == 'M') ||
            (array[row - 1][col - 1] == 'M' && array[row + 1][col + 1] == 'S' && array[row - 1][col + 1] == 'S' && array[row + 1][col - 1] == 'M') {
            *count += 1;
        }
    }
}

fn get_array_from_text(input: String) -> Vec<Vec<char>> {

    match input.as_str() {
        "demo" | "test" => (), // Do nothing on success
        _ => panic!("Invalid input state"),
    }

    let filename = format!("input/day_04_{}.txt", input);
    let path = Path::new(&filename);
    let file = File::open(&path).expect("Failed to open file");
    let reader = io::BufReader::new(file);
    let mut char_2d_array = Vec::new();
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let char_vec: Vec<char> = line.chars().collect();
        char_2d_array.push(char_vec);
    }
    return char_2d_array;
}
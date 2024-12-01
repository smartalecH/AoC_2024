mod days;

fn main() {
    let day = std::env::args().nth(1).unwrap_or("1".to_string());
    let input = std::env::args().nth(2).unwrap_or("demo".to_string());

    match day.as_str() {
        "1" => days::day01::run(input),
        // Add more days as needed
        _ => println!("Invalid day"),
    }
}
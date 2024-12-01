mod days;
fn main() {
    let day = std::env::args().nth(1).unwrap_or("1".to_string());
    match day.as_str() {
        "1" => days::day01::run(),
        // Add more days as needed
        _ => println!("Invalid day"),
    }
}
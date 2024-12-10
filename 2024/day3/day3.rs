use std::{
    fs::File
};
use std::io::{self, BufRead, BufReader};
use std::time::Instant;
use regex::Regex;

fn extract_mul_sum(input: &str) -> i32 {
    // Define the regex pattern to match `mul(x,y)` where x and y are integers
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    // Initialize total as 0
    let mut total: i32 = 0;

    // Find all matches in the input string
    for cap in re.captures_iter(input) {
        if let (Ok(x), Ok(y)) = (cap[1].parse::<i32>(), cap[2].parse::<i32>()) {
            total += x * y;
        }
    }

    total
}

fn main() -> io::Result<()> {
    let start = Instant::now(); // Start timer

    // Open the input file
    let input_file = File::open("input.txt")?;
    let reader = BufReader::new(input_file);

    // Create count for multiplication values
    let mut multcount: i32 = 0;

    for line in reader.lines() {
        let line = line?;  // Handle each line safely
        let total = extract_mul_sum(&line);
        multcount += total;
    }

    let total_duration = start.elapsed(); // Total time
    println!("Total: {:?}", multcount);

    // Print benchmark times
    println!("Benchmark Results: {:?}", total_duration);

    Ok(())
}
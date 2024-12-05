use std::{
    fs::File
};
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
use std::time::Instant;

fn main() -> io::Result<()> {
    let start = Instant::now(); // Start timer

    // Open the input file
    let input_file = File::open("input.txt")?;
    let reader = BufReader::new(input_file);

    // Create vectors to store integer data
    let mut column1: Vec<i32> = Vec::new();
    let mut column2: Vec<i32> = Vec::new();

    for line in reader.lines() {
        let line = line?;  // Handle each line safely
        let columns: Vec<&str> = line.split_whitespace().collect(); // Split by whitespace
        
        // Parse the columns as integers and handle potential errors
        if let (Ok(num1), Ok(num2)) = (columns[0].parse::<i32>(), columns[1].parse::<i32>()) {
            column1.push(num1);
            column2.push(num2);
        } else {
            eprintln!("Skipping line due to insufficient columns: {}", line);
        }
    }

    // Sort the vectors in ascending order
    column1.sort();
    column2.sort();

    // part1

    let differences: Vec<i32> = column1.iter()
    .zip(column2.iter())
    .map(|(a, b)| (b - a).abs())
    .collect();

    let sum: i32 = differences.iter().sum();

    // Print the sorted results
    println!("sum: {:?}", sum);

    // part2

    // Count the occurrences of each value in column2 using a HashMap
    let mut column2_counts: HashMap<i32, usize> = HashMap::new();
    for &num in &column2 {
        *column2_counts.entry(num).or_insert(0) += 1;
    }

// Compute the product of each value in column1 and its count in column2
let results: Vec<i32> = column1.iter()
    .map(|&num| {
        let count = column2_counts.get(&num).unwrap_or(&0);
        num * (*count as i32)
    })
    .collect();

    let sum2: i32 = results.iter().sum();

    println!("sum part 2: {:?}", sum2);

    let total_duration = start.elapsed(); // Total time

        // Print benchmark times
        println!("Benchmark Results: {:?}", total_duration);
    

    Ok(())
}
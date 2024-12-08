use std::{
    fs::File
};
use std::io::{self, BufRead, BufReader};
use std::time::Instant;

// Function to check if differences meet the criteria
fn check_differences(numbers: &[i32]) -> bool {
    // Check if all differences are positive or all are negative
    let differences: Vec<i32> = numbers
        .windows(2) // Create overlapping windows of size 2
        .map(|pair| pair[1] - pair[0])
        .collect();
    let all_positive = differences.iter().all(|&d| d > 0);
    let all_negative = differences.iter().all(|&d| d < 0);

    // Check if all differences are either 1 or 2
    let one_to_three = differences.iter().all(|&d| d.abs() <= 3);
    (all_positive || all_negative) && one_to_three
}

fn main() -> io::Result<()> {
    let start = Instant::now(); // Start timer

    // Open the input file
    let input_file = File::open("input.txt")?;
    let reader = BufReader::new(input_file);

    // Create vectors to store integer data
    let mut safecount: i32 = 0;

    for line in reader.lines() {
        let line = line?;  // Handle each line safely
        let values: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok())
            //.parse::<i32>().ok()
            .collect(); // Split by whitespace
      
        // part1
        if check_differences(&values) {
            safecount = safecount+1;
        } else { //part2
            // Try removing one value at a time and re-checking
            for i in 0..values.len() {
                let mut modified_list = values.clone();
                modified_list.remove(i);

                if check_differences(&modified_list) {
                    //println!(
                    //"The sequence passes after removing the value at index {}: {:?}",
                    //i, values[i]);
                    safecount=safecount+1;
                    break;
            }
        }

        }

        //println!("differences: {:?}", differences);
        //println!("final conclusion: {:?}", (all_positive || all_negative) && one_to_three)
    }


    println!("safecount: {:?}", safecount);

    let total_duration = start.elapsed(); // Total time

    // Print benchmark times
    println!("Benchmark Results: {:?}", total_duration);

    Ok(())
}
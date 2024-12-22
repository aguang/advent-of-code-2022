use std::{
    fs::File
};
use std::io::{self, BufRead, BufReader};
use std::time::Instant;
use regex::Regex;

fn extract_mul_sum(input: &str, enabled: &mut bool) -> (i32, bool) {
    // Define the regex pattern to match `mul(x,y)` where x and y are integers
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let do_re = Regex::new(r"do\(\)").unwrap();
    let dont_re = Regex::new(r"don't\(\)").unwrap();

    let mut total: i32 = 0;

    // Split input string into token
    let mut pos = 0;
    //println!("start of new extract_mul_sum, enabled is: {:?}", enabled);
    while pos < input.len() {
        //println!("pos: {:?}", pos);
        if *enabled {
            // get re, don't positions
            if let Some(cap) = re.captures(&input[pos..]) {
                if let Some(dont) = dont_re.find(&input[pos..]) {
                    // if re is before don't, add it and move position up to re.end()
                    if cap.get(0).unwrap().start() < dont.start() {
                        //println!("enabled is true & re < dont_re. cap: {:?}", cap);
                        //println!("dont_re: {:?}", dont);
                        if let (Ok(x), Ok(y)) = (cap[1].parse::<i32>(), cap[2].parse::<i32>()) {
                            total += x*y;
                        }
                        pos += cap.get(0).unwrap().end(); // Advance the position
                    }
                    // if re is after don't, don't add it and set enabled to be false,
                    // as well as position to be don't.end()
                    else if cap.get(0).unwrap().start() > dont.start() {
                        //println!("re > dont_re, setting enabled to be false. cap: {:?}", cap);
                        //println!("dont_re: {:?}", dont);
                        *enabled = false;
                        pos += dont.end();
                    }
                } else { // also a possibility there is no more dont token in which case just add
                    if let (Ok(x), Ok(y)) = (cap[1].parse::<i32>(), cap[2].parse::<i32>()) {
                        total += x*y;
                    }
                    pos += cap.get(0).unwrap().end(); // Advance the position
                }
            } else { break; } // means no more re tokens anyway, so just end
                
        } else {  // while enabled is false
            if let Some(do_token) = do_re.find(&input[pos..]) { // get do position
                //println!("found next do token, setting enabled to be true. do: {:?}", do_token);
                *enabled = true;
                pos += do_token.end(); // move position to next do
            } else { break; } // if no more dos then no more adding
        }
    }

    (total, *enabled)
}

fn main() -> io::Result<()> {
    let start = Instant::now(); // Start timer

    // Open the input file
    let input_file = File::open("input.txt")?;
    let reader = BufReader::new(input_file);

    // Create count for multiplication values
    let mut multcount: i32 = 0;
    let mut enabled: bool = true; // Tracks whether mul instructions are enabled

    for line in reader.lines() {
        let line = line?;  // Handle each line safely
        let (total, _enabled) = extract_mul_sum(&line, &mut enabled);
        //println!("enabled is: {:?}", enabled);
        multcount += total;
    }

    let total_duration = start.elapsed(); // Total time
    println!("Total: {:?}", multcount);

    // Print benchmark times
    println!("Benchmark Results: {:?}", total_duration);

    Ok(())
}
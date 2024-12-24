use std::time::Instant;
use std::fs::File;
use std::io::{self, BufRead};

fn count_word(grid: &[Vec<char>], word: &str) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let word_chars: Vec<char> = word.chars().collect();
    let mut count = 0;

    // Helper function to check if the word exists in a specific direction
    fn matches(grid: &[Vec<char>], word: &[char], start_row: usize, start_col: usize, delta_row: isize, delta_col: isize) -> bool {
        //println!("delta_row and delta_col is: {:?} {:?}", delta_row, delta_col);
        let mut row = start_row as isize;
        let mut col = start_col as isize;

        for &ch in word {
            if row < 0 || row >= grid.len() as isize || col < 0 || col >= grid[0].len() as isize {
                return false;
            }
            if grid[row as usize][col as usize] != ch {
                return false;
            }
        //    println!("matched");
        //    println!("current grid char is: {:?}", grid[row as usize][col as usize]);
            row += delta_row;
            col += delta_col;
        }
        row -= delta_row; // reverse positions by 1 to undo last command of for loop
        col -= delta_col;
        // if found match check to see if X has
        //println!("word MAS found");
        //println!("position row: {:?}", row);
        //println!("position col: {:?}", col);
        //println!("direction row: {:?}", delta_row);
        //println!("direction col: {:?}", delta_col);
        let res = xmatch(grid, row, col, delta_row, delta_col);
        if let Ok(_value) = res {
            //println!("MAS match found: {:?}", value);
            return true;
        } else {
            return false;
        }
    }

    // Another helper function for: if match is found in one direction, then check the other spots
    // in grid for also an MAS
    fn xmatch(grid: &[Vec<char>],
        row: isize, col: isize,
        delta_row: isize,
        delta_col: isize) -> Result<((isize, isize), (isize, isize)), &'static str> {
        // row, col is last row, col MAS was found on
        // delta_row, delta_col indicates diagonality, we need to get the proper other X
        // X = (row, col-2*delta_col), (row-2*delta_row, col)
        let x_pos_1 = (row, col - 2 * delta_col);
        let x_pos_2 = (row - 2 * delta_row, col);
        //println!("x_pos_1: {:?}", x_pos_1);
        //println!("x_pos_2: {:?}", x_pos_2);

        // Check if the positions are within bounds
        if x_pos_1.0 < 0
            || x_pos_1.0 >= grid.len() as isize
            || x_pos_1.1 < 0
            || x_pos_1.1 >= grid[0].len() as isize
            || x_pos_2.0 < 0
            || x_pos_2.0 >= grid.len() as isize
            || x_pos_2.1 < 0
            || x_pos_2.1 >= grid[0].len() as isize
        {
         return Err("X is out of grid");
        } else {
            let char1 = grid[x_pos_1.0 as usize][x_pos_1.1 as usize];
            let char2 = grid[x_pos_2.0 as usize][x_pos_2.1 as usize];
            //println!("char1: {:?}", char1);
            //println!("char2: {:?}", char2);
            if (char1 == 'M' && char2 == 'S') || (char1 == 'S' && char2 == 'M') {
                Ok((x_pos_1, x_pos_2))
            } else {
                Err("Characters do not form a valid XMAS match")
            }
        }

    }

    // Directions to check: right, down, down-right, down-left
    let directions = [
        //(0, 1),   // Horizontal (right)
        //(0, -1),  // Horizontal (left)
        //(1, 0),   // Vertical (down)
        //(-1, 0),  // Vertical (up)
        (-1, 1), // Diagonal up-right 
        (1, 1),   // Diagonal down-right
        (-1, -1), // Diagonal up-left
        (1, -1),  // Diagonal down-left
    ];

    for row in 0..rows {
        for col in 0..cols {
            for &(delta_row, delta_col) in &directions {
                if matches(grid, &word_chars, row, col, delta_row, delta_col) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn read_grid_from_file(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let mut grid = Vec::new();
    let file = File::open(filename)?;
    for line in io::BufReader::new(file).lines() {
        let line = line?;
        grid.push(line.chars().collect());
    }
    Ok(grid)
}

fn main() {
    let start = Instant::now(); // Start timer
    let filename = "input.txt";
    let word = "MAS";

    match read_grid_from_file(filename) {
        Ok(grid) => {
            // our method of looking for corresponding X will double-count, so divide by 2
            let total_count = count_word(&grid, word) / 2;
            println!("The total number of '{}' occurrences is: {}", word, total_count);
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
        }
    }

    let total_duration = start.elapsed(); // Total time
    // Print benchmark times
    println!("Benchmark Results: {:?}", total_duration);

}
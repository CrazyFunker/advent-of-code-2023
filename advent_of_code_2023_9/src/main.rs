use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Not;
use std::path::Path;

fn main() {
    let mut result: i32 = 0;

    // read the input file
    let path: &Path = Path::new("input.txt");
    let file: File = File::open(&path).unwrap();
    let reader: io::BufReader<File> = io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let line_numbers: Vec<i32> = line
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();

                let sum_of_extrapolated_numbers = extrapolate_row(&line_numbers);
                println!("Sum of extrapolated numbers: {}", sum_of_extrapolated_numbers);
                result += sum_of_extrapolated_numbers;
            }
            Err(error) => {
                println!("Error reading line: {}", error);
            }
        }
    }

    println!("Result: {}", result);
}

// returns the sum of the extrapolated numbers
fn extrapolate_row(row: &Vec<i32>) -> i32 {
    let mut result: i32 = 0;
    let mut last_numbers: Vec<i32> = Vec::new();
    let mut current_row: Vec<i32> = row.clone();
    last_numbers.push(current_row.last().unwrap().clone());

    while !current_row.iter().all(|n| n == &0) {
        let mut new_row: Vec<i32> = Vec::new();

        for i in 0..current_row.len() - 1 {
            let a = current_row[i];
            let b = current_row[i + 1];
            new_row.push(b - a);
        }

        last_numbers.push(new_row.last().unwrap().clone());
        current_row = new_row;
    }

    println!("Last numbers: {:?}", last_numbers);

    let mut extrapolated_number: i32 = 0;
    while last_numbers.len() > 0 {
        let next_extrapolated_number = last_numbers.pop().unwrap() + extrapolated_number;
        result += next_extrapolated_number;
    }

    result
}

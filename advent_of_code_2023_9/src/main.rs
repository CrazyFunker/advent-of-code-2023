use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut result: i32 = 0;
    let mut result_v2: i32 = 0;

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

                let sum_of_extrapolated_numbers_right = extrapolate_row_right(&line_numbers);
                result += sum_of_extrapolated_numbers_right;

                let sum_of_extrapolated_numbers_left = extrapolate_row_left(&line_numbers);
                result_v2 += sum_of_extrapolated_numbers_left;
            }
            Err(error) => {
                println!("Error reading line: {}", error);
            }
        }
    }

    println!("Result: {}", result);
    println!("Result_v2: {}", result_v2);
}

// returns the sum of the extrapolated numbers
fn extrapolate_row_right(row: &Vec<i32>) -> i32 {
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

    let mut extrapolated_number: i32 = 0;
    while last_numbers.len() > 0 {
        let next_extrapolated_number = last_numbers.pop().unwrap() + extrapolated_number;
        extrapolated_number = next_extrapolated_number;
    }

    extrapolated_number
}

// returns the sum of the extrapolated numbers
fn extrapolate_row_left(row: &Vec<i32>) -> i32 {
    let mut first_numbers: Vec<i32> = Vec::new();
    let mut current_row: Vec<i32> = row.clone();

    while !current_row.iter().all(|n| n == &0) {
        first_numbers.push(current_row.first().unwrap().clone());
        let mut new_row: Vec<i32> = Vec::new();

        for i in 0..current_row.len() - 1 {
            let a = current_row[i];
            let b = current_row[i + 1];
            new_row.push(b - a);
        }

        current_row = new_row;
    }

    let mut extrapolated_number: i32 = 0;
    while first_numbers.len() > 0 {
        let next_extrapolated_number = first_numbers.pop().unwrap() - extrapolated_number;
        extrapolated_number = next_extrapolated_number;
    }

    extrapolated_number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extrapolate_row_left() {
        let row = vec![10, 13, 16, 21, 30, 45];
        assert_eq!(extrapolate_row_left(&row), 5);
    }
}

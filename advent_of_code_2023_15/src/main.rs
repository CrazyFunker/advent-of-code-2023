use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

enum Operation {
    Add,
    Remove,
}

struct LensOperation {
    operation: String,
    value: u64,
}

fn main() {
    let mut all_steps: String = String::new();

    // read the input file
    let path: &Path = Path::new("input.txt");
    let file: File = File::open(&path).unwrap();
    let reader: io::BufReader<File> = io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                all_steps.push_str(&line);
                break;
            }
            Err(error) => {
                println!("Error reading line: {}", error);
            }
        }
    }

    let steps: Vec<&str> = all_steps.split(",").collect();

    let start = Instant::now();
    let result = get_part_1(&steps);
    println!("Time: {:?}", start.elapsed());
    println!("Part 1: {}", result);
}

fn get_part_1(steps: &Vec<&str>) -> u64 {
    let mut result: u64 = 0;

    for step in steps {
        result += hash(step) as u64;
    }

    result
}

// The HASH algorithm is a way to turn any string of characters into a single number in the range 0 to 255. To run the HASH algorithm on a string, start with a current value of 0. Then, for each character in the string starting from the beginning:
//
// Determine the ASCII code for the current character of the string.
// Increase the current value by the ASCII code you just determined.
// Set the current value to itself multiplied by 17.
// Set the current value to the remainder of dividing itself by 256.
// After following these steps for each character in the string in order, the current value is the output of the HASH algorithm.

fn hash(s: &str) -> u8 {
    let mut current_value: u16 = 0;
    for c in s.chars() {
        let ascii_code = c as u16;
        current_value += ascii_code;
        current_value *= 17;
        current_value %= 256;
    }
    current_value as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_test() {
        let s = "HASH";
        let expected: u8 = 52;

        assert_eq!(hash(s), expected);
    }
}

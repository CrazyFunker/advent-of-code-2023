use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::VecDeque;

fn main() {
    let path: &Path = Path::new("input.txt");

    let file: File = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    let mut result: i32 = 0;
    let digits: Vec<&str> = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut queue: VecDeque<char> = VecDeque::with_capacity(5);

    // iterate line by line
    for line in reader.lines() {
        match line {
            Ok(line) => {
                let mut line_result: i32 = 0;

                for ch in line.chars() {
                    // first let's check if we found a digit
                    if ch.is_digit(10) {
                        line_result += ch.to_digit(10).unwrap() as i32 * 10;
                        break;
                    }

                    // if not, let's keep looking for spelt out digits
                    queue.push_back(ch);
                    // if the queue is too long, let's pop the first element
                    if queue.len() > 5 {
                        queue.pop_front();
                    }

                    // let's check if it's a number
                    let search_string: String = queue.iter().collect();
                    if let Some((index, _)) = digits.iter().enumerate().find(|&(_, &s)| search_string.to_lowercase().contains(s)) {
                        let found_digit: i32 = index as i32 + 1;
                        line_result += found_digit * 10;
                        break;
                    }
                }

                // clear the deque
                queue.clear();

                // iterate in reverse
                for ch in line.chars().rev() {
                    // first let's check if we found a digit
                    if ch.is_digit(10) {
                        let digit_found: u32 = ch.to_digit(10).unwrap();
                        line_result += digit_found as i32;
                        break;
                    }

                    // if not, let's keep looking for spelt out digits
                    queue.push_front(ch);
                    // if the queue is too long, let's pop the first element
                    if queue.len() > 5 {
                        queue.pop_back();
                    }

                    // let's check if it's a number
                    let search_string: String = queue.iter().collect();
                    if let Some((index, _)) = digits.iter().enumerate().find(|&(_, &s)| search_string.to_lowercase().contains(s)) {
                        let found_digit: i32 = index as i32 + 1;
                        line_result += found_digit;
                        break;
                    }
                }

                // clear the deque
                queue.clear();

                result += line_result;
            }
            Err(error) => {
                println!("Error reading line: {}", error);
            }
        }
    }

    println!("Result: {}", result);
}

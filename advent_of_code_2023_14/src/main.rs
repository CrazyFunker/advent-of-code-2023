use std::fs::File;
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut platform: Vec<Vec<char>> = Vec::new();
    let cycles: i32 = 1_000_000_000;

    // read the input file
    let path: &Path = Path::new("input.txt");
    let file: File = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                lines.push(line.clone());
                platform.push(line.chars().collect());
            }
            Err(error) => {
                println!("Error reading line: {}", error);
            }
        }
    }

    let start = std::time::Instant::now();
    roll_north(&mut platform);
    println!("{:?}", platform);
    let _ = write_to_file(&platform, "output.txt");
    let result = calculate_score(&mut platform);
    println!("Time: {:?}", start.elapsed());
    println!("Result: {}", result);
}

fn write_to_file(platform: &Vec<Vec<char>>, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;

    for row in platform {
        let s: String = row.into_iter().collect();
        writeln!(file, "{}", s)?;
    }

    Ok(())
}

fn cycle(platform: &mut Vec<Vec<char>>) {
    roll_north(platform);
    roll_west(platform);
    roll_south(platform);
    roll_east(platform);
}

fn roll_west(platform: &mut Vec<Vec<char>>) {
    let row_length = platform[0].len();
    let mut next_space: Vec<usize> = vec![0; row_length];
    for i in 0..platform.len() {
        let row = platform[i].clone();

        for (j, c) in row.iter().enumerate() {
            if i == 0 {
                if c != &'.' {
                    next_space[j] = 1;
                }
            } else {
                if c == &'O' {
                    // roll them north
                    if next_space[j] != i {
                        platform[next_space[j]][j] = 'O';
                        platform[i][j] = '.';
                    }

                    // next free space is +1 from last
                    next_space[j] += 1;
                } else if c == &'#' {
                    // reset next free space
                    next_space[j] = i + 1;
                }
            }
        }
    }
}

fn roll_south(platform: &mut Vec<Vec<char>>) {
    let row_length = platform[0].len();
    let mut next_space: Vec<usize> = vec![0; row_length];
    for i in (0..platform.len()).rev() {
        let row = platform[i].clone();

        for (j, c) in row.iter().enumerate() {
            if i == platform.len() - 1 {
                if c != &'.' {
                    next_space[j] = platform.len() - 2;
                }
            } else {
                if c == &'O' {
                    // roll them north
                    if next_space[j] != i {
                        platform[next_space[j]][j] = 'O';
                        platform[i][j] = '.';
                    }

                    // next free space is +1 from last
                    next_space[j] = next_space[j].saturating_sub(1);
                } else if c == &'#' {
                    // reset next free space
                    next_space[j] = i.saturating_sub(1);
                }
            }
        }
    }
}

fn roll_east(platform: &mut Vec<Vec<char>>) {
    let row_length = platform[0].len();
    let mut next_space: Vec<usize> = vec![0; row_length];
    for i in 0..platform.len(){
        let row = platform[i].clone();

        for (j, c) in row.iter().enumerate() {
            if i == 0 {
                if c != &'.' {
                    next_space[j] = 1;
                }
            } else {
                if c == &'O' {
                    // roll them north
                    if next_space[j] != i {
                        platform[next_space[j]][j] = 'O';
                        platform[i][j] = '.';
                    }

                    // next free space is +1 from last
                    next_space[j] += 1;
                } else if c == &'#' {
                    // reset next free space
                    next_space[j] = i + 1;
                }
            }
        }
    }
}


fn roll_north(platform: &mut Vec<Vec<char>>) {
    let row_length = platform[0].len();
    let mut next_space: Vec<usize> = vec![0; row_length];
    for i in 0..platform.len() {
        let row = platform[i].clone();

        for (j, c) in row.iter().enumerate() {
            if i == 0 {
                if c != &'.' {
                    next_space[j] = 1;
                }
            } else {
                if c == &'O' {
                    // roll them north
                    if next_space[j] != i {
                        platform[next_space[j]][j] = 'O';
                        platform[i][j] = '.';
                    }

                    // next free space is +1 from last
                    next_space[j] += 1;
                } else if c == &'#' {
                    // reset next free space
                    next_space[j] = i + 1;
                }
            }
        }
    }
}

fn calculate_score(platform: &mut Vec<Vec<char>>) -> i32 {
    let mut score: i32 = 0;
    for (i, row) in platform.iter().enumerate() {
        let score_multiplier = (platform.len() - i) as i32;
        for c in row.iter() {
            if c == &'O' {
                score += score_multiplier;
            }
        }
    }
    score
}

// tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roll_north() {
        let mut platform: Vec<Vec<char>> = Vec::new();
        let mut expected_platform: Vec<Vec<char>> = Vec::new();

        platform.push(".O.".chars().collect());
        platform.push("O#.".chars().collect());
        platform.push("O.O".chars().collect());
        
        expected_platform.push("OOO".chars().collect());
        expected_platform.push("O#.".chars().collect());
        expected_platform.push("...".chars().collect());

        roll_north(&mut platform);
        
        assert_eq!(platform, expected_platform);
    }

    #[test]
    fn test_roll_west() {
        
    }


    #[test]
    fn test_roll_south() {
        let mut platform: Vec<Vec<char>> = Vec::new();
        let mut expected_platform: Vec<Vec<char>> = Vec::new();

        platform.push("OOO".chars().collect());
        platform.push("O#.".chars().collect());
        platform.push("...".chars().collect());

        expected_platform.push(".O.".chars().collect());
        expected_platform.push("O#.".chars().collect());
        expected_platform.push("O.O".chars().collect());

        roll_south(&mut platform);
        
        assert_eq!(platform, expected_platform);
    }


    #[test]
    fn test_roll_east() {
        
    }
}
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Write;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut platform: Vec<Vec<char>> = Vec::new();
    let cycles: usize = 1_000_000_000;

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

    let mut platform_v2 = platform.clone();
    let mut platform_v2_hashes_map: HashMap<u64, Vec<Vec<char>>> = HashMap::new();
    let mut result_map: HashMap<i32, usize> = HashMap::new();

    let start = std::time::Instant::now();
    roll_north(&mut platform);
    let _ = write_to_file(&platform, "output.txt");
    let result = calculate_score(&mut platform);
    println!("Time: {:?}", start.elapsed());
    println!("Result: {}", result);

    let total_start = std::time::Instant::now();
    let mut start = std::time::Instant::now();
    for i in 0..cycles {
        if i % 1_000_000 == 0 {
            if i != 0 {
                println!("Time: {:?}", start.elapsed());
                start = std::time::Instant::now();
            }
            println!("Cycle: {}", i);
        }
        if cycle(&mut platform_v2, &mut platform_v2_hashes_map) {
            let result_v2 = calculate_score(&mut platform_v2);
            println!("Result: {}", result_v2);
            if result_map.contains_key(&result_v2) {
                break;
            } else {
                result_map.insert(result_v2, i);
            }
        }
    }

    println!("result_vec: {:#?}", result_map);

    let _ = write_to_file(&platform_v2, "output_v2.txt");
    let result_v2 = calculate_score(&mut platform_v2);
    println!("Result: {}", result_v2);
    println!("Time: {:?}", total_start.elapsed());
}

fn write_to_file(platform: &Vec<Vec<char>>, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;

    for row in platform {
        let s: String = row.into_iter().collect();
        writeln!(file, "{}", s)?;
    }

    Ok(())
}

// returns true if a cycle was found
fn cycle(platform: &mut Vec<Vec<char>>, map: &mut HashMap<u64, Vec<Vec<char>>>) -> bool {
    // hash platform and check if already in the map
    // it is, just assign that to platform
    // if not, calculate the next platform and add it to the map
    let hash = hash_vec(platform);
    if map.contains_key(&hash) {
        println!("Found a preexisting cycle!");
        *platform = map.get(&hash).unwrap().clone();
        return true;
    } else {
        println!("New cycle!");
        roll_north(platform);
        roll_west(platform);
        roll_south(platform);
        roll_east(platform);

        // key: hash -> platform pre cycle
        // value: platform post cycle
        map.insert(hash, platform.clone());
        false
    }
}

fn hash_vec(vec: &Vec<Vec<char>>) -> u64 {
    let mut hasher = DefaultHasher::new();
    vec.hash(&mut hasher);
    hasher.finish()
}

fn roll_west(platform: &mut Vec<Vec<char>>) {
    for i in 0..platform.len() {
        let mut next_space: usize = 0;
        let row = platform[i].clone();

        for (j, c) in row.iter().enumerate() {
            if j == 0 {
                if c != &'.' {
                    next_space = 1;
                }
            } else {
                if c == &'O' {
                    // roll them west
                    if next_space != j {
                        platform[i][next_space] = 'O';
                        platform[i][j] = '.';
                    }

                    // next free space is +1 from last
                    next_space += 1;
                } else if c == &'#' {
                    // reset next free space
                    next_space = j + 1;
                }
            }
        }
    }
}

fn roll_south(platform: &mut Vec<Vec<char>>) {
    let row_length = platform[0].len();
    let mut next_space: Vec<usize> = vec![platform.len() - 1; row_length];
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

    for i in 0..platform.len() {
        let mut next_space: usize = row_length - 1;
        let row = platform[i].clone();

        for (j, c) in row.iter().enumerate().rev() {
            if j == row_length - 1 {
                if c != &'.' {
                    next_space = row_length - 2;
                }
            } else {
                if c == &'O' {
                    // roll them west
                    if next_space != j {
                        platform[i][next_space] = 'O';
                        platform[i][j] = '.';
                    }

                    // next free space is +1 from last
                    next_space = next_space.saturating_sub(1);
                } else if c == &'#' {
                    // reset next free space
                    next_space = j.saturating_sub(1);
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

        platform.push("...O#O".chars().collect());
        platform.push("O#.O..".chars().collect());
        platform.push("OOO#OO".chars().collect());

        expected_platform.push("O.OO#O".chars().collect());
        expected_platform.push("O#.OOO".chars().collect());
        expected_platform.push(".O.#..".chars().collect());

        roll_north(&mut platform);

        assert_eq!(platform, expected_platform);
    }

    #[test]
    fn test_roll_west() {
        let mut platform: Vec<Vec<char>> = Vec::new();
        let mut expected_platform: Vec<Vec<char>> = Vec::new();

        platform.push(".OO".chars().collect());
        platform.push(".#O".chars().collect());
        platform.push("..O".chars().collect());
        platform.push("OO#".chars().collect());
        platform.push("#.O".chars().collect());
        platform.push("O.O".chars().collect());

        expected_platform.push("OO.".chars().collect());
        expected_platform.push(".#O".chars().collect());
        expected_platform.push("O..".chars().collect());
        expected_platform.push("OO#".chars().collect());
        expected_platform.push("#O.".chars().collect());
        expected_platform.push("OO.".chars().collect());

        roll_west(&mut platform);

        for row in &platform {
            println!("{:?}", row);
        }
        println!();
        for row in &expected_platform {
            println!("{:?}", row);
        }

        assert_eq!(platform, expected_platform);
    }

    #[test]
    fn test_roll_south() {
        let mut platform: Vec<Vec<char>> = Vec::new();
        let mut expected_platform: Vec<Vec<char>> = Vec::new();

        platform.push("OOO#OO".chars().collect());
        platform.push("O#.O..".chars().collect());
        platform.push("...O#O".chars().collect());

        expected_platform.push(".O.#..".chars().collect());
        expected_platform.push("O#.OOO".chars().collect());
        expected_platform.push("O.OO#O".chars().collect());

        roll_south(&mut platform);

        println!("{:?}", expected_platform[0]);
        println!("{:?}", expected_platform[1]);
        println!("{:?}", expected_platform[2]);
        println!();
        println!("{:?}", platform[0]);
        println!("{:?}", platform[1]);
        println!("{:?}", platform[2]);

        assert_eq!(platform, expected_platform);
    }

    #[test]
    fn test_roll_east() {
        let mut platform: Vec<Vec<char>> = Vec::new();
        let mut expected_platform: Vec<Vec<char>> = Vec::new();

        platform.push("OO.".chars().collect());
        platform.push("O#.".chars().collect());
        platform.push("O..".chars().collect());
        platform.push("#OO".chars().collect());
        platform.push("O.#".chars().collect());
        platform.push("O.O".chars().collect());

        expected_platform.push(".OO".chars().collect());
        expected_platform.push("O#.".chars().collect());
        expected_platform.push("..O".chars().collect());
        expected_platform.push("#OO".chars().collect());
        expected_platform.push(".O#".chars().collect());
        expected_platform.push(".OO".chars().collect());

        roll_east(&mut platform);

        for row in &platform {
            println!("{:?}", row);
        }
        println!();
        for row in &expected_platform {
            println!("{:?}", row);
        }

        assert_eq!(platform, expected_platform);
    }
}

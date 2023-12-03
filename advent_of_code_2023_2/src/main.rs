use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)] // Add this line
struct ColorCounts {
    red: i32,
    green: i32,
    blue: i32,
}

fn main() {
    let max_red: i32 = 12;
    let max_green: i32 = 13;
    let max_blue: i32 = 14;
    let mut sum: i32 = 0;
    let mut power: i32 = 0;

    // read the input file
    let path: &Path = Path::new("input.txt");
    let file: File = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    let mut games: HashMap<i32, ColorCounts> = HashMap::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                // Step 1 & 2: Split the line and extract game number
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                let game_number: i32 = parts[0].replace("Game ", "").trim().parse::<i32>().unwrap();

                // Step 3: Tokenize the data sets
                let mut color_counts: ColorCounts = ColorCounts {
                    red: 0,
                    green: 0,
                    blue: 0,
                };
                let data_sets: Vec<&str> = parts[1]
                    .split(';')
                    .map(|s: &str| s.trim())
                    .collect::<Vec<&str>>();

                // Output
                for data in data_sets {
                    for pair in data.split(',') {
                        let pair_parts: Vec<&str> = pair.trim().split_whitespace().collect();
                        let count: i32 = pair_parts[0].parse::<i32>().unwrap();
                        match pair_parts[1] {
                            "red" => {
                                if count > color_counts.red {
                                    color_counts.red = count;
                                }
                            },
                            "green" => {
                                if count > color_counts.green {
                                    color_counts.green = count;
                                }
                            },
                            "blue" => {
                                if count > color_counts.blue {
                                    color_counts.blue = count;
                                }
                            },
                            _ => println!("Unknown color: {}", pair_parts[1]),
                        }
                    }
                }

                // println!("Game {}: {:?}", game_number, color_counts);

                if color_counts.red <= max_red
                    && color_counts.green <= max_green
                    && color_counts.blue <= max_blue
                {
                    sum += game_number;
                }

                power += color_counts.red * color_counts.green * color_counts.blue;

                games.insert(game_number, color_counts);
            }
            Err(error) => println!("error: {}", error),
        }
    }

    // println!("Games: {:?}", games);
    println!("Sum of games: {}", sum);
    println!("Power of set: {}", power);

}

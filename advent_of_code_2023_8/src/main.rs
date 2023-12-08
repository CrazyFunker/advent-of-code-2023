use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct CamelMapEntry {
    left: &'static str,
    right: &'static str,
    name: &'static str,
}

fn main() {
    let mut camel_map: HashMap<String, CamelMapEntry> = HashMap::new();

    // read the input file
    let path: &Path = Path::new("input.txt");
    let file: File = File::open(&path).unwrap();
    let reader: io::BufReader<File> = io::BufReader::new(file);
    let mut current_positions_v2: Vec<&str> = Vec::new();

    let mut directions: String = String::new();

    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }

                if index == 0 {
                    directions = line.clone();
                    println!("First line: {}", directions);
                } else if index > 1 {
                    let parts: Vec<&str> = line.splitn(2, '=').collect();
                    let name: &str = parts[0].trim();
                    let parts2: Vec<&str> = parts[1]
                        .trim_matches(|c| c == '(' || c == ')')
                        .splitn(2, ',')
                        .collect();
                    let left: &str = parts2[0].trim();
                    let right: &str = parts2[1].trim();
                    let map_entry: CamelMapEntry = CamelMapEntry {
                        left,
                        right,
                        name,
                    };
                    camel_map.insert(name.to_owned(), map_entry);

                    if name.ends_with('A') {
                        current_positions_v2.push(name);
                    }
                }
            }
            Err(error) => {
                println!("Error reading line: {}", error);
            }
        }
    }
    // println!("CamelMapEntry: {:?}", camel_map);

    // while current position != ZZZ
    let mut current_position: String = "AAA".to_owned();
    let mut steps: i32 = 0;
    while current_position != "ZZZ" {
        for c in directions.chars() {
            if c == 'L' {
                current_position = camel_map.get(&current_position).unwrap().left.to_owned();
            } else if c == 'R' {
                current_position = camel_map.get(&current_position).unwrap().right.to_owned();
            }

            steps += 1;

            if current_position == "ZZZ" {
                break;
            }
        }
    }

    println!("Steps: {}", steps);

    let mut steps_v2: u64 = 0;
    let mut break_out_v2 = false;
    while !break_out_v2 {
        for c in directions.chars() {
            travel_v2(c, &camel_map, &mut current_positions_v2);

            steps_v2 += 1;

            if are_we_there_yet_v2(&current_positions_v2) {
                break_out_v2 = true;
                break;
            }
        }
    }

    println!("Steps v2: {}", steps_v2);
}

fn travel_v2(
    direction: char,
    camel_map: &HashMap<String, CamelMapEntry>,
    current_positions: &mut Vec<&str>,
) {
    if direction == 'L' {
        for position in current_positions {
            *position = camel_map.get(position).unwrap().left;
        }
    } else if direction == 'R' {
        for position in current_positions {
            *position = camel_map.get(position).unwrap().right;
        }
    }
}

fn are_we_there_yet_v2(current_positions: &[&str]) -> bool {
    current_positions.iter().all(|position| position.ends_with('Z'))
}

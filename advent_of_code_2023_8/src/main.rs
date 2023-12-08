use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct CamelMapEntry {
    left: String,
    right: String,
    name: String,
}

fn main() {
    let mut camel_map: HashMap<String, CamelMapEntry> = HashMap::new();

    // read the input file
    let path: &Path = Path::new("input.txt");
    let file: File = File::open(&path).unwrap();
    let reader: io::BufReader<File> = io::BufReader::new(file);
    let mut current_positions_v2: Vec<String> = Vec::new();

    let mut directions: String = String::new();

    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => {
                if line.len() == 0 {
                    continue;
                }

                if index == 0 {
                    directions = Some(line).unwrap();
                    println!("First line: {}", directions);
                } else if index > 1 {
                    let parts: Vec<&str> = line.splitn(2, '=').collect();
                    let name: String = parts[0].trim().to_string();
                    let parts2: Vec<&str> = parts[1]
                        .trim()
                        .trim_matches('(')
                        .trim_matches(')')
                        .splitn(2, ',')
                        .collect();
                    let left: String = parts2[0].trim().to_string();
                    let right: String = parts2[1].trim().to_string();
                    let map_entry: CamelMapEntry = CamelMapEntry {
                        left: left.clone(),
                        right: right.clone(),
                        name: name.clone(),
                    };
                    camel_map.insert(name.clone(), map_entry);

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
    let mut current_position: String = "AAA".to_string();
    let mut steps: i32 = 0;
    loop {
        for c in directions.chars() {
            if c == 'L' {
                current_position = camel_map.get(&current_position).unwrap().left.clone();
            } else if c == 'R' {
                current_position = camel_map.get(&current_position).unwrap().right.clone();
            }

            steps += 1;

            if current_position == "ZZZ" {
                break;
            }
        }
        if current_position == "ZZZ" {
            break;
        }
    }

    println!("Steps: {}", steps);

    let mut steps_v2: u64 = 0;
    let mut break_out_v2 = false;
    loop {
        for c in directions.chars() {
            travel_v2(c, &camel_map, &mut current_positions_v2);

            steps_v2 += 1;

            if are_we_there_yet_v2(&current_positions_v2) {
                break_out_v2 = true;
                break;
            }
        }
        if break_out_v2 {
            break;
        }
    }

    println!("Steps v2: {}", steps_v2);
}

fn travel_v2(
    direction: char,
    camel_map: &HashMap<String, CamelMapEntry>,
    current_positions: &mut Vec<String>,
) {
    if direction == 'L' {
        for i in 0..current_positions.len() {
            current_positions[i] = camel_map.get(&current_positions[i]).unwrap().left.clone();
        }
    } else if direction == 'R' {
        for i in 0..current_positions.len() {
            current_positions[i] = camel_map.get(&current_positions[i]).unwrap().right.clone();
        }
    }
}

fn are_we_there_yet_v2(current_positions: &Vec<String>) -> bool {
    for position in current_positions {
        if !position.ends_with('Z') {
            return false;
        }
    }

    return true;
}

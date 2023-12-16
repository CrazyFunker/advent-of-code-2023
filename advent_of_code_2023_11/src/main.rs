use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Galaxy {
    x: u32,
    y: u32,
}

fn main() {
    let mut cosmos: Vec<Vec<char>> = Vec::new();
    let mut cosmos_rows_to_duplicate: Vec<u32> = Vec::new();
    let mut galaxies_in_columns: Vec<u32> = Vec::new();
    let mut galaxies: Vec<Galaxy> = Vec::new();

    // read in cosmos
    let path: &Path = Path::new("input.txt");

    let file: File = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        match line {
            Ok(line) => {
                let mut cosmos_line: Vec<char> = Vec::new();
                let mut cosmos_line_with_galaxy: bool = false;
                for (c_index, c) in line.chars().enumerate() {
                    if c == '#' {
                        cosmos_line_with_galaxy = true;
                        if index == 0 {
                            galaxies_in_columns.push(1)
                        } else {
                            galaxies_in_columns[c_index] += 1;
                        }
                    } else {
                        if index == 0 {
                            galaxies_in_columns.push(0)
                        }
                    }
                    cosmos_line.push(c);
                }
                if !cosmos_line_with_galaxy {
                    cosmos_rows_to_duplicate.push(index as u32);
                }

                cosmos.push(cosmos_line);
            }
            Err(error) => {
                println!("Error reading line: {}", error);
            }
        }
    }

    // prepare expanded cosmos
    // expand horizontally
    for cosmos_line in &mut cosmos {
        for i in (0..galaxies_in_columns.len()).rev() {
            if galaxies_in_columns[i] == 0 {
                cosmos_line.insert(i, '.');
            }
        }
    }

    // expand vertically
    for i in (0..cosmos_rows_to_duplicate.len()).rev() {
        let cosmos_line: Vec<char> = cosmos[cosmos_rows_to_duplicate[i] as usize].clone();
        cosmos.insert(cosmos_rows_to_duplicate[i] as usize, cosmos_line);
    }

    // read in every galaxy
    for (index, cosmos_line) in cosmos.iter().enumerate() {
        for (c_index, c) in cosmos_line.iter().enumerate() {
            if *c == '#' {
                galaxies.push(Galaxy {
                    x: c_index as u32,
                    y: index as u32,
                });
            }
        }
    }

    let mut path_sum: u32 = 0;
    // for each pair of galaxies, get the shortest path
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let path: u32 = get_shortest_path(&galaxies[i], &galaxies[j]);
            path_sum += path;
        }
    }

    println!("path_sum: {}", path_sum);
}

fn get_shortest_path(galaxy1: &Galaxy, galaxy2: &Galaxy) -> u32 {
    let mut x: u32;
    let mut y: u32;
    if galaxy1.x > galaxy2.x {
        x = galaxy1.x - galaxy2.x;
    } else {
        x = galaxy2.x - galaxy1.x;
    }

    if galaxy1.y > galaxy2.y {
        y = galaxy1.y - galaxy2.y;
    } else {
        y = galaxy2.y - galaxy1.y;
    }

    return x + y;
}

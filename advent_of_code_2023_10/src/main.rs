use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    let mut start_pos: (i32, i32) = (-1, -1);

    // read the input file
    let path: &Path = Path::new("input.txt");
    let file: File = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);
    let mut current_x: i32 = 0;
    let mut current_y: i32 = 0;
    let mut loop_positions: Vec<(i32, i32)> = Vec::new();

    for line in reader.lines() {
        match line {
            Ok(line) => {
                for c in line.chars() {
                    map.insert((current_x, current_y), c);
                    if c == 'S' {
                        start_pos = (current_x, current_y);
                    }

                    current_x += 1;
                }
            
                current_x = 0;
                current_y += 1;
            }
            Err(error) => {
                println!("Error reading line: {}", error);
            }
        }
    }

    let mut cur_pos: (i32, i32) = start_pos;
    let mut steps_taken: u32 = 0;
    let mut current_c: char = 'S';
    let next_dir_map: HashMap<(char, char), char> = get_next_mode_dir_map();
    let mut cur_dir: char = find_first_step(&cur_pos, &map);

    loop_positions.push(cur_pos.clone());

    cur_pos = get_next_pos_by_dir(&cur_pos, cur_dir);
    steps_taken += 1;
    current_c = map.get(&cur_pos).unwrap().clone();

    while current_c != 'S' {
        loop_positions.push(cur_pos);
        // I was moving [N, E, S, W] and stepped onto char [|, -, L, J, 7, F]
        // Next direction is char
        cur_dir = next_dir_map.get(&(cur_dir, current_c)).unwrap().clone();
        cur_pos = get_next_pos_by_dir(&cur_pos, cur_dir);
        current_c = map.get(&cur_pos).unwrap().clone();

        steps_taken += 1;
    }

    let result = steps_taken / 2;
    println!("Result: {}", result);

    // Part 2
    let mut result_v2 = 0;
    for pos in map.keys() {
        // ignore loop itself
        if loop_positions.contains(pos) {
            continue;
        }
        
        if check_if_point_inside_loop(&pos, &loop_positions) {
            result_v2 += 1;
        }
    }

    println!("Result v2: {}", result_v2);
}

fn check_if_point_inside_loop(pos: &(i32, i32), loop_positions: &Vec<(i32, i32)>) -> bool {
    let max_x: i32 = 140;

    let mut intersections: u32 = 0;

    for i in pos.0..max_x {
        if loop_positions.contains(&(i, pos.1)) {
            intersections += 1;
        }
    }

    return intersections % 2 != 0;
}

fn get_next_pos_by_dir(pos: &(i32, i32), dir: char) -> (i32, i32) {
    let next_pos: (i32, i32) = match dir {
        'N' => (pos.0, pos.1 - 1),
        'E' => (pos.0 + 1, pos.1),
        'S' => (pos.0, pos.1 + 1),
        'W' => (pos.0 - 1, pos.1),
        _ => panic!("Invalid direction"),
    };

    if pos.0 < 0 || pos.1 < 0 {
        panic!("Invalid direction")
    }

    return next_pos;
}

fn find_first_step(pos: &(i32, i32), map: &HashMap<(i32, i32), char>) -> char {
    // check N, E, S, W and if encountered a valid direction, return it
    for c in "NESW".chars() {
        let next_pos: (i32, i32) = get_next_pos_by_dir(pos, c);
        let next_c: char = map.get(&next_pos).unwrap().clone();

        match c {
            'N' => {
                if next_c == '|' || next_c == '7' || next_c == 'F' {
                    return c;
                }
            }
            'E' => {
                if next_c == '-' || next_c == '7' || next_c == 'J' {
                    return c;
                }
            }
            'S' => {
                if next_c == '|' || next_c == 'J' || next_c == 'L' {
                    return c;
                }
            }
            'W' => {
                if next_c == '-' || next_c == 'F' || next_c == 'L' {
                    return c;
                }
            }
            _ => panic!("Invalid direction"),
        }
    }

    panic!("Invalid map");
}

fn get_next_mode_dir_map() -> HashMap<(char, char), char> {
    // I was moving [N, E, S, W] and stepped onto char [|, -, L, J, 7, F]
    // Next direction is char
    let mut map: HashMap<(char, char), char> = HashMap::new();

    // | is a vertical pipe connecting north and south.
    map.insert(('N', '|'), 'N');
    map.insert(('S', '|'), 'S');

    // - is a horizontal pipe connecting east and west.
    map.insert(('E', '-'), 'E');
    map.insert(('W', '-'), 'W');

    // L is a 90-degree bend connecting north and east.
    map.insert(('S', 'L'), 'E');
    map.insert(('W', 'L'), 'N');

    // J is a 90-degree bend connecting north and west.
    map.insert(('S', 'J'), 'W');
    map.insert(('E', 'J'), 'N');

    // 7 is a 90-degree bend connecting south and west.
    map.insert(('N', '7'), 'W');
    map.insert(('E', '7'), 'S');

    // F is a 90-degree bend connecting south and east.
    map.insert(('N', 'F'), 'E');
    map.insert(('W', 'F'), 'S');

    return map;
}

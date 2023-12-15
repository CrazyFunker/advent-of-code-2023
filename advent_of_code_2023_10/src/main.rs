use colored::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpret_lines() {
        let lines = vec![
            String::from("S..."),
            String::from("...."),
            String::from("...."),
            String::from("...."),
        ];
        let mut map: HashMap<(i32, i32), char> = HashMap::new();
        let (start_pos, _max_x) = interpret_lines(&lines, &mut map);
        assert_eq!(start_pos, (0, 0));
        assert_eq!(map.get(&(0, 0)), Some(&'S'));
        assert_eq!(map.get(&(1, 0)), Some(&'.'));
        assert_eq!(map.get(&(0, 1)), Some(&'.'));
        assert_eq!(map.get(&(3, 3)), Some(&'.'));
    }

    #[test]
    fn test_second_part_1() {
        let lines = vec![String::from("S7"), String::from("LJ")];
        let mut map: HashMap<(i32, i32), char> = HashMap::new();
        let (start_pos, max_x) = interpret_lines(&lines, &mut map);

        let mut loop_positions: Vec<(i32, i32)> = Vec::new();
        first_part(start_pos, &map, &mut loop_positions);
        let second_part: i32 = second_part_v2(map, max_x, lines.len() as i32, loop_positions);
        assert_eq!(second_part, 0);
    }

    #[test]
    fn test_second_part_2() {
        let lines = vec![
            String::from("...."),
            String::from(".F7."),
            String::from(".LS."),
            String::from("...."),
        ];
        let mut map: HashMap<(i32, i32), char> = HashMap::new();
        let (start_pos, max_x) = interpret_lines(&lines, &mut map);

        let mut loop_positions: Vec<(i32, i32)> = Vec::new();
        first_part(start_pos, &map, &mut loop_positions);
        let second_part: i32 = second_part_v2(map, max_x, lines.len() as i32, loop_positions);
        assert_eq!(second_part, 0);
    }

    #[test]
    fn test_second_part_3() {
        let lines = vec![
            String::from("....."),
            String::from(".F-7."),
            String::from(".|.|."),
            String::from(".L-S."),
            String::from("....."),
        ];
        let mut map: HashMap<(i32, i32), char> = HashMap::new();
        let (start_pos, max_x) = interpret_lines(&lines, &mut map);

        let mut loop_positions: Vec<(i32, i32)> = Vec::new();
        first_part(start_pos, &map, &mut loop_positions);
        let second_part: i32 = second_part_v2(map, max_x, lines.len() as i32, loop_positions);
        assert_eq!(second_part, 1);
    }

    // result: 4
    #[test]
    fn test_second_part_4() {
        let lines = vec![
            String::from(".........."),
            String::from(".F------7."),
            String::from(".|F----7|."),
            String::from(".||....||."),
            String::from(".||....||."),
            String::from(".|L-7F-J|."),
            String::from(".|..||..|."),
            String::from(".L--JL--S."),
            String::from(".........."),
        ];
        let mut map: HashMap<(i32, i32), char> = HashMap::new();
        let (start_pos, max_x) = interpret_lines(&lines, &mut map);

        let mut loop_positions: Vec<(i32, i32)> = Vec::new();
        first_part(start_pos, &map, &mut loop_positions);
        let second_part: i32 = second_part_v2(map, max_x, lines.len() as i32, loop_positions);
        assert_eq!(second_part, 4);
    }

    // result: 8
    #[test]
    fn test_second_part_5() {
        let lines = vec![
            String::from(".F----7F7F7F7F-7...."),
            String::from(".|F--7||||||||FJ...."),
            String::from(".||.FJ||||||||L7...."),
            String::from("FJL7L7LJLJ||LJ.L-7.."),
            String::from("L--J.L7...LJF7F-7L7."),
            String::from("....F-J..F7FJ|L7L7L7"),
            String::from("....L7.F7||L7|.L7L7|"),
            String::from(".....|FJLJ|FJ|F7|.LJ"),
            String::from("....FJL-7.||.||||..."),
            String::from("....L---J.LJ.LSLJ..."),
        ];
        let mut map: HashMap<(i32, i32), char> = HashMap::new();
        let (start_pos, max_x) = interpret_lines(&lines, &mut map);

        let mut loop_positions: Vec<(i32, i32)> = Vec::new();
        first_part(start_pos, &map, &mut loop_positions);
        let second_part: i32 = second_part_v2(map, max_x, lines.len() as i32, loop_positions);
        assert_eq!(second_part, 8);
    }

    // result: 10
    #[test]
    fn test_second_part_6() {
        let lines = vec![
            String::from("FF7F7F7F7F7F7F7F---7"),
            String::from("L|LJ||||||||||||F--J"),
            String::from("FL-7LSLJ||||||LJL-77"),
            String::from("F--JF--7||LJLJ.F7FJ-"),
            String::from("L---JF-JLJ....FJLJJ7"),
            String::from("|F|F-JF---7...L7L|7|"),
            String::from("|FFJF7L7F-JF7..L---7"),
            String::from("7-L-JL7||F7|L7F-7F7|"),
            String::from("L.L7LFJ|||||FJL7||LJ"),
            String::from("L7JLJL-JLJLJL--JLJ.L"),
        ];
        let mut map: HashMap<(i32, i32), char> = HashMap::new();
        let (start_pos, max_x) = interpret_lines(&lines, &mut map);

        let mut loop_positions: Vec<(i32, i32)> = Vec::new();
        first_part(start_pos, &map, &mut loop_positions);
        let second_part: i32 = second_part_v2(map, max_x, lines.len() as i32, loop_positions);
        assert_eq!(second_part, 10);
    }
}

fn interpret_lines(lines: &Vec<String>, map: &mut HashMap<(i32, i32), char>) -> ((i32, i32), i32) {
    let mut current_x: i32 = 0;
    let mut current_y: i32 = 0;
    let mut max_x: i32 = 0;
    let mut start_pos: (i32, i32) = (-1, -1);

    for line in lines {
        for c in line.chars() {
            map.insert((current_x, current_y), c);
            if c == 'S' {
                start_pos = (current_x, current_y);
            }

            current_x += 1;
            if current_x > max_x {
                max_x = current_x;
            }
        }

        current_x = 0;
        current_y += 1;
    }

    return (start_pos, max_x);
}

fn main() {
    let mut map: HashMap<(i32, i32), char> = HashMap::new();
    let start_pos: (i32, i32);

    // read the input file
    let path: &Path = Path::new("input.txt");
    let file: File = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);
    let mut loop_positions: Vec<(i32, i32)> = Vec::new();
    let mut lines: Vec<String> = Vec::new();
    let max_x: i32;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                lines.push(line.clone());
            }
            Err(error) => {
                println!("Error reading line: {}", error);
            }
        }
    }

    (start_pos, max_x) = interpret_lines(&lines, &mut map);
    let result = first_part(start_pos, &map, &mut loop_positions);
    println!("Result: {}", result);

    let result_v2 = second_part_v2(map, max_x, lines.len() as i32, loop_positions);
    println!("Result v2: {}", result_v2);
}

fn second_part_v2(
    map: HashMap<(i32, i32), char>,
    max_x: i32,
    max_y: i32,
    loop_positions: Vec<(i32, i32)>,
) -> i32 {
    // go char by char from left to right, top to bottom
    let mut result_v2 = 0;

    for y in 0..max_y {
        let mut inside: bool = false;
        let mut needle_dir: char = 'x';
        let mut last_needle_dir: char = 'x';
        for x in 0..max_x {
            let pos: (i32, i32) = (x, y);
            let cur_c: char = map.get(&pos).unwrap().clone();

            if loop_positions.contains(&pos) {
                // we're on top of the loop
                if cur_c == 'L' || cur_c == '7' {
                    needle_dir = 'S';
                }
                if cur_c == 'F' || cur_c == 'J' || cur_c == 'S' {
                    needle_dir = 'N';
                }
                if cur_c == '|' && last_needle_dir != 'x' {
                    if last_needle_dir == 'S' {
                        needle_dir = 'N';
                    } else {
                        needle_dir = 'S';
                    }
                }

                if needle_dir == 'x' {
                    print!("{}", cur_c.to_string().green());
                } else if needle_dir == 'S' {
                    print!("{}", cur_c.to_string().blue());
                } else if needle_dir == 'N' {
                    print!("{}", cur_c.to_string().yellow());
                }

                if cur_c != '-' && cur_c != '|' {
                    if needle_dir != last_needle_dir {
                        inside = !inside;
                        // print!("{}", "~".green().bold());
                    }
                } else if cur_c == '|' {
                    inside = !inside;
                    // print!("{}", "~".green().bold());
                }

                last_needle_dir = needle_dir;
            } else if inside {
                print!("{}", "X".red().bold());
                result_v2 += 1;
            } else {
                print!("{}", cur_c.to_string().dimmed());
            }
        }

        print!("\n");
    }

    return result_v2;
}

fn first_part(
    start_pos: (i32, i32),
    map: &HashMap<(i32, i32), char>,
    loop_positions: &mut Vec<(i32, i32)>,
) -> u32 {
    let mut cur_pos: (i32, i32) = start_pos;
    let mut steps_taken: u32 = 0;
    let mut current_c: char;
    let next_dir_map: HashMap<(char, char), char> = get_next_mode_dir_map();
    let mut cur_dir: char = find_first_step(&cur_pos, map);

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

    return result;
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
        if next_pos.0 < 0 || next_pos.1 < 0 {
            continue;
        }
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

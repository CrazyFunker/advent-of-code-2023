use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Race {
    time: u32,
    distance: u32,
}

fn main() {
    let mut races: Vec<Race> = Vec::new();
    let mut result = 1;
    let result_v2;

    // read the input file
    let path: &Path = Path::new("input.txt");
    let file: File = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);
    let mut times: Vec<u32> = Vec::new();
    let mut distances: Vec<u32> = Vec::new();
    let mut time_v2: u64 = 0;
    let mut distance_v2: u64 = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.len() == 0 {
                    break;
                }
                if line.starts_with("Time:") {
                    times = line.splitn(2, ":").collect::<Vec<&str>>()[1]
                        .split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();
                    time_v2 = line.splitn(2, ":").collect::<Vec<&str>>()[1]
                        .replace(" ", "")
                        .parse::<u64>()
                        .unwrap();
                }
                if line.starts_with("Distance:") {
                    distances = line.splitn(2, ":").collect::<Vec<&str>>()[1]
                        .split_whitespace()
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>();
                    distance_v2 = line.splitn(2, ":").collect::<Vec<&str>>()[1]
                        .replace(" ", "")
                        .parse::<u64>()
                        .unwrap();
                }
            }
            Err(error) => {
                println!("Error reading line: {}", error);
            }
        }
    }

    for i in 0..times.len() {
        races.push(Race {
            time: times[i],
            distance: distances[i],
        })
    }

    for race in races {
        result *= how_many_ways_to_win(race.time as u64, race.distance as u64);
    }

    println!("time_v2: {}, distance_v2: {}", time_v2, distance_v2);
    result_v2 = how_many_ways_to_win(time_v2, distance_v2);

    println!("result: {}", result);
    println!("result_v2: {}", result_v2);
}

fn how_many_ways_to_win(race_t: u64, record_d: u64) -> u64 {
    let mut win_count = 0;
    // button hold time equates to velocity
    for button_hold_time in 0..race_t {
        let sail_time = race_t - button_hold_time;
        let sailed_d = sail_time * button_hold_time;
        if sailed_d > record_d {
            win_count += 1;
        }
    }

    return win_count;
}

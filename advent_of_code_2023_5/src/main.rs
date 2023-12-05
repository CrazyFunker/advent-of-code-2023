use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct XmasMap {
    dst_range_start: u64,
    src_range_start: u64,
    range_length: u64,
}

trait XmasTrait {
    fn map(&self, param: u64) -> u64;
}

// Step 3: Implement the trait for the struct
impl XmasTrait for XmasMap {
    // Step 4: Define the functions declared in the trait
    fn map(&self, param: u64) -> u64 {
        let src_range_end = self.src_range_start + self.range_length;
        if self.src_range_start <= param && param < src_range_end {
            return self.dst_range_start + (param - self.src_range_start);
        } else {
            return param;
        }
    }
}

fn main() {
    let mut seeds: Vec<u64> = Vec::new();
    let mut s2s: Vec<XmasMap> = Vec::new();
    let mut s2f: Vec<XmasMap> = Vec::new();
    let mut f2w: Vec<XmasMap> = Vec::new();
    let mut w2l: Vec<XmasMap> = Vec::new();
    let mut l2t: Vec<XmasMap> = Vec::new();
    let mut t2h: Vec<XmasMap> = Vec::new();
    let mut h2l: Vec<XmasMap> = Vec::new();

    let path: &Path = Path::new("input.txt");
    let file: File = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);
    let mut reading_modes: VecDeque<&str> = vec![
        "seeds",
        "seed-to-soil",
        "soil-to-fertilizer",
        "fertilizer-to-water",
        "water-to-light",
        "light-to-temperature",
        "temperature-to-humidity",
        "humidity-to-location",
    ]
    .into();
    let mut reading_mode: &str = reading_modes.pop_front().unwrap();

    // read the input file
    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line == "" {
                    if reading_modes.len() > 0 {
                        reading_mode = reading_modes.pop_front().unwrap();
                        continue;
                    } else {
                        break;
                    }
                }

                if reading_mode != "seeds" && line.contains(reading_mode) {
                    continue; // skip the header line
                }

                match reading_mode {
                    "seeds" => {
                        let parts: Vec<&str> = line.splitn(2, ':').collect();
                        seeds = parts[1]
                            .split_whitespace()
                            .map(|s| s.parse::<u64>().unwrap())
                            .collect();
                    }
                    "seed-to-soil" => {
                        let parts: Vec<u64> = line
                            .split_whitespace()
                            .map(|s| s.parse::<u64>().unwrap())
                            .collect();
                        let dest_range_start = parts[0];
                        let src_range_start = parts[1];
                        let range_length = parts[2];
                        s2s.push(XmasMap {
                            dst_range_start: dest_range_start,
                            src_range_start: src_range_start,
                            range_length: range_length,
                        });
                    }
                    "soil-to-fertilizer" => {
                        let parts: Vec<u64> = line
                            .split_whitespace()
                            .map(|s| s.parse::<u64>().unwrap())
                            .collect();
                        let dest_range_start = parts[0];
                        let src_range_start = parts[1];
                        let range_length = parts[2];
                        s2f.push(XmasMap {
                            dst_range_start: dest_range_start,
                            src_range_start: src_range_start,
                            range_length: range_length,
                        });
                    }
                    "fertilizer-to-water" => {
                        let parts: Vec<u64> = line
                            .split_whitespace()
                            .map(|s| s.parse::<u64>().unwrap())
                            .collect();
                        let dest_range_start = parts[0];
                        let src_range_start = parts[1];
                        let range_length = parts[2];
                        f2w.push(XmasMap {
                            dst_range_start: dest_range_start,
                            src_range_start: src_range_start,
                            range_length: range_length,
                        });
                    }
                    "water-to-light" => {
                        let parts: Vec<u64> = line
                            .split_whitespace()
                            .map(|s| s.parse::<u64>().unwrap())
                            .collect();
                        let dest_range_start = parts[0];
                        let src_range_start = parts[1];
                        let range_length = parts[2];
                        w2l.push(XmasMap {
                            dst_range_start: dest_range_start,
                            src_range_start: src_range_start,
                            range_length: range_length,
                        });
                    }
                    "light-to-temperature" => {
                        let parts: Vec<u64> = line
                            .split_whitespace()
                            .map(|s| s.parse::<u64>().unwrap())
                            .collect();
                        let dest_range_start = parts[0];
                        let src_range_start = parts[1];
                        let range_length = parts[2];
                        l2t.push(XmasMap {
                            dst_range_start: dest_range_start,
                            src_range_start: src_range_start,
                            range_length: range_length,
                        });
                    }
                    "temperature-to-humidity" => {
                        let parts: Vec<u64> = line
                            .split_whitespace()
                            .map(|s| s.parse::<u64>().unwrap())
                            .collect();
                        let dest_range_start = parts[0];
                        let src_range_start = parts[1];
                        let range_length = parts[2];
                        t2h.push(XmasMap {
                            dst_range_start: dest_range_start,
                            src_range_start: src_range_start,
                            range_length: range_length,
                        });
                    }
                    "humidity-to-location" => {
                        let parts: Vec<u64> = line
                            .split_whitespace()
                            .map(|s| s.parse::<u64>().unwrap())
                            .collect();
                        let dest_range_start = parts[0];
                        let src_range_start = parts[1];
                        let range_length = parts[2];
                        h2l.push(XmasMap {
                            dst_range_start: dest_range_start,
                            src_range_start: src_range_start,
                            range_length: range_length,
                        });
                    }
                    _ => {
                        println!("unrecognised reading_mode: {}", reading_mode);
                    }
                }
            }
            Err(error) => {
                println!("Error reading line: {}", error);
            }
        }
    }

    let mut lowest_location: u64 = std::u64::MAX;
    let seeds_clone = seeds.clone();
    for seed in seeds_clone {
        let soil = map_using_map(seed, &s2s);
        let fertilizer = map_using_map(soil, &s2f);
        let water = map_using_map(fertilizer, &f2w);
        let light = map_using_map(water, &w2l);
        let temperature = map_using_map(light, &l2t);
        let humidity = map_using_map(temperature, &t2h);
        let location = map_using_map(humidity, &h2l);

        if location < lowest_location {
            lowest_location = location;
        }
    }
    println!("lowest_location: {}", lowest_location);

    // ==================== V2 ====================
    let mut lowest_location_v2: u64 = std::u64::MAX;
    let mut seeds_clone_v2 = seeds.clone();
    let seeds_v2: Vec<u64> = find_seeds_v2(&mut seeds_clone_v2);
    println!("found seeds_v2.len(): {}", seeds_v2.len());

    for seed in seeds_v2 {
        let soil = map_using_map(seed, &s2s);
        let fertilizer = map_using_map(soil, &s2f);
        let water = map_using_map(fertilizer, &f2w);
        let light = map_using_map(water, &w2l);
        let temperature = map_using_map(light, &l2t);
        let humidity = map_using_map(temperature, &t2h);
        let location = map_using_map(humidity, &h2l);

        if location < lowest_location_v2 {
            println!("temp lower_loacation_v2: {}", location);
            lowest_location_v2 = location;
        }
    }
    println!("lowest_location_v2: {}", lowest_location_v2);
}

fn find_seeds_v2(seeds: &mut Vec<u64>) -> Vec<u64> {
    let mut seeds_v2: Vec<u64> = Vec::new();

    for _i in 0..seeds.len() / 2 {
        let len = seeds.pop().unwrap();
        let start = seeds.pop().unwrap();

        println!("start: {}, len: {}", start, len);

        for i in start..start + len {
            seeds_v2.push(i);
        }
    }

    println!("found seeds_v2.len(): {}", seeds_v2.len());
    return seeds_v2;
}

fn map_using_map(param: u64, map: &Vec<XmasMap>) -> u64 {
    let mut result: u64 = param;
    for xmas_map in map {
        result = xmas_map.map(result);
        if result != param {
            break;
        }
    }

    return result;
}

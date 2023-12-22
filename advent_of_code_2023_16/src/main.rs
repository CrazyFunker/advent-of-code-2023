use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Beam {
    x: i32,
    y: i32,
    dir: Direction,
}

fn main() {
    // load map from input.txt
    let tiles: Vec<Vec<char>> = load_map();

    // start timer
    let now = Instant::now();
    let mut max_energy: usize = 0;
    // start beam
    // all left side tiles
    for i in 0..tiles.len() {
        let start_beam: Beam = Beam {
            x: 0,
            y: i as i32,
            dir: Direction::Right,
        };
        let energy = get_energised_tiles_num(&tiles, start_beam);
        if energy > max_energy {
            max_energy = energy;
        }
    }
    // all top side tiles
    for i in 0..tiles[0].len() {
        let start_beam: Beam = Beam {
            x: i as i32,
            y: 0,
            dir: Direction::Down,
        };
        let energy = get_energised_tiles_num(&tiles, start_beam);
        if energy > max_energy {
            max_energy = energy;
        }
    }
    // all right side tiles
    for i in 0..tiles.len() {
        let start_beam: Beam = Beam {
            x: tiles[0].len() as i32 - 1,
            y: i as i32,
            dir: Direction::Left,
        };
        let energy = get_energised_tiles_num(&tiles, start_beam);
        if energy > max_energy {
            max_energy = energy;
        }
    }
    // all bottom side tiles
    for i in 0..tiles[0].len() {
        let start_beam: Beam = Beam {
            x: i as i32,
            y: tiles.len() as i32 - 1,
            dir: Direction::Up,
        };
        let energy = get_energised_tiles_num(&tiles, start_beam);
        if energy > max_energy {
            max_energy = energy;
        }
    }

    println!("Time: {:?}", now.elapsed());
    println!("Max energy: {}", max_energy);
}

fn get_energised_tiles_num(tiles: &Vec<Vec<char>>, start_beam: Beam) -> usize {
    // x goes left -> right
    // y goes top -> bottom
    // create first beam and position it at 0,0
    let mut energised_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut memoised_position_dir: HashSet<(i32, i32, Direction)> = HashSet::new();
    let mut beams: Vec<Beam> = Vec::new();
    beams.push(start_beam);
    energised_positions.insert((0, 0));

    // start simulation until all beams are gone
    while beams.len() > 0 {
        // check if any beams are out of bounds or if the current beam position and dir has been seen before
        for i in (0..beams.len()).rev() {
            let beam: &Beam = &beams[i];
            if beam.x < 0
                || beam.y < 0
                || beam.x >= tiles[0].len() as i32
                || beam.y >= tiles.len() as i32
                || memoised_position_dir.contains(&(beam.x, beam.y, beam.dir))
            {
                beams.remove(i);
            }
        }

        // memoise current beam positions and directions
        for beam in beams.iter() {
            memoised_position_dir.insert((beam.x, beam.y, beam.dir));
        }

        let mut new_beams: Vec<Beam> = Vec::new();
        for beam in beams.iter_mut() {
            // update the energised positions
            energised_positions.insert((beam.x, beam.y));
            // check if we need to change direction or split the beams
            let tile = tiles[beam.y as usize][beam.x as usize];

            match tile {
                '.' => {
                    // do nothing
                }
                '/' => match beam.dir {
                    Direction::Up => beam.dir = Direction::Right,
                    Direction::Down => beam.dir = Direction::Left,
                    Direction::Left => beam.dir = Direction::Down,
                    Direction::Right => beam.dir = Direction::Up,
                },
                '\\' => match beam.dir {
                    Direction::Up => beam.dir = Direction::Left,
                    Direction::Down => beam.dir = Direction::Right,
                    Direction::Left => beam.dir = Direction::Up,
                    Direction::Right => beam.dir = Direction::Down,
                },
                '-' => {
                    match beam.dir {
                        Direction::Up => {
                            beam.dir = Direction::Left;
                            // split beam
                            let new_beam: Beam = Beam {
                                x: beam.x,
                                y: beam.y,
                                dir: Direction::Right,
                            };
                            new_beams.push(new_beam);
                        }
                        Direction::Down => {
                            beam.dir = Direction::Left;
                            // split beam
                            let new_beam: Beam = Beam {
                                x: beam.x,
                                y: beam.y,
                                dir: Direction::Right,
                            };
                            new_beams.push(new_beam);
                        }
                        Direction::Left => beam.dir = Direction::Left,
                        Direction::Right => beam.dir = Direction::Right,
                    }
                }
                '|' => {
                    match beam.dir {
                        Direction::Up => beam.dir = Direction::Up,
                        Direction::Down => beam.dir = Direction::Down,
                        Direction::Left => {
                            beam.dir = Direction::Up;
                            // split beam
                            let new_beam: Beam = Beam {
                                x: beam.x,
                                y: beam.y,
                                dir: Direction::Down,
                            };
                            new_beams.push(new_beam);
                        }
                        Direction::Right => {
                            beam.dir = Direction::Up;
                            // split beam
                            let new_beam: Beam = Beam {
                                x: beam.x,
                                y: beam.y,
                                dir: Direction::Down,
                            };
                            new_beams.push(new_beam);
                        }
                    }
                }
                _ => {
                    panic!("Unknown tile: {}", tile);
                }
            }
        }

        // memoise new beams
        for beam in new_beams.iter() {
            memoised_position_dir.insert((beam.x, beam.y, beam.dir));
        }

        // add new beams
        beams.append(&mut new_beams);

        // move all beams
        for beam in beams.iter_mut() {
            match beam.dir {
                Direction::Up => beam.y -= 1,
                Direction::Down => beam.y += 1,
                Direction::Left => beam.x -= 1,
                Direction::Right => beam.x += 1,
            }
        }
    }

    // tiles: 110x110
    // Energised positions: 8323
    // println!("Energised positions: {}", energised_positions.len());

    return energised_positions.len();
}

fn load_map() -> Vec<Vec<char>> {
    let mut tiles: Vec<Vec<char>> = Vec::new();

    let path: &Path = Path::new("input.txt");
    let file: File = File::open(&path).unwrap();
    let reader: io::BufReader<File> = io::BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let mut row: Vec<char> = Vec::new();
        for c in line.chars() {
            row.push(c);
        }
        tiles.push(row);
    }

    println!("tiles: {}x{}", tiles[0].len(), tiles.len());

    tiles
}

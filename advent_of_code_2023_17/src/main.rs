use std::{
    collections::{HashMap, VecDeque},
    ops::Index,
};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct LastDirectionsQueue {
    queue: VecDeque<Direction>,
    limit: usize,
}

impl LastDirectionsQueue {
    fn new(limit: usize) -> Self {
        LastDirectionsQueue {
            queue: VecDeque::new(),
            limit,
        }
    }

    fn get_last_direction(&self) -> Option<Direction> {
        self.queue.back().cloned()
    }

    fn has_repeated_direction(&self, count: usize) -> Option<Direction> {
        let dirs = vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
        for dir in &dirs {
            let mut count_inner: usize = 0;
            for d in &self.queue {
                if d == dir {
                    count_inner += 1;
                }
            }
            if count_inner == count {
                return Some(dir.clone());
            }
        }

        None
    }

    fn push(&mut self, item: Direction) {
        if self.queue.len() == self.limit {
            self.queue.pop_front();
        }
        self.queue.push_back(item);
    }

    fn len(&self) -> usize {
        self.queue.len()
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct CityBlock {
    position: Position,
    heat_loss: u8,
}

fn main() {
    // read in the map
    let map: Vec<Vec<u8>> = std::fs::read_to_string("input.txt")
        .expect("Could not read file")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<u8>().unwrap())
                .collect()
        })
        .collect();

    let w = map[0].len();
    let h = map.len();
    let mut city_block_map: HashMap<Position, CityBlock> = HashMap::new();

    for y in 0..h {
        for x in 0..w {
            let position = Position { x, y };
            let city_block = CityBlock {
                position,
                heat_loss: map[y][x],
            };
            city_block_map.insert(position, city_block.clone());
        }
        println!();
    }

    println!("map: {}x{}", map[0].len(), map.len());

    // Dijkstra's algorithm
    let mut past_moves_directions: LastDirectionsQueue = LastDirectionsQueue::new(3);
    let start_position = Position { x: 0, y: 0 };
    let end_position = Position { x: w - 1, y: h - 1 };
    let dirs = vec![
        Direction::Up,
        Direction::Down,
        Direction::Left,
        Direction::Right,
    ];

    // A* Search Algorithm
    // 1.  Initialize the open list
    let mut open_list: Vec<Position> = Vec::new();
    // 2.  Initialize the closed list
    let mut closed_list: Vec<Position> = Vec::new();
    //     put the starting node on the open
    //     list (you can leave its f at zero)
    open_list.push(start_position);

    // 3.  while the open list is not empty
    while !open_list.is_empty() {
        //     a) find the node with the least f on
        //        the open list, call it "q"
        let pos_least_f = open_list
            .iter()
            .min_by_key(|p| calculate_f(p, &end_position, &city_block_map))
            .unwrap();
        let i = open_list.iter().position(|p| p == pos_least_f).unwrap();
        //     b) pop q off the open list
        let q = open_list.remove(i);

        //     c) generate q's 3 successors and set their
        //        parents to q
        // ignore moving backwards and direction repeating 4 times
        let mut successors: Vec<(Direction, Position)> = Vec::new();
        for dir in &dirs {
            let mut position = q.clone();
            match dir {
                Direction::Up => {
                    if position.y > 0 {
                        position.y -= 1;
                    } else {
                        continue;
                    }
                }
                Direction::Down => {
                    if position.y < h - 1 {
                        position.y += 1;
                    } else {
                        continue;
                    }
                }
                Direction::Left => {
                    if position.x > 0 {
                        position.x -= 1;
                    } else {
                        continue;
                    }
                }
                Direction::Right => {
                    if position.x < w - 1 {
                        position.x += 1;
                    } else {
                        continue;
                    }
                }
            }
            successors.push((dir.clone(), position));
        }

        match past_moves_directions.has_repeated_direction(3) {
            Some(Direction::Up) => {
                // can't go up
                successors.retain(|(dir, _)| dir != &Direction::Up);
            }
            Some(Direction::Down) => {
                // can't go down
                successors.retain(|(dir, _)| dir != &Direction::Down);
            }
            Some(Direction::Left) => {
                // can't go lelf
                successors.retain(|(dir, _)| dir != &Direction::Left);
            }
            Some(Direction::Right) => {
                // can't go right
                successors.retain(|(dir, _)| dir != &Direction::Right);
            }
            None => {
                // can go anywhere
            }
        }

        // look up last move direction
        let last_move_direction: Option<Direction> = past_moves_directions.get_last_direction();
        match last_move_direction {
            Some(Direction::Up) => {
                // can't go down
                successors.retain(|(dir, _)| dir != &Direction::Down);
            }
            Some(Direction::Down) => {
                // can't go up
                successors.retain(|(dir, _)| dir != &Direction::Up);
            }
            Some(Direction::Left) => {
                // can't go right
                successors.retain(|(dir, _)| dir != &Direction::Right);
            }
            Some(Direction::Right) => {
                // can't go left
                successors.retain(|(dir, _)| dir != &Direction::Left);
            }
            None => {
                // can go anywhere
            }
        }

        //     d) for each successor
        for successor in successors {
            //         i) if successor is the goal, stop search
            if successor.1 == end_position {
                //         i) if successor is the goal, stop search
                println!("found goal");
                break;
            } else {
                //         ii) else, compute both g and h for successor
                //           successor.g = q.g + distance between
                //                               successor and q
                //           successor.h = distance from goal to
                //           successor (This can be done using many
                //           ways, we will discuss three heuristics-
                //           Manhattan, Diagonal and Euclidean
                //           Heuristics)

                //           successor.f = successor.g + successor.h
                let f = calculate_f(&successor.1, &end_position, &city_block_map);
                //         iii) if a node with the same position as
                //             successor is in the OPEN list which has a
                //            lower f than successor, skip this successor
                if open_list
                    .iter()
                    .any(|p| calculate_f(p, &end_position, &city_block_map) < f)
                {
                    continue;
                }

                //         iV) if a node with the same position as
                //             successor  is in the CLOSED list which has
                //             a lower f than successor, skip this successor
                if closed_list.iter().any(|p| {
                    p == &successor.1 && calculate_f(p, &end_position, &city_block_map) < f
                }) {
                    continue;
                }

                //             otherwise, add  the node to the open list
                open_list.push(successor.1);
                past_moves_directions.push(successor.0);

                //      end (for loop)
            }
        }

        //     e) push q on the closed list
        closed_list.push(q);
        //     end (while loop)
    }
}

fn calculate_f(
    position: &Position,
    end_position: &Position,
    city_block_map: &HashMap<Position, CityBlock>,
) -> u32 {
    let city_block = city_block_map.get(position).unwrap();
    let g = city_block.heat_loss;
    let h = calculate_h(position, end_position);
    g as u32 + h
}

// Manhattan Distance
fn calculate_h(position: &Position, end_position: &Position) -> u32 {
    let x = (position.x as i32 - end_position.x as i32).abs();
    let y = (position.y as i32 - end_position.y as i32).abs();
    (x + y) as u32
}

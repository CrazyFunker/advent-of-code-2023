use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct Card {
    card_number: u32,
    match_count: u32,
    points_won: i32,
    copies: u64,
}

fn main() {
    // read line by line
    let path: &Path = Path::new("input.txt");
    let file: File = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);
    let mut cards: Vec<Card> = Vec::new();
    let mut points_won: u32 = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                // Step 1 & 2: Split the line and extract game number
                let parts: Vec<&str> = line.splitn(2, ':').collect();
                let card_number: u32 = parts[0].replace("Card ", "").trim().parse::<u32>().unwrap();
                let all_numbers: Vec<&str> = parts[1].splitn(2, '|').collect();

                // numbers on left of | are winning numbers
                let winning_numbers: Vec<i32> = all_numbers[0]
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();

                // numbers on right of | are my numbers
                let my_numbers: Vec<i32> = all_numbers[1]
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();

                let mut match_count: u32 = 0;
                for &num in &winning_numbers[..] {
                    if my_numbers.contains(&num) {
                        match_count += 1;
                    }
                }

                let mut points_won_for_game: u32 = 0;
                if match_count > 0 {
                    // points won = 2^(number of found winning numbers-1)
                    points_won_for_game = 2u32.pow(match_count - 1);
                }

                cards.push(Card {
                    card_number,
                    match_count,
                    points_won: points_won_for_game as i32,
                    copies: 1,
                });
            }
            Err(error) => {
                println!("Error reading line: {}", error);
            }
        }
    }
    
    let num_cards: usize = cards.len();
    let mut copied_cards: Vec<Card> = cards.clone();
    
    // process copies of scratch cards
    for card in &mut cards[..] {
        if card.match_count > 0 {
            // increase copies of next cards by 1
            let start_index: usize = card.card_number as usize;
            for i in start_index..start_index + card.match_count as usize {
                if i < num_cards {
                    copied_cards[i].copies += copied_cards[start_index - 1].copies;
                }
            }
        }
    }
    
    // sum all points won
    for card in cards {
        points_won += card.points_won as u32;
    }
    
    // sum up all copies of cards
    let mut total_copies: u64 = 0;
    for card in &copied_cards[..] {
        total_copies += card.copies as u64;
    }

    println!("points_won: {}", points_won);
    println!("total_copies: {}", total_copies);
}

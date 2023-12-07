use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: u32,
    rank: u32,
    jokers: u32,
    joker_rank: u32,
}

fn main() {
    let mut hands: Vec<Hand> = Vec::new();
    let mut total_winnings: u32 = 0;
    let mut total_winnings_v2: u32 = 0;
    // read all hands and bids from file

    // read the input file
    let path: &Path = Path::new("input.txt");
    let file: File = File::open(&path).unwrap();
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line == "" {
                    continue;
                }

                let parts: Vec<&str> = line.split_whitespace().collect();

                // calculate rank
                let cards = parts[0].to_string();
                let cards_count = count_chars(&cards);
                let rank = get_card_rank(&cards_count);

                // calculate jokers by looking up cards count for 'J'
                let jokers = cards_count.get(&'J').unwrap_or(&0);
                let joker_rank = get_card_rank_v2(&cards_count, jokers.to_owned());

                let hand = Hand {
                    cards,
                    bid: parts[1].parse::<u32>().unwrap(),
                    rank,
                    jokers: *jokers,
                    joker_rank: joker_rank,
                };
                println!("{:?}", hand);

                hands.push(hand);
            }
            Err(error) => {
                println!("Error reading line: {}", error);
            }
        }
    }

    // sort by rank
    hands.sort_by(compare_hands);

    // calculate winnings
    for i in 1..=hands.len() {
        total_winnings += (i as u32) * hands[i - 1].bid;
    }

    println!("Total winnings: {}", total_winnings);

    // sort by rank
    hands.sort_by(compare_hands_v2);

    // calculate winnings
    for i in 1..=hands.len() {
        total_winnings_v2 += (i as u32) * hands[i - 1].bid;
    }

    // not 250_927_840 250927840
    println!("Total winnings v2: {}", total_winnings_v2);
}

fn compare_hands_v2(hand1: &Hand, hand2: &Hand) -> Ordering {
    if hand1.joker_rank > hand2.joker_rank {
        return Ordering::Greater;
    }

    if hand1.joker_rank < hand2.joker_rank {
        return Ordering::Less;
    }

    // if ranks are equal, compare cards from left to right
    for i in 0..5 {
        let dif = card_char_to_number(hand1.cards.chars().nth(i).unwrap())
            - card_char_to_number(hand2.cards.chars().nth(i).unwrap());

        if dif > 0 {
            return Ordering::Greater;
        }
        if dif < 0 {
            return Ordering::Less;
        }
    }

    return Ordering::Equal;
}

fn compare_hands(hand1: &Hand, hand2: &Hand) -> Ordering {
    if hand1.rank > hand2.rank {
        return Ordering::Greater;
    }

    if hand1.rank < hand2.rank {
        return Ordering::Less;
    }

    // if ranks are equal, compare cards from left to right
    for i in 0..5 {
        let dif = card_char_to_number(hand1.cards.chars().nth(i).unwrap())
            - card_char_to_number(hand2.cards.chars().nth(i).unwrap());

        if dif > 0 {
            return Ordering::Greater;
        }
        if dif < 0 {
            return Ordering::Less;
        }
    }

    return Ordering::Equal;
}

fn card_char_to_number(c: char) -> i32 {
    match c {
        'T' => 10,
        'J' => 11,
        'Q' => 12,
        'K' => 13,
        'A' => 14,
        _ => c.to_digit(10).unwrap() as i32,
    }
}

fn card_char_to_number_v2(c: char) -> i32 {
    match c {
        'J' => 1,
        'T' => 10,
        'Q' => 11,
        'K' => 12,
        'A' => 13,
        _ => c.to_digit(10).unwrap() as i32,
    }
}

fn count_chars(s: &str) -> HashMap<char, u32> {
    let mut counts = HashMap::new();
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    counts
}

fn get_card_rank(cards: &HashMap<char, u32>) -> u32 {
    // 7 Five of a kind, where all five cards have the same label: AAAAA
    // 6 Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    // 5 Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    // 4 Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    // 3 Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    // 2 One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    // 1 High card, where all cards' labels are distinct: 23456
    if cards.keys().len() == 1 {
        // five of a kind
        return 7;
    }

    if cards.keys().len() == 2 {
        // four of a kind or full house
        for (_, count) in cards.iter() {
            if *count == 4 {
                return 6;
            }
        }
        return 5;
    }

    if cards.keys().len() == 3 {
        // three of a kind or two pair
        for (_, count) in cards.iter() {
            if *count == 3 {
                return 4;
            }
        }
        return 3;
    }

    if cards.keys().len() == 4 {
        // one pair
        return 2;
    }

    if cards.keys().len() == 5 {
        // high card
        return 1;
    }

    return 0;
}

fn get_card_rank_v2(cards: &HashMap<char, u32>, jokers: u32) -> u32 {
    // 7 Five of a kind, where all five cards have the same label: AAAAA
    // 6 Four of a kind, where four cards have the same label and one card has a different label: AA8AA
    // 5 Full house, where three cards have the same label, and the remaining two cards share a different label: 23332
    // 4 Three of a kind, where three cards have the same label, and the remaining two cards are each different from any other card in the hand: TTT98
    // 3 Two pair, where two cards share one label, two other cards share a second label, and the remaining card has a third label: 23432
    // 2 One pair, where two cards share one label, and the other three cards have a different label from the pair and each other: A23A4
    // 1 High card, where all cards' labels are distinct: 23456

    if cards.keys().len() == 1 {
        // five of a kind
        return 7;
    }

    // 22233
    // 22223
    // 222JJ -> 7 Five of a kind
    // 2222J -> 7 Five of a kind
    if cards.keys().len() == 2 {
        if jokers > 0 {
            // five of a kind
            return 7;
        }

        // four of a kind or full house
        for (_, count) in cards.iter() {
            if *count == 4 {
                // four of a kind
                return 6;
            }
        }

        // full house
        return 5;
    }

    // 22234
    // 22334
    // 2223J -> 6 four of kind
    // 22JJ4 -> 6 four of kind
    if cards.keys().len() == 3 {
        if jokers > 0 {
            // four of a kind
            return 6;
        }

        // three of a kind or two pair
        for (_, count) in cards.iter() {
            if *count == 3 {
                return 4;
            }
        }
        return 3;
    }

    // 23455
    // J3455 -> 3 two pair
    // 234JJ -> 4 three of a kind
    if cards.keys().len() == 4 {
        if jokers == 1 {
            // two pair
            return 3;
        }

        if jokers == 2 {
            // three of a kind
            return 4;
        }

        // one pair
        return 2;
    }

    // 23456
    // 2345J -> 2 one pair
    if cards.keys().len() == 5 {
        if jokers == 1 {
            // one pair
            return 2;
        }

        // high card
        return 1;
    }

    return 0;
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_card_rank() {
        let cards1 = count_chars("AAAAA");
        assert_eq!(get_card_rank(&cards1), 7);

        let cards2 = count_chars("AA8AA");
        assert_eq!(get_card_rank(&cards2), 6);

        let cards3 = count_chars("23332");
        assert_eq!(get_card_rank(&cards3), 5);

        let cards4 = count_chars("TTT98");
        assert_eq!(get_card_rank(&cards4), 4);

        let cards5 = count_chars("23432");
        assert_eq!(get_card_rank(&cards5), 3);

        let cards6 = count_chars("A23A4");
        assert_eq!(get_card_rank(&cards6), 2);

        let cards7 = count_chars("23456");
        assert_eq!(get_card_rank(&cards7), 1);
    }

    #[test]
    fn test_get_card_rank_v2() {
        let cards1 = count_chars("AAAAA");
        assert_eq!(get_card_rank_v2(&cards1, 0), 7);

        let cards2 = count_chars("AA8AA");
        assert_eq!(get_card_rank_v2(&cards2, 0), 6);

        let cards3 = count_chars("23332");
        assert_eq!(get_card_rank_v2(&cards3, 0), 5);

        let cards4 = count_chars("TTT98");
        assert_eq!(get_card_rank_v2(&cards4, 0), 4);

        let cards5 = count_chars("23432");
        assert_eq!(get_card_rank_v2(&cards5, 0), 3);

        let cards6 = count_chars("A23A4");
        assert_eq!(get_card_rank_v2(&cards6, 0), 2);

        let cards7 = count_chars("23456");
        assert_eq!(get_card_rank_v2(&cards7, 0), 1);

        let cards8 = count_chars("222JJ");
        assert_eq!(get_card_rank_v2(&cards8, 1), 7);

        let cards9 = count_chars("2222J");
        assert_eq!(get_card_rank_v2(&cards9, 1), 7);

        let cards10 = count_chars("2223J");
        assert_eq!(get_card_rank_v2(&cards10, 0), 6);

        let cards11 = count_chars("22JJ4");
        assert_eq!(get_card_rank_v2(&cards11, 0), 6);

        let cards12 = count_chars("J3455");
        assert_eq!(get_card_rank_v2(&cards12, 1), 3);

        let cards13 = count_chars("234JJ");
        assert_eq!(get_card_rank_v2(&cards13, 0), 4);

        let cards14 = count_chars("23455");
        assert_eq!(get_card_rank_v2(&cards14, 1), 3);

        let cards15 = count_chars("23456");
        assert_eq!(get_card_rank_v2(&cards15, 1), 2);

        let cards16 = count_chars("2345J");
        assert_eq!(get_card_rank_v2(&cards16, 1), 2);
    }
}

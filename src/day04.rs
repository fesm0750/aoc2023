//! Day 04: Scratchcards
//!
//! Link to the challenge: https://adventofcode.com/2023/day/1

use std::{collections::HashSet, fs};
pub fn run() {
    let input = fs::read_to_string("inputs/day04").unwrap();
    let cards = parse_input(&input);

    // part 1
    let total_points: u32 = cards.iter().map(|c| c.points()).sum();
    println!("Part 01: Total points: {}", total_points);

    // part 2
    let total_cards: u32 = process_card_pile(&cards);
    println!("Part 02: Total cards: {}", total_cards);
}

fn parse_input(s: &str) -> Vec<Scratchcard> {
    let mut cards = Vec::<Scratchcard>::new();

    // Example line: "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53"
    for line in s.lines() {
        let mut iter = line.strip_prefix("Card").unwrap().split([':', '|']);
        let id: usize = iter.next().unwrap().trim().parse().unwrap();

        let mut parse_number_sequence = || -> HashSet<u32> {
            iter.next()
                .unwrap()
                .trim()
                .split_ascii_whitespace()
                .flat_map(str::parse)
                .collect()
        };

        let win: HashSet<u32> = parse_number_sequence();
        let have: HashSet<u32> = parse_number_sequence();

        cards.push(Scratchcard::new(id, &win, &have));
    }

    cards
}

fn process_card_pile(cards: &[Scratchcard]) -> u32 {
    let mut card_pile = vec![1; cards.len()];

    for card in cards {
        let n_cards = card_pile[card.id - 1];
        (card.id..card.id + card.matches as usize).for_each(|i| card_pile[i] += n_cards);
    }

    card_pile.iter().sum()
}

struct Scratchcard {
    id: usize,
    matches: u32,
}

impl Scratchcard {
    fn new(id: usize, win: &HashSet<u32>, have: &HashSet<u32>) -> Scratchcard {
        Scratchcard {
            id,
            matches: win.intersection(have).count() as u32,
        }
    }

    fn points(&self) -> u32 {
        if self.matches > 0 {
            2u32.pow(self.matches - 1)
        } else {
            0
        }
    }
}

//----------
// Tests
//----------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let cards = parse_input(input);

        // part 1
        assert_eq!(cards[0].points(), 8);
        assert_eq!(cards[1].points(), 2);
        assert_eq!(cards[3].points(), 1);
        assert_eq!(cards[5].points(), 0);

        // part 2
        assert_eq!(process_card_pile(&cards), 30);
    }
}

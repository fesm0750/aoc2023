//! Day 04: Scratchcards
//!
//! Link to the challenge: https://adventofcode.com/2023/day/4
//!
//! # Problem
//!
//! A Scratchcard consists of a set of winning numbers and a set of lottery numbers.
//!
//! From an input file containing lines of ascii text representing scratchcards:
//!
//! 1. Find the total points from all scratchcards.
//!     - Each card's score is determined by the matches between lottery and winning numbers: "The first match makes the
//!       card worth one point and each match after the first doubles the point value of that card.".
//!
//! 2. Determine the final number of cards.
//!     - Each match results in winning a copy of a scratchcard with an id  higher than the current card's id. For
//!       instance: "if card 10 were to have 5 matching numbers, you would win one copy each of cards 11, 12, 13, 14,
//!       and 15."
//!
//! # Solution
//!
//! - Using `Hashset` to find the matches between lottery and winning numbers;
//!
//! - Employing an array to track the quantities of each card by id.

use std::{collections::HashSet, error, fs, str::FromStr};
pub fn run() {
    let input = fs::read_to_string("inputs/day04").unwrap();
    let cards: Vec<Scratchcard> = input.lines().flat_map(str::parse).collect();

    // part 1
    let total_points: u32 = cards.iter().map(|c| c.points()).sum();
    println!("Part 01: Total points: {}", total_points);

    // part 2
    let total_cards: u32 = process_card_pile(&cards);
    println!("Part 02: Total cards: {}", total_cards);
}

//----------
// Helper methods
//----------

/// Returns the final number of cards based on the rules defined in part 2.
/// @param `cards`: reference to an array of `Scratchcards` ordered by `id`. The `id`s must be sequential, starting at
/// 1, to ensure accurate processing and output calculation.
fn process_card_pile(cards: &[Scratchcard]) -> u32 {
    // Occurrence array. Initially, we have one of each card.
    let mut card_pile = vec![1; cards.len()];
    for card in cards {
        // `card.id` starts at one instead of zero.
        let n_cards = card_pile[card.id - 1];
        (card.id..card.id + card.matches as usize).for_each(|i| card_pile[i] += n_cards);
    }
    card_pile.iter().sum()
}

//----------
// Scratchcard structs
//----------

/// Stores a scratchcard `id` and the number of `matches` between lottery and winning numbers.
#[derive(Debug, PartialEq)]
struct Scratchcard {
    id: usize,
    matches: u32,
}

impl Scratchcard {
    /// Calculates the point value of the `Scratchcard`,
    fn points(&self) -> u32 {
        if self.matches > 0 {
            2u32.pow(self.matches - 1)
        } else {
            0
        }
    }
}

impl FromStr for Scratchcard {
    type Err = Box<dyn error::Error>;

    /// Parses a Scratchcard.
    /// @param `s`: String expected in the format: "Card {id}: {winning numbers} | {lottery numbers}". Here, winning and
    /// lottery numbers are sequences of unsigned integers separated by whitespace.
    /// A valid example is: "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s
            .strip_prefix("Card")
            .ok_or("`Card` prefix not found.")?
            .split([':', '|']);
        let id: usize = iter.next().ok_or("Not able to get `ID`.")?.trim().parse()?;

        let mut parse_number_sequence = || -> Result<HashSet<u32>, Self::Err> {
            Ok(iter
                .next()
                .ok_or("Not able to find a number sequence.")?
                .split_ascii_whitespace()
                .map(str::parse)
                .collect::<Result<HashSet<u32>, _>>()?)
        };

        let win = parse_number_sequence()?;
        let lottery = parse_number_sequence()?;

        Ok(Scratchcard {
            id,
            matches: win.intersection(&lottery).count() as u32,
        })
    }
}

//----------
// Tests
//----------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_scratchcard() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        assert_eq!(input.parse::<Scratchcard>().unwrap(), Scratchcard { id: 1, matches: 4 });
    }

    #[test]
    fn tests() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        let cards: Vec<Scratchcard> = input.lines().flat_map(str::parse).collect();

        // part 1
        assert_eq!(cards[0].points(), 8);
        assert_eq!(cards[1].points(), 2);
        assert_eq!(cards[3].points(), 1);
        assert_eq!(cards[5].points(), 0);

        // part 2
        assert_eq!(process_card_pile(&cards), 30);
    }
}

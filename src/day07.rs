use std::{cmp::Ordering, error, fs, str::FromStr};

use HandType::*;

pub fn run() {
    let input = fs::read_to_string("inputs/day07").unwrap();
    let hands = parse_input(&input);

    // Part 1
    let total_pt1 = total_winnings(&hands);
    println!("Part 1: Total winnings: {total_pt1}");

    // Part 2
    let joker_hands = into_joker_hands(hands);
    let total_pt2 = total_winnings(&joker_hands);
    println!("Part 2: Total winnings: {total_pt2}");
}

//----------
// Helper Methods
//----------

fn parse_input(input: &str) -> Vec<Hand> {
    let mut hands = input.lines().flat_map(str::parse).collect::<Vec<Hand>>();
    hands.sort_unstable();
    hands
}

fn total_winnings(hands: &[Hand]) -> u64 {
    hands.iter().enumerate().map(|(i, h)| (i as u64 + 1) * h.bid).sum()
}

fn into_joker_hands(mut hands: Vec<Hand>) -> Vec<Hand> {
    hands.iter_mut().for_each(|card| card.change_to_joker_hand());
    hands.sort_unstable();
    hands
}

//----------
// Structs and Enums
//----------

#[derive(PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
    h_type: HandType,
    bid: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfKind,
    FullHouse,
    FourOfKind,
    FiveOfKind,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum Card {
    A,
    K,
    Q,
    J,
    T,
    N(u8),
    Joker,
}

//----------
// Implementations
//----------

impl Card {
    fn discriminant(&self) -> u8 {
        // SAFETY: https://doc.rust-lang.org/std/mem/fn.discriminant.html
        unsafe { *<*const _>::from(self).cast::<u8>() }
    }
}

impl Hand {
    fn new(cards: [Card; 5], bid: u64) -> Hand {
        Hand {
            cards,
            bid,
            h_type: HandType::new(&cards),
        }
    }

    fn change_to_joker_hand(&mut self) {
        self.cards.iter_mut().for_each(|card| {
            if *card == Card::J {
                *card = Card::Joker;
            }
        });

        // change hand type
        let count_jokers = self.cards.iter().filter(|&&card| card == Card::Joker).count();
        self.h_type = match (count_jokers, self.h_type) {
            (0, _) => self.h_type,
            (1, HighCard) => OnePair,
            (1, OnePair) => ThreeOfKind,
            (1, TwoPair) => FullHouse,
            (1, ThreeOfKind) => FourOfKind,
            (1, FourOfKind) => FiveOfKind,
            (1, _) => unreachable!(),
            (2, OnePair) => ThreeOfKind,
            (2, TwoPair) => FourOfKind,
            (2, FullHouse) => FiveOfKind,
            (2, _) => unreachable!(),
            (3, ThreeOfKind) => FourOfKind,
            (3, FullHouse) => FiveOfKind,
            (3, _) => unreachable!(),
            (4, _) => FiveOfKind,
            (_, FiveOfKind) => FiveOfKind,
            _ => unreachable!(),
        };
    }
}

impl HandType {
    fn new(cards: &[Card; 5]) -> HandType {
        let mut cards = *cards;
        cards.sort();

        let equal_pairs_iter = || cards[0..4].iter().zip(cards[1..5].iter()).filter(|(a, b)| a == b);

        let equal_pairs_first_and_last = || {
            let mut pairs = equal_pairs_iter();
            (pairs.next().unwrap(), pairs.next_back().unwrap())
        };

        let pairs_count = equal_pairs_iter().count();

        match pairs_count {
            0 => Self::HighCard,
            1 => Self::OnePair,
            2 => {
                let (f, l) = equal_pairs_first_and_last();
                if f != l {
                    Self::TwoPair
                } else {
                    Self::ThreeOfKind
                }
            }
            3 => {
                let (f, l) = equal_pairs_first_and_last();
                if f != l {
                    Self::FullHouse
                } else {
                    Self::FourOfKind
                }
            }
            4 => Self::FiveOfKind,
            _ => unreachable!(),
        }
    }
}

//----------
// FromStr
//----------

/// example
/// "32T3K 765"
/// "T55J5 684"
impl FromStr for Hand {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').ok_or("Not able to find cards and bids.")?;

        //? not able to use the ? operator here
        let cards = cards
            .chars()
            .flat_map(|c| c.to_string().parse::<Card>())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Ok(Hand::new(cards, bid.parse()?))
    }
}

impl FromStr for Card {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Card::A,
            "K" => Card::K,
            "Q" => Card::Q,
            "J" => Card::J,
            "T" => Card::T,
            _ if (b'2'..=b'9').contains(&s.as_bytes()[0]) => Card::N(s.parse()?),
            _ => Err("Not able to parse `Card`.")?,
        })
    }
}

//----------
// Order Traits
//----------

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.h_type.cmp(&other.h_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                let pair = self.cards.iter().zip(other.cards.iter()).find(|(a, b)| a != b);
                if pair.is_none() {
                    Ordering::Equal
                } else {
                    let (a, b) = pair.unwrap();
                    a.cmp(b)
                }
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.discriminant();
        let b = other.discriminant();

        // smaller discriminants represent the highest cards
        if a < b {
            Ordering::Greater
        } else if a > b {
            Ordering::Less
        } else if self == other {
            Ordering::Equal
        } else {
            // case both are N, but payloads differ
            if let (Card::N(a), Card::N(b)) = (self, other) {
                a.cmp(b)
            } else {
                unreachable!()
            }
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

//----------
// Test
//----------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        // part 1
        let hands = parse_input(input);
        assert_eq!(total_winnings(&hands), 6440);

        // part 2
        let joker_hands = into_joker_hands(hands);
        assert_eq!(total_winnings(&joker_hands), 5905);
    }
}

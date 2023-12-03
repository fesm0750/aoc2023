//! Day 02
//!
//! Link of the challenge: https://adventofcode.com/2023/day/2
//!
//! # Problem:
//!
//! A game consist of 3 sets of cubes of different colors (Red, Green and Blue), for each round we must keep track of
//! the maximum amount of cubes from each color that have been showed.
//! - A game is valid if the maximum amount of cubes is below a limit for that color.
//! - The "power" of a game is given by the multiplication of the maximum amount of cubes registered for each color.
//!
//! From an input file containing lines of ascii text representing records of games.
//!
//! 1. Find the sum of valid game ids;
//!
//! 2. Find the sum of "powers" of all recorded games.

use std::{error, fs, str::FromStr};
use Color::*;

pub fn run() {
    let input = fs::read_to_string("inputs/day02").unwrap();
    let games = parse_input(&input).unwrap();

    println!("Part 01: Sum of Valid games IDs: {}", sum_valid(&games));
    println!("Part 02: Sum of Powers: {}", sum_powers(&games));
}

/// Parses the input string into a collection of `Game`s
/// @param input: reference to a string containing records of games.
fn parse_input(input: &str) -> Option<Vec<Game>> {
    let mut games: Vec<Game> = Vec::new();

    for line in input.lines() {
        // Example line: "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"
        // Structure: Game {id}: records{[n Color], [n Color]; [n Color]}

        // Get Game id
        let mut iter = line.split(": ");
        let id: u32 = iter.next()?.split_ascii_whitespace().nth(1)?.parse().unwrap();

        // create game struct
        let mut game = Game::new(id);

        // break game record and parse into `Cube`s, then update `Game` struct
        let iter_cubes_str = iter.next()?.split([',', ';']);
        for cube in iter_cubes_str {
            let cube = cube.trim().parse().unwrap();
            game.update(cube);
        }

        games.push(game);
    }

    Some(games)
}

/// returns the sum of `id`s of valid games.
/// @param `games`: a reference to an array of `Game`s
fn sum_valid(games: &[Game]) -> u32 {
    games.iter().filter(|g| g.is_valid()).map(|g| g.id).sum()
}

/// returns the sum of "powers" of games.
/// @param `games`: a reference to an array of `Game`s
fn sum_powers(games: &[Game]) -> u32 {
    games.iter().map(|g| g.power()).sum()
}

//----------
// Structs and enums
//----------

/// Represents the possible colors of the cubes.
#[derive(Copy, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

// Intermediary struct to parse a game record, it stores the color and amount of cubes.
#[derive(Copy, Clone)]
struct Cube {
    color: Color,
    quantity: u32,
}

/// Stores a game id and the max amount of each kind of cube showed.
struct Game {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32,
}

//----------
// Game Implementation
//----------

impl Game {
    //---
    // Limits for each color
    //---
    const LIMIT_RED: u32 = 12;
    const LIMIT_GREEN: u32 = 13;
    const LIMIT_BLUE: u32 = 14;

    /// Initialises a new struct given the id.
    fn new(id: u32) -> Game {
        Game {
            id,
            max_red: 0,
            max_green: 0,
            max_blue: 0,
        }
    }

    /// Updates the game from a cube record
    /// @param `cube`: a cube struct.
    fn update(&mut self, cube: Cube) {
        match cube.color {
            Red => {
                if cube.quantity > self.max_red {
                    self.max_red = cube.quantity
                }
            }
            Green => {
                if cube.quantity > self.max_green {
                    self.max_green = cube.quantity
                }
            }
            Blue => {
                if cube.quantity > self.max_blue {
                    self.max_blue = cube.quantity
                }
            }
        }
    }

    /// Returns a boolean indicating if the game is valid.
    /// A game is valid if the maximum amount of each color is below the limit.
    fn is_valid(&self) -> bool {
        self.max_red <= Game::LIMIT_RED && self.max_green <= Game::LIMIT_GREEN && self.max_blue <= Game::LIMIT_BLUE
    }

    /// Returns the power of the game.
    fn power(&self) -> u32 {
        self.max_red * self.max_green * self.max_blue
    }
}

//----------
// FromStr
//
// Parsing methods for enums and structs
//----------

impl FromStr for Color {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(Red),
            "green" => Ok(Green),
            "blue" => Ok(Blue),
            _ => Err("Error parsing color.".to_owned()),
        }
    }
}

impl FromStr for Cube {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_ascii_whitespace();

        Ok(Cube {
            quantity: iter.next().ok_or("Error getting quantity.")?.parse()?,
            color: iter.next().ok_or("Error getting color")?.parse()?,
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
    fn tests() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let games = parse_input(&input).unwrap();

        // Part 01
        assert_eq!(sum_valid(&games), 8);

        // Part 02
        assert_eq!(sum_powers(&games), 2286);
    }
}

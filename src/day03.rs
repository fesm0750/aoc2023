//! Day 03: Gear Ratios
//!
//! Link to the challenge: https://adventofcode.com/2023/day/3
//!
//! # Problem:
//!
//! An input contains two-dimensional data composed of numbers and symbols, defined as follows:
//! - A `symbol` is any character that is neither a digit nor a '.';
//! - A `number` is a sequence of digits;
//! - A `part number` is a number adjacent to at least one symbol (above, below, sides, or diagonals);
//! - A `gear` is represented by a '*' symbol that is adjacent exactly to two part numbers and the `gear ratio` is the
//!   result of multiplying those numbers.
//!
//! From an input file containing lines of ascii text representing the aforementioned data:
//!
//! 1. Find the sum of all part numbers;
//!
//! 2. Find the sum of all "gear rations".
//!
//! # Solution
//!
//! - Expanded borders of the input in order to avoid dealing with overflow in boundary cases.
//!
//! - For part 1: Parse all numbers and stores their start and end positions, then check for any symbol around the
//!   extended rectangle defined by those positions.
//!
//! - For part 2: Checks the input data for '*' and them compare their positions to the part numbers, if there are
//!   exactly two adjacent part numbers, store the gear ratio.
//!
//! # Commentaries
//!
//! - Part 2 solution has an O^2 complexity, which could be improved.
use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day03").unwrap();
    let (extended_grid, _, n_cols) = expand_borders(&input, '.');
    let part_numbers = find_part_numbers(&extended_grid, n_cols);

    // Part 1
    println!("Part 01: Sum of part numbers: {}", sum_numbers(&part_numbers));

    // Part 2
    let gears = find_gears(&extended_grid, &part_numbers);
    println!("Part 02: Gear ratio sum: {}", sum_gear_ratios(&gears));
}

/// Takes an `input` string representing a grid of two-dimensional data and expands its borders with the `neutral`
/// character.
/// Returns a tuple containing the expanded grid, number of rows and number of columns.
fn expand_borders(input: &str, neutral: char) -> (String, usize, usize) {
    let n_cols = input.lines().next().unwrap().len(); // number of columns
    let n_rows = input.len() / (n_cols + 1); // number of rows; +1 accounts for '\n'

    // extends borders of the grid
    let mut grid = String::with_capacity((n_cols + 3) * (n_rows + 2));
    let border = std::iter::repeat(neutral).take(n_cols + 2).collect::<String>();
    grid.push_str(&border);
    grid.push('\n');
    grid.extend(input.lines().map(|l| format!(".{}.\n", l)));
    grid.push_str(&border);

    (grid, n_rows + 2, n_cols + 2)
}

/// Takes an expanded grid string and the number of columns to return a Vec containing the part numbers.
fn find_part_numbers(expanded_grid: &str, n_cols: usize) -> Vec<Number> {
    // Numbers array and auxiliary variables
    let mut numbers = Vec::<Number>::new();
    let mut number_acc = Vec::<u8>::new();
    let (mut start, mut end) = (0, 0);

    // runs over the data storing all possible part numbers
    for (row, line) in expanded_grid.lines().enumerate() {
        for (col, char) in line.as_bytes().iter().enumerate() {
            // push digits into accumulation buffer
            if char.is_ascii_digit() && col < n_cols {
                if number_acc.is_empty() {
                    start = col;
                }
                number_acc.push(*char);
                end = col;
            }

            // if numeric sequence ends, resolve number and save position
            if !number_acc.is_empty() && (!char.is_ascii_digit() || col == n_cols - 1) {
                // converts a sequence of characters into a number
                let n: u32 = (0..number_acc.len()).fold(0, |acc, i| acc * 10 + (number_acc[i] - b'0') as u32);

                // could have converted number_acc to string and done the parse
                // let n: u32 = std::str::from_utf8(&number_acc).unwrap().parse().unwrap();

                // saves number and its position
                numbers.push(Number {
                    val: n,
                    row,
                    start,
                    end,
                    is_part: false,
                });

                // clears auxiliary variables
                number_acc.clear();
                start = 0;
                end = 0;
            }
        }
    }

    // closure to determine if a character is considered a symbol
    let contains_symbol = |s: &str| -> bool { s.contains(|c: char| c != '.' && !c.is_ascii_digit()) };

    // checks if numbers are part numbers
    for n in &mut numbers {
        // Because the grid has been extended, there is no need to deal with boundary conditions.
        let start = n.start - 1;
        let end = n.end + 1;

        let idx_curr = n.row * (n_cols + 1);
        let idx_above = (n.row - 1) * (n_cols + 1);
        let idx_below = (n.row + 1) * (n_cols + 1);

        // a number is a part number if there is a symbol adjacent to it
        let is_part = contains_symbol(&expanded_grid[idx_above + start..idx_above + end + 1])     // line above
        || contains_symbol(&expanded_grid[idx_below + start..idx_below + end + 1])                  // line below
        || contains_symbol(&expanded_grid[idx_curr + start..idx_curr + start + 1])                  // character to the left
        || contains_symbol(&expanded_grid[idx_curr + end..idx_curr + end + 1]); // character to the right

        if is_part {
            n.is_part = true;
        }
    }

    // keep only part numbers
    numbers.retain(|n| n.is_part);

    numbers
}

/// Takes a grid and an array of part numbers to return a Vec of gears.
fn find_gears(grid: &str, part_numbers: &[Number]) -> Vec<Gear> {
    let mut gears = Vec::<Gear>::new();

    for (row, line) in grid.lines().enumerate() {
        for (col, &char) in line.as_bytes().iter().enumerate() {
            if char == b'*' {
                // checks adjacency
                let mut adjacency = Vec::<u32>::new();
                adjacency.extend(
                    part_numbers
                        .iter()
                        .filter(|n| n.is_adjacent(Position::new(row, col)))
                        .map(|n| n.val),
                );

                if let Some(gear) = Gear::new(&adjacency) {
                    gears.push(gear);
                }
            }
        }
    }

    gears
}

/// Returns the sum of all values stored in the `Number` struct
fn sum_numbers(part_numbers: &[Number]) -> u32 {
    part_numbers.iter().map(|n| n.val).sum()
}

/// Returns the sum of all gears ratios
fn sum_gear_ratios(gears: &[Gear]) -> u32 {
    gears.iter().map(|g| g.ratio).sum()
}

/// Struct to store two-dimensional grid positions.
struct Position {
    row: usize,
    col: usize,
}

/// Struct representing numbers and part numbers on the grid. It stores the value of the number, the row where it is
/// located, start and end positions within the row and indicates whether the number is a part number.
struct Number {
    val: u32,
    row: usize,
    start: usize,
    end: usize,
    is_part: bool,
}

/// Struct to represent gears and store the gear ratio.
struct Gear {
    ratio: u32,
}

impl Position {
    /// Creates a new `Position` from row and column indexes within grid.
    fn new(row: usize, col: usize) -> Position {
        Position { row, col }
    }
}

impl Gear {
    /// Creates a new `Gear` if it meets the adjacency parameters.
    fn new(adjacency: &[u32]) -> Option<Gear> {
        if adjacency.len() == 2 {
            return Some(Gear {
                ratio: adjacency.iter().product(),
            });
        }

        None
    }
}

impl Number {
    /// Checks if a given position is adjacent to a number on the grid.
    fn is_adjacent(&self, symbol: Position) -> bool {
        // avoids the use of subtraction, cuz it may cause overflow on edge cases
        self.row.abs_diff(symbol.row) <= 1  // same row, one above or one below
            && symbol.col <= (self.end + 1) // must be at most the position immediately after end
            && (symbol.col >= self.start || self.start.abs_diff(symbol.col) == 1) // must be at least the position
                                                                                  // immediately before start
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
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let (grid, _, n_cols) = expand_borders(input, '.');
        let part_numbers = find_part_numbers(&grid, n_cols);

        // Part 01
        assert_eq!(sum_numbers(&part_numbers), 4361);

        // Part 02
        let gears = find_gears(&grid, &part_numbers);
        assert_eq!(sum_gear_ratios(&gears), 467835);
    }
}

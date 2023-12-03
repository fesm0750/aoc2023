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
//! - A `gear` is represented by a '*' symbol that is adjacent to exactly to part numbers and the `gear ratio` is the
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
//! - For part 1: Parse all numbers and stores their start and end positions, check if there is any symbol around the
//!   expanded rectangle defined by those positions.
//! - For part 2: Checks the input data for '*' and them compare their positions to the part numbers, if there are
//!   exactly two adjacent part number, store the gear ratio.
//!
//! # Commentaries
//!
//! - Could have expanded the borders of the input with the neutral character '.', so as to not deal with boundary
//!   cases.
//!
//! - Part 2 solution is O^2 complexity, could improve it
use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day03").unwrap();
    let n_cols = input.lines().next().unwrap().len(); // number of columns
    let n_rows = input.len() / (n_cols + 1); // number of rows; +1 accounts for '\n'
    println!("{} {}", n_cols, n_rows);

    // Numbers array and auxiliary variables
    let mut numbers = Vec::<Number>::new();
    let mut number_acc = Vec::<u8>::new();
    let (mut start, mut end) = (0, 0);

    // runs over the data storing all possible part numbers
    for (row, line) in input.lines().enumerate() {
        for (col, char) in line.as_bytes().iter().enumerate() {
            // push digits into accumulation buffer
            if char.is_ascii_digit() && col < n_cols {
                if number_acc.len() == 0 {
                    start = col;
                }
                number_acc.push(*char);
                end = col;
            }

            // if numeric sequence ends, resolve number and save position
            if number_acc.len() > 0 && (!char.is_ascii_digit() || col == n_cols - 1) {
                // converts a sequence of characters into a number
                let n: u32 = (0..number_acc.len()).fold(0, |acc, i| acc * 10 + (number_acc[i] - b'0') as u32);

                // could have converted number_acc to string and done the parse
                // let n: u32 = std::str::from_utf8(&number_acc).unwrap().parse().unwrap();

                // saves number and its position
                numbers.push(Number {
                    val: n,
                    line: row,
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
    let is_symbol = |s: &str| -> bool { s.contains(|c: char| c != '.' && !c.is_ascii_digit()) };

    // checks if numbers are part numbers
    for n in &mut numbers {
        // determine indexes for the lines and characters position to check for symbols
        // if indexes are past boundaries, use positions of the number itself
        let start = if n.start > 0 { n.start - 1 } else { n.start };
        let end = if n.end < n_cols - 1 { n.end + 1 } else { n.end };

        let line_above = if n.line > 0 { n.line - 1 } else { n.line };
        let line_below = if n.line < n_rows - 1 { n.line + 1 } else { n.line };

        let idx_curr = n.line * (n_cols + 1);
        let idx_above = line_above * (n_cols + 1);
        let idx_below = line_below * (n_cols + 1);

        // a number is a part number if there is a symbol adjacent to it
        let is_part = is_symbol(&input[idx_above + start..idx_above + end + 1])     // line above
            || is_symbol(&input[idx_below + start..idx_below + end + 1])                  // line below
            || is_symbol(&input[idx_curr + start..idx_curr + start + 1])                  // character to the left
            || is_symbol(&input[idx_curr + end..idx_curr + end + 1]); // character to the right

        if is_part {
            n.is_part = true;
        }
    }

    // keep only part numbers
    numbers.retain(|n| n.is_part);

    let total: u32 = numbers.iter().filter(|n| n.is_part).map(|n| n.val).sum();

    println!("Part 01: Sum of part numbers: {}", total);

    //---
    // Part 2
    //---

    let mut gears = Vec::<Gear>::new();

    for (row, line) in input.lines().enumerate() {
        for (col, &char) in line.as_bytes().iter().enumerate() {
            if char == b'*' {
                // checks adjacency
                let mut adjacency = Vec::<u32>::new();
                adjacency.extend(
                    numbers
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

    // obtains the sum of gear ratios
    let sum: u32 = gears.iter().map(|g| g.ratio).sum();
    println!("Part 02: Gear ratio sum: {}", sum);
}

struct Position {
    row: u32,
    col: u32,
}

struct Number {
    val: u32,
    line: usize,
    start: usize,
    end: usize,
    is_part: bool,
}

struct Gear {
    ratio: u32,
}

impl Position {
    fn new(row: usize, col: usize) -> Position {
        Position {
            row: row as u32,
            col: col as u32,
        }
    }
}

impl Gear {
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
    fn is_adjacent(&self, symbol: Position) -> bool {
        let line_above = if self.line > 0 { self.line - 1 } else { self.line } as u32;
        let start_left = if self.start > 0 { self.start - 1 } else { self.start } as u32;
        // does not need special case for `end`, because that may not overflow

        symbol.row >= line_above
            && symbol.row <= self.line as u32 + 1
            && symbol.col >= start_left
            && symbol.col <= self.end as u32 + 1
    }
}

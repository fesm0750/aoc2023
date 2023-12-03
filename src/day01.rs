//! Day 01
//!
//! Link of the challenge: https://adventofcode.com/2023/day/1
//!
//! # Problem:
//!
//! From an input file containing lines of ascii text ("calibration data"):
//!
//! 1. Find the first and last digit of each line ("calibration digits"), combine them together to form a two digit
//!    number ("calibration value") and then sum them all;
//!
//! 2. Same task as before, but now digits can also be spelled out with letters.

use std::fs;
use std::str;

pub fn run() {
    let input = fs::read_to_string("inputs/day01").unwrap();

    let a = total_calibration_value(&input, calibration_digits_pt01);
    println!("Part 01: Total Calibration value: {}", a);

    let b = total_calibration_value(&input, calibration_digits_pt02);
    println!("Part 02: Total Calibration value: {}", b);
}

/// returns the total sum of calibration values
/// @param s: string containing the calibration data
/// @param calibration: function that reads the input data and returns the calibration digits
fn total_calibration_value(s: &str, calibration: fn(&str) -> (u32, u32)) -> u32 {
    s.lines().map(calibration).map(|(first, last)| first * 10 + last).sum()
}

/// returns the calibration digits from an input line according to part 1 rules
/// @param line: a string containing a single line of text (no '\n')
fn calibration_digits_pt01(line: &str) -> (u32, u32) {
    let mut digits = line.chars().filter_map(|c| c.to_digit(10));
    // lines have at least one digit,
    let first = digits.next().unwrap();
    // if only one, then repeat
    let last: u32 = digits.next_back().unwrap_or(first);
    (first, last)
}

/// returns the calibration digits from an input line according to part 2 rules
/// @param line: a string containing a single line of text (no '\n')
fn calibration_digits_pt02<'a>(line: &str) -> (u32, u32) {
    // closure to parse digits and spelled values into numbers
    let digit_filter = |s: &str| -> Option<u32> {
        match s {
            _ if s.as_bytes()[0].is_ascii_digit() => Some((s.as_bytes()[0] - b'0') as u32),
            _ if s.starts_with("one") => Some(1),
            _ if s.starts_with("two") => Some(2),
            _ if s.starts_with("three") => Some(3),
            _ if s.starts_with("four") => Some(4),
            _ if s.starts_with("five") => Some(5),
            _ if s.starts_with("six") => Some(6),
            _ if s.starts_with("seven") => Some(7),
            _ if s.starts_with("eight") => Some(8),
            _ if s.starts_with("nine") => Some(9),
            _ => None,
        }
    };

    let len = line.len();
    // search from left
    let first = (0..len).find_map(|i| digit_filter(&line[i..])).unwrap();
    // search from right
    let last = (1..len + 1).find_map(|i| digit_filter(&line[len - i..])).unwrap();

    (first, last)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt01_tests() {
        assert_eq!(total_calibration_value("1abc2", calibration_digits_pt01), 12);
        assert_eq!(total_calibration_value("pqr3stu8vwx", calibration_digits_pt01), 38);
        assert_eq!(total_calibration_value("a1b2c3d4e5f", calibration_digits_pt01), 15);
        assert_eq!(total_calibration_value("treb7uchet", calibration_digits_pt01), 77);
    }

    #[test]
    fn pt02_tests() {
        assert_eq!(total_calibration_value("two1nine", calibration_digits_pt02), 29);
        assert_eq!(total_calibration_value("eightwothree", calibration_digits_pt02), 83);
        assert_eq!(total_calibration_value("abcone2threexyz", calibration_digits_pt02), 13);
        assert_eq!(total_calibration_value("xtwone3four", calibration_digits_pt02), 24);
        assert_eq!(total_calibration_value("4nineeightseven2", calibration_digits_pt02), 42);
        assert_eq!(total_calibration_value("zoneight234", calibration_digits_pt02), 14);
        assert_eq!(total_calibration_value("7pqrstsixteen", calibration_digits_pt02), 76);
        assert_eq!(total_calibration_value("oneight", calibration_digits_pt02), 18);
    }
}

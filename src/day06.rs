//! Day 06: Wait For It
//!
//! Link: https://adventofcode.com/2023/day/6
//!
//! # Problem
//!
//! A racing game where the time "accelerating" determines the speed of the boat, the winner is the one that goes
//! farthest inside the time limit. The time limit comprises the time accelerating and the remaining racing time.
//!
//! From an input file containing the time limit for the race and the current distance record:
//!
//! 1. Find the number of ways each of the races can have the record beaten and them multiply the values;
//!
//! 2. The spacing in the input file was wrong, find the new data and determine the number of ways the record can be
//!    beaten;
//!
//! # Solution
//!
//! Solves a closed form equation using the quadratic formula. The count of ways to beat the record corresponds to the
//! number of integer values within the range given by the solutions of the equation.
//!
//! The time complexity of the closed form solution is theoretically O(n), but it uses a square root function that may
//! or may not have have a grater complexity depending on how it is implemented in hardware.
//!
//! # Possible alternative solutions
//!
//! - Brute Force: the problem is symmetric, so the range can be found by counting from 0 to the first success case;
//!
//! - Binary search: the success cases are in the middle of the range.
//!
//! - Newton's method: can be employed to use only integer values.
use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day06").unwrap();
    let races = parse_input(&input);

    // part 1
    let beat: u64 = races.iter().map(|&r| count_record_beating_ways(r)).product();
    println!("Part 1: Product of the number of ways to beat the record: {}", beat);

    // part 2, remove whitespace from input
    let mut input_pt2 = input.clone();
    input_pt2.retain(|c: char| c != ' ');
    let race = parse_input(&input_pt2);
    let beat_pt2 = count_record_beating_ways(race[0]);

    println!("Part 2: Number of ways to beat the record: {}", beat_pt2);
}

/// Parses an input string into a `Vec` of `Race`s.
///
/// Each `Race` object comprises a pair of time and distance. The input must adhere to the structure shown below, where
/// each line consists of a prefix followed by a list o numbers separated by whitespace. The prefixes must appear in the
/// specified order as shown:
///
/// "Time:      7  15   30"
/// "Distance:  9  40  200"
fn parse_input(s: &str) -> Vec<Race> {
    let mut lines = s.lines();

    let mut get_next_line = |prefix: &str| {
        lines
            .next()
            .unwrap()
            .strip_prefix(prefix)
            .unwrap()
            .split_ascii_whitespace()
            .flat_map(str::parse)
    };

    let time = get_next_line("Time:");
    let distance = get_next_line("Distance:");

    time.zip(distance)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

/// Returns the count of ways to beat the race record
///
/// # Calculation
///
/// This function determines the number of ways to beat the race record by employing a closed-form solution based on
/// the equation of motion. The process involves using the quadratic formula to determine the time boundaries and
/// subsequently computing the count of ways to exceed the record time.
///
/// Given:
///
/// - Minimum distance 'D' and total time 'T' from the input.
///
/// Equations:
///
/// d = v * t (Equation of Motion: distance equals velocity times time)
///
/// T = t_a + t_r (total time is equals to acceleration time plus racing time)
///
/// V = t_a  (speed V equals accelerating time)
///
/// Deriving Equation of Motion in terms of 't_a':
///
/// d = v * t
/// D = V * t_r
/// D = t_a * (T - t_a)
///
/// This leads to the following quadratic equation:
///
/// t_a^2 - T*t_a + D = 0
///
/// The solutions of the quadratic equation delineate limits where t_a equals the distance D. To ensure distances
/// surpass D, we must stay within the following boundaries:
///
/// t1 > T/2 - sqrt(delta)/2
/// t2 < T/2 + sqrt(delta)/2
///
/// where delta = T^2 - 4*D
///
/// Due the interval being open, rounding the values of t1 and t2 is necessary to place them in the correct range.
/// Consequently, there is a corner case where the solutions are exact, making it impossible to fit the values within
/// the limits by simple rounding. To address this without resorting to conditionals, a workaround involves adding or
/// subtracting 1 and then using a rounding method inverse to that originally needed (for example, ceil instead of
/// floor).
fn count_record_beating_ways(r: Race) -> u64 {
    let delta_sqrt = ((r.time * r.time - 4 * r.distance) as f64).sqrt();

    let t1 = ((r.time as f64 + delta_sqrt) / 2.0 - 1.0).ceil() as u64; // always "rounds" down, even if delta is exact
    let t2 = ((r.time as f64 - delta_sqrt) / 2.0 + 1.0).floor() as u64; // always "rounds" up, even if delta is exact

    //+1 because range inclusive
    t1 - t2 + 1
}

//----------
// Structs
//----------

#[derive(Clone, Copy)]
struct Race {
    time: u64,
    distance: u64,
}

//----------
// Test
//----------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let races = parse_input(input);
        assert_eq!(count_record_beating_ways(races[0]), 4);
        assert_eq!(count_record_beating_ways(races[1]), 8);
        assert_eq!(count_record_beating_ways(races[2]), 9);

        assert_eq!(count_record_beating_alternative(races[0]), 4);
        assert_eq!(count_record_beating_alternative(races[1]), 8);
        assert_eq!(count_record_beating_alternative(races[2]), 9);
    }
}

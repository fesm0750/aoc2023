//! Day 08: Haunted Wasteland
//!
//! Link: https://adventofcode.com/2023/day/6
//!
//! For part 2: Assumes the values starting cycling if a Z position is reached

use num::integer::lcm;
use std::{collections::HashMap, fs};

type Nodes<'a> = HashMap<&'a [u8], (&'a [u8], &'a [u8])>;

pub fn run() {
    let input = fs::read_to_string("inputs/day08").unwrap();
    let (directions, nodes, starts) = parse_input(&input);

    let count = solve_pt1(directions, &nodes);
    println!("Part 1: Total steps: {count}");

    let count = solve_pt2(directions, &nodes, starts);
    println!("Part 2: Total steps: {count}");
}

fn parse_input(input: &str) -> (&str, Nodes, Vec<&[u8]>) {
    let mut lines = input.lines();

    let directions = lines.next().unwrap();
    let mut nodes = Nodes::new();
    let mut starts = Vec::<&[u8]>::new();

    // AAA = (BBB, CCC)
    for line in lines.skip(1) {
        let line = line.as_bytes();
        let node = &line[0..3];
        let dir_l = &line[7..10];
        let dir_r = &line[12..15];
        nodes.insert(node, (dir_l, dir_r));

        if node[2..3] == [b'A'] {
            starts.push(node);
        }
    }

    (directions, nodes, starts)
}

fn solve_pt1(directions: &str, nodes: &Nodes) -> u64 {
    const STARTING_NODE: [u8; 3] = [b'A', b'A', b'A'];
    const ENDING_NODE: [u8; 3] = [b'Z', b'Z', b'Z'];
    solve(directions, nodes, &STARTING_NODE, |node| node == ENDING_NODE)
}

fn solve_pt2(directions: &str, nodes: &Nodes, starts: Vec<&[u8]>) -> u64 {
    let end = |node: &[u8]| node[2..3] == [b'Z'];
    let values: Vec<u64> = starts
        .iter()
        .map(|start| solve(directions, nodes, start, end))
        .collect::<Vec<u64>>();
    lcm_of_vector(&values)
}

fn solve(directions: &str, nodes: &Nodes, start: &[u8], end: fn(&[u8]) -> bool) -> u64 {
    let mut node = start;
    let mut count = 0;

    for &dir in directions.as_bytes().iter().cycle() {
        count += 1;
        let (l, r) = nodes.get(node).unwrap();
        if dir == b'L' {
            node = l;
        } else {
            node = r;
        }

        if end(node) {
            break;
        }
    }

    count
}

//----------
// helper Methods
//----------

fn lcm_of_vector(values: &[u64]) -> u64 {
    let mut result = values[0];
    for &value in values.iter().skip(1) {
        result = lcm(result, value);
    }

    result
}

//----------
// Test
//----------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pt1_test0() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

        let (directions, nodes, _) = parse_input(input);
        assert_eq!(solve_pt1(directions, &nodes), 6);
    }

    #[test]
    fn pt1_test1() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        let (directions, nodes, _) = parse_input(input);
        assert_eq!(solve_pt1(directions, &nodes), 2);
    }

    #[test]
    fn pt2_test() {
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        let (directions, nodes, starts) = parse_input(input);
        assert_eq!(solve_pt2(directions, &nodes, starts), 6);
}
}

//! Day 05: If You Give A Seed A Fertilizer
//!
//! Link to [Day 05 Problem](https://adventofcode.com/2023/day/5)
//!
//! # Problem
//!
//! We need to locate the closest spot for planting a seed. In order to do this, an almanac maps the ideal relationships
//! like seeds to soil, soils to fertilizes, and so on.
//!
//! From an input file containing the codes of the seeds and the several maps:
//!
//! 1. Find the lowest location number that corresponds to any of the initial seed codes;
//!
//! 2. Find again the lowest location, but now the data in the input file correspond to seed ranges.
//!
//! # Solution
//!
//! - Utilizes brute force by applying the mapping transformation to all seeds.
//!
//! - For part 2, uses Rayon to paralelize the iterator.
//!
//! The single-threaded version for part 2 took around 75 seconds, whereas the parallel solution ran in 9.5 seconds,
//! handling over 3,5 billion seeds. These timing are amazing and if compared to what people were reporting for brute
//! force approaches on the Advent of Code subreddit.
//!
//! # Possible Alternative Solutions
//!
//! - Reverse search brute force: Begins from the lowest possible position and walks up the maps, iterates until finding
//!   an initial seed. The number of positions to search should be much lower than the total amount of seeds for part 2,
//!   due to the location being in the order of millions instead of billions.
//!
//! - Bucket splitting: Operates with buckets of seeds instead of individual seeds. When a bucket is larger than a
//!   range, split the bucket. Working with buckets should vastly reduce the input size.
//!
//! - Range splitting: Preprocesses the maps by segmenting the ranges into smaller ones until establishing a direct map
//!   between seed and location.
use rayon::prelude::*;
use std::{cmp::Ordering, error, fs, str::FromStr, time::Instant};

type Seeds = Vec<u64>;
type AMap = Vec<Entry>;
type Almanac = Vec<AMap>;

pub fn run() {
    let input = fs::read_to_string("inputs/day05").unwrap();
    let (seeds, almanac) = parse_input(&input);

    // part 1
    let location = process_lowest_location(&seeds, &almanac);
    println!("Part 1: Lowest Location number: {}", location);

    // part 2
    let now = Instant::now();
    let location_pt2 = process_lowest_location_pt2_mt(&seeds, &almanac);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Part 2: Lowest Location number: {}", location_pt2);
}

/// Parse an input string into `Seeds` and `Almanac`
///
/// Maps are stored in the order of occurrence, while the entries within the maps are sorted.
fn parse_input(s: &str) -> (Seeds, Almanac) {
    let mut iter = s.split("\n\n");

    let seeds = iter
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .flat_map(str::parse)
        .collect::<Seeds>();

    let almanac = iter
        .map(|s| s.lines().skip(1).flat_map(str::parse).collect::<AMap>())
        .map(|mut vec| {
            vec.sort_unstable();
            vec
        })
        .collect::<Almanac>();

    (seeds, almanac)
}

/// Returns the lowest location from the `Seeds` and `Almanac` inputs.
fn process_lowest_location(seeds: &Seeds, almanac: &Almanac) -> u64 {
    // position is given by the last map
    let mut location = u64::MAX;
    let mut val;
    for &seed in seeds {
        val = seed;
        for map in almanac {
            let idx = map.binary_search_by(|e| e.cmp_to(val));

            val = if let Ok(idx) = idx {
                let diff = val - map[idx].start;
                map[idx].destination_start + diff
            } else {
                val
            };
        }
        location = location.min(val);
    }

    location
}

/// Returns the lowest location from the `Seeds` and `Almanac` inputs. Internally, seeds will be processed using part
/// 2 rules and muti-threading.
///
/// It is the same algorithm as part 1, but rewritten to use only iterators, allowing parallel execution with Rayon.
fn process_lowest_location_pt2_mt(seeds: &Seeds, almanac: &Almanac) -> u64 {
    let seeds = seeds.par_chunks(2).flat_map(|a| (a[0]..a[0] + a[1])); // .take(a[1] as usize)

    seeds
        .map(|seed| {
            almanac
                .iter()
                .scan(seed, |val, map| {
                    let v = *val;
                    let idx = map.binary_search_by(|e| {
                        if e.start > v {
                            Ordering::Greater
                        } else if e.start <= v && v <= e.end {
                            Ordering::Equal
                        } else {
                            Ordering::Less
                        }
                    });

                    *val = if let Ok(idx) = idx {
                        let diff = v - map[idx].start;
                        map[idx].destination_start + diff
                    } else {
                        v
                    };

                    Some(*val)
                })
                .last()
                .unwrap()
        })
        .min()
        .unwrap()
}

//----------
// Structs
//----------

#[derive(Clone, Copy, PartialEq, Eq)]
struct Entry {
    start: u64,
    end: u64,
    destination_start: u64,
}

impl Entry {
    fn cmp_to(&self, val: u64) -> Ordering {
        if self.start > val {
            Ordering::Greater
        } else if self.start <= val && val <= self.end {
            Ordering::Equal
        } else {
            Ordering::Less
        }
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Entry {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_ascii_whitespace().flat_map(str::parse);
        let destination_start = iter.next().ok_or("Not able to parse the destination start.")?;
        let start = iter.next().ok_or("Not able to parse the start of the range.")?;
        let end = start + iter.next().ok_or("Not able to parse the end of the range.")? - 1;

        Ok(Entry {
            start,
            end,
            destination_start,
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
        let input = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

        let (seeds, almanac) = parse_input(input);

        // part 1
        let location = process_lowest_location(&seeds, &almanac);
        assert_eq!(location, 35);

        // part 2
        let location2mt = process_lowest_location_pt2_mt(&seeds, &almanac);
        assert_eq!(location2mt, 46);
    }
}

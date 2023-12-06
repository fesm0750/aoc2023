use std::{cmp::Ordering, error, fs, str::FromStr, time::Instant};

type AlmanacMap = Vec<Entry>;
type Almanac = Vec<AlmanacMap>;

pub fn run() {
    let input = fs::read_to_string("inputs/day05").unwrap();

    // let find_destination = ||

    let (seeds, almanac) = parse_input(&input);

    // part 1
    let location = process_lowest_location(&seeds, &almanac);
    println!("Part 1: Lowest Location number: {}", location);

    // part 2
    let now = Instant::now();
    let location_pt2 = process_lowest_location_pt2(&seeds, &almanac);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Part 2: Lowest Location number: {}", location_pt2);
}

fn parse_input(s: &str) -> (Vec<u64>, Almanac) {
    let mut iter = s.lines();

    let seeds = iter
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split_ascii_whitespace()
        .flat_map(str::parse)
        .collect::<Vec<u64>>();

    // parses the maps as Almanac, they are stored in order in the vec
    let mut almanac: Vec<Vec<Entry>> = Vec::new();
    let mut almanac_map: Vec<Entry> = Vec::new();
    for line in iter {
        if line.is_empty() || line.starts_with(|c: char| c.is_ascii_alphabetic()) {
            if !almanac_map.is_empty() {
                almanac_map.sort_unstable();
                almanac.push(almanac_map.to_vec());
                almanac_map.clear();
            }
            continue;
        }
        almanac_map.push(line.parse::<Entry>().unwrap());
    }

    almanac_map.sort_unstable();
    almanac.push(almanac_map.to_vec());
    (seeds, almanac)
}

fn process_lowest_location(seeds: &[u64], almanac: &Almanac) -> u64 {
    // position is given by the last map
    let mut location = u64::MAX;
    let mut val;
    for &seed in seeds {
        val = seed;
        for map in almanac {
            let idx = map.binary_search_by(|e| {
                if e.start > val {
                    Ordering::Greater
                } else if e.start <= val && val <= e.end {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            });

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

fn process_lowest_location_pt2(seeds: &[u64], almanac: &Almanac) -> u64 {
    let seeds = seeds.chunks(2).flat_map(|a| (a[0]..).take(a[1] as usize));
    // position is given by the last map
    let mut location = u64::MAX;
    let mut val;
    for seed in seeds {
        val = seed;
        for map in almanac {
            let idx = map.binary_search_by(|e| {
                if e.start > val {
                    Ordering::Greater
                } else if e.start <= val && val <= e.end {
                    Ordering::Equal
                } else {
                    Ordering::Less
                }
            });

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

#[derive(Clone, Copy, PartialEq, Eq)]
struct Entry {
    start: u64,
    end: u64,
    destination_start: u64,
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.start.cmp(&other.start))
    }
}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start)
    }
}

impl FromStr for Entry {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split_ascii_whitespace().flat_map(str::parse);
        let destination_start = iter.next().unwrap();
        let start = iter.next().unwrap();
        let end = start + iter.next().unwrap() - 1;

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

    // #[test]
    // fn test_parse_scratchcard() {
    //     let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
    //     assert_eq!(input.parse::<Scratchcard>().unwrap(), Scratchcard { id: 1, matches: 4 });
    // }

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
        let location2 = process_lowest_location_pt2(&seeds, &almanac);
        assert_eq!(location2, 46);
    }
}

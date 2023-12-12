use std::fs;

pub fn run() {
    let input = fs::read_to_string("inputs/day09").unwrap();
    let history_data = parse_input(&input);

    // part 1
    let back: i64 = sum_extrapolated(&history_data, extrapolate_back_rec);
    println!("Part 1: Sum of extrapolated back values: {back}");

    // part 2
    let front: i64 = sum_extrapolated(&history_data, extrapolate_front_rec);
    println!("Part 2: Sum of extrapolated front values: {front}");
}

fn parse_input(s: &str) -> Vec<Vec<i64>> {
    let mut ret = Vec::new();
    for line in s.lines() {
        ret.push(line.split_ascii_whitespace().flat_map(str::parse).collect());
    }
    ret
}

fn sum_extrapolated(history_data: &[Vec<i64>], recursion: fn(&[i64]) -> i64) -> i64 {
    history_data.iter().map(|history| recursion(history)).sum()
}

fn reduce(data: &[i64]) -> Vec<i64> {
    data.iter().map_windows(|[a, b]| *b - *a).collect::<Vec<_>>()
}

fn extrapolate_back_rec(data: &[i64]) -> i64 {
    let step = reduce(data);

    data.last().unwrap()
        + if step.iter().any(|&n| n != 0) {
            extrapolate_back_rec(&step)
        } else {
            0
        }
}

fn extrapolate_front_rec(data: &[i64]) -> i64 {
    let step = reduce(data);

    data[0]
        - if step.iter().any(|&n| n != 0) {
            extrapolate_front_rec(&step)
        } else {
            0
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests() {
        let input = "10  13  16  21  30  45";
        let input = parse_input(input);
        assert_eq!(extrapolate_back_rec(&input[0]), 68);
        assert_eq!(extrapolate_front_rec(&input[0]), 5);
    }
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn solution_part_1(input: &Path) -> u32 {
    let mut first = vec![];
    let mut second = vec![];

    let reader = File::open(input)
        .map(BufReader::new)
        .expect("File should exist.");

    for line in reader.lines() {
        let line = line.unwrap();
        let mut args = line.split_whitespace();

        if let Some(f) = args.next() {
            let fnum: u32 = f.parse().expect("value {f} should be valid number.");
            first.push(fnum);
        };

        if let Some(s) = args.next() {
            let snum: u32 = s.parse().expect("value {s} should be valid number.");
            second.push(snum);
        }
    }

    let is_same_len = first.len() == second.len();
    if !is_same_len {
        panic!("data should match in length.");
    }

    first.sort();
    second.sort();

    let mut distance = 0;

    for (idx, item) in first.iter().enumerate() {
        let delta = item.abs_diff(second[idx]);
        distance += delta;
    }

    println!("distance is {distance}");
    distance
}

pub fn solution_part_2(input: &Path) -> u32 {
    let mut first = vec![];
    let mut second_dupes_map: HashMap<u32, u32> = HashMap::new();

    let reader = File::open(input)
        .map(BufReader::new)
        .expect("File should exist.");

    for line in reader.lines() {
        let line = line.unwrap();
        let mut args = line.split_whitespace();

        if let Some(f) = args.next() {
            let fnum: u32 = f.parse().expect("value {f} should be valid number.");
            first.push(fnum);
        };

        if let Some(s) = args.next() {
            let snum: u32 = s.parse().expect("value {s} should be valid number.");
            second_dupes_map
                .entry(snum)
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        }
    }

    let mut score = 0;

    for num in first.iter() {
        score += num * second_dupes_map.get(num).unwrap_or(&0);
    }

    println!("similarity score is {score}");
    score
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn total_distance() {
        let path = Path::new("src/day_01/aoc_test_input.txt");
        assert_eq!(solution_part_1(&path), 11);
    }

    #[test]
    fn similarity_score() {
        let path = Path::new("src/day_01/aoc_test_input.txt");
        assert_eq!(solution_part_2(&path), 31);
    }
}

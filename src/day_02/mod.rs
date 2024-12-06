use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    ASC,
    DESC,
    NONE,
}

pub fn solution_part_1(file: &Path) -> u32 {
    let reader = File::open(file)
        .map(BufReader::new)
        .expect("File should exist.");

    let mut safe_count = 0;

    'line: for line in reader.lines() {
        let line = line.unwrap();
        let args: Vec<u32> = line
            .split_whitespace()
            .map(|v| v.parse().expect("value should be a number."))
            .collect();

        let mut prev_direction: Direction = Direction::NONE;

        for i in 1..args.len() {
            let prev: u32 = args[i - 1];
            let curr: u32 = args[i];

            let direction = if prev > curr {
                Direction::DESC
            } else {
                Direction::ASC
            };

            if direction != prev_direction && prev_direction != Direction::NONE {
                continue 'line;
            }

            prev_direction = direction;

            let diff = prev.abs_diff(curr);

            if diff < 1 || diff > 3 {
                continue 'line;
            }
        }

        safe_count += 1;
    }

    safe_count
}

pub fn solution_part_2(file: &Path) -> u32 {
    let reader = File::open(file)
        .map(BufReader::new)
        .expect("File should exist.");

    let mut safe_count = 0;

    'line: for line in reader.lines() {
        let line = line.unwrap();
        let args: Vec<u32> = line
            .split_whitespace()
            .map(|v| v.parse().expect("value should be a number."))
            .collect();

        'variant: for i in 0..args.len() {
            let mut new_vec = args.clone();
            new_vec.remove(i);

            let mut prev_direction: Direction = Direction::NONE;

            for i in 1..new_vec.len() {
                let prev: u32 = new_vec[i - 1];
                let curr: u32 = new_vec[i];

                let direction = if prev > curr {
                    Direction::DESC
                } else {
                    Direction::ASC
                };

                if direction != prev_direction && prev_direction != Direction::NONE {
                    continue 'variant;
                };

                prev_direction = direction;

                let diff = prev.abs_diff(curr);

                if diff < 1 || diff > 3 {
                    continue 'variant;
                }
            }

            safe_count += 1;
            continue 'line
        }
    }

    safe_count
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn safe_count() {
        let path = Path::new("src/day_02/test_input.txt");
        let solution = solution_part_1(path);
        assert_eq!(solution, 2);
    }

    #[test]
    fn tolerant_safe_count() {
        let path = Path::new("src/day_02/test_input.txt");
        let solution = solution_part_2(path);
        assert_eq!(solution, 4);
    }
}

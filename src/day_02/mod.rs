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
            let mut _direction: Direction = Direction::NONE;

            if prev > curr {
                _direction = Direction::DESC;
            } else {
                _direction = Direction::ASC;
            }

            if _direction != prev_direction && i > 1 {
                continue 'line;
            }

            prev_direction = _direction;

            let diff = prev.abs_diff(curr);

            if diff < 1 || diff > 3 {
                continue 'line;
            }
        }

        safe_count += 1;
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
}

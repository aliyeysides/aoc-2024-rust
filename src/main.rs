use std::{fs::File, io::Read, path::Path};

use aoc_2024::day_03::solution_part_1;

fn main() {
    let path = Path::new("src/day_03/input.txt");

    let mut file = File::open(path).expect("File should exist.");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read file.");

    let ans = solution_part_1(&contents);
    println!("{}", ans);
}

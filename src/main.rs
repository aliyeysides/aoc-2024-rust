use std::path::Path;

use aoc_2024::day_02::solution_part_1;

fn main() {
    let path = Path::new("src/day_02/input.txt");
    let ans = solution_part_1(path);
    println!("{}", ans);
}

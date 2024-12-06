use regex::Regex;

pub fn solution_part_1(mem: &str) -> u32 {
    let re = Regex::new(r"(mul\(\d+,\d+\))").unwrap();
    let mut total = 0;

    for (_, [m]) in re.captures_iter(mem).map(|c| c.extract()) {
        let numre = Regex::new(r"(\d+),(\d+)").unwrap();

        for (_, [d1, d2]) in numre.captures_iter(m).map(|c| c.extract()) {
            let num1: u32 = d1.parse().expect("match should be a u32");
            let num2: u32 = d2.parse().expect("match should be a u32");

            total += num1 * num2;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_memory() {
        let mem = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let res = solution_part_1(mem);
        assert_eq!(res, 161);
    }
}

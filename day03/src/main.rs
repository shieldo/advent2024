use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    println!("Answer to part 1: {}", part1(input));
    println!("Answer to part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(input)
        .map(|c| c[1].parse::<usize>().unwrap() * c[2].parse::<usize>().unwrap())
        .sum()
}

fn part2(input: &str) -> usize {
    Regex::new(r"(?ms)don't\(\).*?do\(\)")
        .unwrap()
        .split(input)
        .map(part1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn test_input() -> &'static str {
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    }

    #[rstest]
    fn test_part1(test_input: &str) {
        assert_eq!(part1(test_input), 161);
    }

    #[fixture]
    fn test_input2() -> &'static str {
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    }

    #[rstest]
    fn test_part2(test_input2: &str) {
        assert_eq!(part2(test_input2), 48);
    }
}

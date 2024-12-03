use regex::Regex;
use std::sync::LazyLock;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    Some(part1(input) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(part2(input) as u32)
}

static MUL_FUNCTION: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"mul\((\d+),(\d+)\)").unwrap());
static DISABLED_BLOCK: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?ms)don't\(\).*?do\(\)").unwrap());

fn part1(input: &str) -> usize {
    MUL_FUNCTION
        .captures_iter(input)
        .map(|c| c[1].parse::<usize>().unwrap() * c[2].parse::<usize>().unwrap())
        .sum()
}

fn part2(input: &str) -> usize {
    DISABLED_BLOCK.split(input).map(part1).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}

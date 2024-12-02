fn main() {
    let input = include_str!("./input.txt");
    println!("Answer to part 1 is: {}", find_safe_report_count(input));
}

fn find_safe_report_count(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let report = line.split(" ").map(|n| n.parse::<isize>().unwrap());
            (report.clone().is_sorted() || report.clone().is_sorted_by(|a, b| a > b))
                && report
                    .collect::<Vec<_>>()
                    .windows(2)
                    .all(|nums| (1..=3).contains(&(nums[0] - nums[1]).abs()))
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn test_input() -> &'static str {
        "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"
    }

    #[rstest]
    fn test_sample_report_count(test_input: &str) {
        assert_eq!(find_safe_report_count(test_input), 2);
    }
}

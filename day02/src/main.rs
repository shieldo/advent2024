use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    println!("Answer to part 1 is: {}", find_safe_report_count(input));
    println!(
        "Answer to part 2 is: {}",
        find_dampened_safe_report_count(input)
    );
}

fn find_safe_report_count(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let report = line.split(" ").map(|n| n.parse::<isize>().unwrap());
            report_is_good(&report.collect_vec())
        })
        .count()
}

fn find_dampened_safe_report_count(input: &str) -> usize {
    input
        .lines()
        .filter(|line| {
            let report = line.split(" ").map(|n| n.parse::<isize>().unwrap());
            if report_is_good(&report.clone().collect_vec()) {
                return true;
            }
            let report_count = line.trim().chars().filter(|c| c == &' ').count() + 1;
            report
                .cycle()
                .take(report_count.pow(2))
                .enumerate()
                .filter_map(|(i, n)| (i % (report_count + 1) != 0).then(|| n))
                .chunks(report_count - 1)
                .into_iter()
                .any(|num| report_is_good(&num.collect_vec()))
        })
        .count()
}

fn report_is_good(report: &[isize]) -> bool {
    (report.iter().is_sorted() || report.iter().is_sorted_by(|a, b| a > b))
        && report
            .windows(2)
            .all(|nums| (1..=3).contains(&(nums[0] - nums[1]).abs()))
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

    #[rstest]
    fn test_dampened_safe_report_count(test_input: &str) {
        assert_eq!(find_dampened_safe_report_count(test_input), 4);
    }
}

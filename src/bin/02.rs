use itertools::Itertools;

advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(find_safe_report_count(input) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(find_dampened_safe_report_count(input) as u32)
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
                .filter_map(|(i, n)| (i % (report_count + 1) != 0).then_some(n))
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

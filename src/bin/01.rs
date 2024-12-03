advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(combine_lists(input) as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(sum_counts(input) as u32)
}

fn combine_lists(input: &str) -> usize {
    let (list1, list2) = parsed_lists(input);

    list1
        .iter()
        .zip(list2.iter())
        .map(|(&a, &b)| (a as isize - b as isize).unsigned_abs())
        .sum()
}

fn sum_counts(input: &str) -> usize {
    let (list1, list2) = parsed_lists(input);

    list1
        .iter()
        .map(|num| *num * list2.iter().filter(|&item| num == item).count())
        .sum()
}

fn parsed_lists(input: &str) -> (Vec<usize>, Vec<usize>) {
    let numbers = input.lines().map(|line| {
        line.split_ascii_whitespace()
            .map(|digits| digits.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    });
    let mut list1 = vec![];
    let mut list2 = vec![];
    for pair in numbers {
        list1.push(pair[0]);
        list2.push(pair[1]);
    }
    list1.sort();
    list2.sort();
    (list1, list2)
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

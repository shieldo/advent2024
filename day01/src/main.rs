fn main() {
    let input = include_str!("input.txt");
    println!("Answer to part 1: {}", combine_lists(input));
    println!("Answer to part 2: {}", sum_counts(input));
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
    use rstest::*;

    #[fixture]
    fn test_input() -> &'static str {
        "3   4
4   3
2   5
1   3
3   9
3   3"
    }

    #[rstest]
    fn test_combine_lists(test_input: &str) {
        assert_eq!(combine_lists(test_input), 11);
    }

    #[rstest]
    fn test_sum_counts(test_input: &str) {
        assert_eq!(sum_counts(test_input), 31);
    }
}

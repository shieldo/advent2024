fn main() {
    let input = include_str!("input.txt");
    println!("Answer to part 1: {}", combine_lists(input));
}

fn combine_lists(input: &str) -> usize {
    let numbers = input.lines().map(|line| line.split_ascii_whitespace().map(|digits| digits.parse::<usize>().unwrap()).collect::<Vec<_>>());
    let mut list1 = vec![];
    let mut list2 = vec![];
    for pair in numbers {
        list1.push(pair[0]);
        list2.push(pair[1]);
    }
    list1.sort();
    list2.sort();
    list1.iter().zip(list2.iter()).map(|(&a, &b)| (a as isize - b as isize).abs() as usize).sum()
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
}

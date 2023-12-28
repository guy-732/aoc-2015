use itertools::Itertools;

#[inline]
fn expand_look_and_say(source: Vec<u8>) -> Vec<u8> {
    let mut result = vec![];
    let mut count = 0;
    let mut last_digit = 0; // count is 0 anyways
    for digit in source {
        if last_digit == digit {
            count += 1;
            continue;
        }

        if count > 0 {
            result.extend_from_slice(&[count, last_digit]);
        }

        last_digit = digit;
        count = 1;
    }

    if count > 0 {
        result.extend_from_slice(&[count, last_digit]);
    }

    result
}

#[aoc(day10, part1)]
fn part1(input: &str) -> usize {
    let mut input = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect_vec();
    for _ in 0..40 {
        input = expand_look_and_say(input);
    }

    input.len()
}

#[aoc(day10, part2)]
fn part2(input: &str) -> usize {
    let mut input = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as u8)
        .collect_vec();
    for _ in 0..50 {
        input = expand_look_and_say(input);
    }

    input.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_expand_tests() {
        assert_eq!(expand_look_and_say(vec![1]), vec![1, 1]);
        assert_eq!(expand_look_and_say(vec![1, 1]), vec![2, 1]);
        assert_eq!(expand_look_and_say(vec![2, 1]), vec![1, 2, 1, 1]);
        assert_eq!(
            expand_look_and_say(vec![1, 2, 1, 1]),
            vec![1, 1, 1, 2, 2, 1]
        );
        assert_eq!(
            expand_look_and_say(vec![1, 1, 1, 2, 2, 1]),
            vec![3, 1, 2, 2, 1, 1]
        );
    }
}

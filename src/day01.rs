use aoc_runner_derive::aoc;

#[aoc(day1, part1)]
fn part1(input: &[u8]) -> i64 {
    let mut count = 0;
    for &c in input {
        if c == b'(' {
            count += 1;
        } else if c == b')' {
            count -= 1;
        }
    }

    count
}

#[aoc(day1, part2)]
fn part2(input: &[u8]) -> i64 {
    let mut count = 0;
    for (i, &c) in input.iter().enumerate() {
        if c == b'(' {
            count += 1;
        } else if c == b')' {
            count -= 1;
        }

        if count == -1 {
            return (i as i64) + 1;
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLES: [(&[u8], i64); 9] = [
        (b"(())", 0),
        (b"()()", 0),
        (b"(((", 3),
        (b"(()(()(", 3),
        (b"))(((((", 3),
        (b"())", -1),
        (b"))(", -1),
        (b")))", -3),
        (b")())())", -3),
    ];

    const PART_2_EXAMPLES: [(&[u8], i64); 2] = [(b")", 1), (b"()())", 5)];

    #[test]
    fn part1_examples() {
        for (input, answer) in EXAMPLES {
            assert_eq!(part1(input), answer);
        }
    }

    #[test]
    fn part2_examples() {
        for (input, answer) in PART_2_EXAMPLES {
            assert_eq!(part2(input), answer);
        }
    }
}

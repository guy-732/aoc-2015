#[inline]
fn count_for_string_minus_in_memory(s: &[u8]) -> usize {
    let mut count = 2;

    if s[0] != b'"' {
        panic!(r#"{:?} does not start with '"' ({})"#, s, b'"');
    }

    if s[s.len() - 1] != b'"' {
        panic!(r#"{:?} does not end with '"' ({})"#, s, b'"');
    }

    let mut i = 1;
    while i < s.len() - 1 {
        if s[i] == b'"' {
            panic!(r#"Unescaped '"'"#);
        }

        if s[i] == b'\\' {
            i += 1;
            if i >= s.len() - 1 {
                panic!("Unfinished escape sequence");
            }

            if s[i] == b'x' {
                i += 2;
                if i >= s.len() - 1 {
                    panic!("Unfinished escape sequence");
                }

                count += 3;
            } else {
                count += 1;
            }
        }

        i += 1;
    }

    count
}

#[inline]
fn extra_characters_to_encode(s: &[u8]) -> usize {
    // + 2 for the 2 '"' around the escaped value
    s.iter().filter(|&&c| matches!(c, b'"' | b'\\')).count() + 2
}

#[aoc(day8, part1)]
fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| count_for_string_minus_in_memory(line.as_bytes()))
        .sum()
}

#[aoc(day8, part2)]
fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| extra_characters_to_encode(line.as_bytes()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = r##"""
"abc"
"aaa\"aaa"
"\x27"
"##;

    #[test]
    fn part1_example1() {
        assert_eq!(part1(EXAMPLE1), 12);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2(EXAMPLE1), 19);
    }
}

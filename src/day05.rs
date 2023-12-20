use fnv::FnvHashSet;

const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

///A nice string is one with all of the following properties:
/// - It contains at least three vowels (aeiou only), like aei, xazegov, or aeiouaeiouaeiou.
/// - It contains at least one letter that appears twice in a row, like xx, abcdde (dd), or aabbccdd (aa, bb, cc, or dd).
/// - It does not contain the strings ab, cd, pq, or xy, even if they are part of one of the other requirements.
fn is_nice(s: &str) -> bool {
    let mut vowels = 0;
    let mut doubled_char = false;
    let mut previous = '\0';
    for c in s.chars() {
        if VOWELS.contains(&c) {
            vowels += 1;
        }

        if c == previous {
            doubled_char = true;
        }

        match (previous, c) {
            ('a', 'b') => return false,
            ('c', 'd') => return false,
            ('p', 'q') => return false,
            ('x', 'y') => return false,
            _ => (),
        }

        previous = c;
    }

    doubled_char && vowels > 2
}


///Now, a nice string is one with all of the following properties:
/// - It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).
/// - It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.
fn is_nice_part2(s: &str) -> bool {
    let mut pairs = FnvHashSet::default();
    let mut first_condition = false;
    let mut second_condition = false;
    let mut previous_previous = '\0';
    let mut previous = '\0';

    for c in s.chars() {
        if pairs.contains(&(previous, c)) {
            first_condition = true;
        }

        pairs.insert((previous_previous, previous));

        if previous_previous == c {
            second_condition = true;
        }

        previous_previous = previous;
        previous = c;
    }

    first_condition && second_condition
}

#[aoc(day5, part1)]
fn part1(input: &str) -> u64 {
    input.lines().filter(|&line| is_nice(line)).count() as u64
}

#[aoc(day5, part2)]
fn part2(input: &str) -> u64 {
    input.lines().filter(|&line| is_nice_part2(line)).count() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(part1("ugknbfddgicrmopn"), 1);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1("aaa"), 1);
    }

    #[test]
    fn part1_example3() {
        assert_eq!(part1("jchzalrnumimnmhp"), 0);
    }

    #[test]
    fn part1_example4() {
        assert_eq!(part1("haegwjzuvuyypxyu"), 0);
    }

    #[test]
    fn part1_example5() {
        assert_eq!(part1("dvszwmarrgswjxmb"), 0);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2("qjhvhtzxzqqjkmpb"), 1);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2("xxyxx"), 1);
    }

    #[test]
    fn part2_example3() {
        assert_eq!(part2("uurcxstgmygtbstg"), 0);
    }

    #[test]
    fn part2_example4() {
        assert_eq!(part2("ieodomkazucvgmuy"), 0);
    }
}

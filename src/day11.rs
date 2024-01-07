use std::error::Error;

use itertools::Itertools;

type Password = [char; 8];

#[derive(Debug, Clone)]
struct PassIter {
    current_pass: Password,
}

impl Iterator for PassIter {
    type Item = Password;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let mut new_pass = self.current_pass;
        for i in (0..8).rev() {
            let c = new_pass[i];
            if c >= 'z' {
                new_pass[i] = 'a';
                continue;
            }

            new_pass[i] = char::from_u32((c as u32) + 1).unwrap();
            break;
        }

        self.current_pass = new_pass;
        Some(new_pass)
    }
}

impl From<Password> for PassIter {
    fn from(value: Password) -> Self {
        Self {
            current_pass: value,
        }
    }
}

#[inline]
fn p1_is_valid_pass(password: &Password) -> bool {
    let mut previous = '\0';
    let mut double_previous = '\0';
    let mut has_increasing_straight = false;
    let mut has_two_pairs = false;
    let mut pair_char = '\0';

    for &c in password {
        if c == 'i' || c == 'o' || c == 'l' {
            return false;
        }

        if previous == c {
            if pair_char != '\0' && pair_char != c {
                has_two_pairs = true;
            } else {
                pair_char = c;
            }
        }

        if is_in_sequence(double_previous, previous, c) {
            has_increasing_straight = true;
        }

        double_previous = previous;
        previous = c;
    }

    has_increasing_straight && has_two_pairs
}

#[inline]
fn is_in_sequence(a: char, b: char, c: char) -> bool {
    ((a as u32) + 1) == (b as u32) && ((b as u32) + 1) == (c as u32)
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Result<Password, Box<dyn Error>> {
    let result = Password::try_from(input.trim().chars().collect_vec());
    result.map_err(|vec| {
        format!(
            "There was not 8 characters in the string (got {} chars)",
            vec.len()
        )
        .into()
    })
}

#[aoc(day11, part1)]
fn part1(initial_pass: &Password) -> String {
    for pass in PassIter::from(*initial_pass) {
        // println!("{:?}", pass);
        if p1_is_valid_pass(&pass) {
            return pass.into_iter().collect();
        }
    }

    unreachable!("No password matched")
}

#[aoc(day11, part2)]
fn part2(initial_pass: &Password) -> String {
    let mut first_matched = false;
    for pass in PassIter::from(*initial_pass) {
        // println!("{:?}", pass);
        if p1_is_valid_pass(&pass) {
            if first_matched {
                return pass.into_iter().collect();
            } else {
                first_matched = true;
            }
        }
    }

    unreachable!("No password matched")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_validity() {
        assert!(!p1_is_valid_pass(&parse("hijklmmn").unwrap()));
        assert!(!p1_is_valid_pass(&parse("abbceffg").unwrap()));
        assert!(!p1_is_valid_pass(&parse("abbcegjk").unwrap()));
        assert!(p1_is_valid_pass(&parse("abcdffaa").unwrap()));
        assert!(p1_is_valid_pass(&parse("ghjaabcc").unwrap()));
    }

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&parse("abcdefgh").unwrap()), "abcdffaa");
        assert_eq!(part1(&parse("ghijklmn").unwrap()), "ghjaabcc");
    }
}

use md5;

#[aoc(day4, part1, brute)]
fn part1_brute(input: &str) -> u64 {
    (1..)
        .find(|num| {
            let digest = md5::compute(format!("{}{}", input, num)).0;
            digest[0] == 0 && digest[1] == 0 && (digest[2] & 0xF0) == 0
        })
        .expect("Could not find any value matching predicate")
}

#[aoc(day4, part2, brute)]
fn part2_brute(input: &str) -> u64 {
    (1..)
        .find(|num| {
            let digest = md5::compute(format!("{}{}", input, num)).0;
            digest[0] == 0 && digest[1] == 0 && digest[2] == 0
        })
        .expect("Could not find any value matching predicate")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "slow"]
    fn part1_brute_example1() {
        assert_eq!(part1_brute("abcdef"), 609043);
    }

    #[test]
    #[ignore = "slow"]
    fn part1_brute_example2() {
        assert_eq!(part1_brute("pqrstuv"), 1048970);
    }
}

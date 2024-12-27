use fnv::FnvHashMap;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Default)]
struct SeatingHappiness(FnvHashMap<String, FnvHashMap<String, i64>>);

impl SeatingHappiness {
    fn total_happiness<S: AsRef<str>>(&self, arrangement: &[&S]) -> i64 {
        let mut result = self[arrangement[arrangement.len() - 1].as_ref()][arrangement[0].as_ref()];
        for i in 0..(arrangement.len() - 1) {
            result += self[arrangement[i].as_ref()][arrangement[i + 1].as_ref()];
        }

        result
    }

    fn maximized_happiness(&self) -> i64 {
        self.keys()
            .permutations(self.len())
            .map(|perm| self.total_happiness(&perm))
            .max()
            .expect("No element?")
    }
}

impl std::ops::Deref for SeatingHappiness {
    type Target = FnvHashMap<String, FnvHashMap<String, i64>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for SeatingHappiness {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

const PARSING_REGEX: &str = r#"^(\S+)\s+\S+\s+(gain|lose)\s+(\d+).+\s+(\S+)\.$"#;

#[aoc_generator(day13, part2)]
fn parse_p2(input: &str) -> SeatingHappiness {
    let mut result = parse(input);
    for other in result.values_mut() {
        other.insert("ME".to_owned(), 0);
    }

    let to_add = result.keys().map(|k| (k.to_owned(), 0)).collect();
    result.insert("ME".to_owned(), to_add);

    result
}

#[aoc_generator(day13, part1)]
fn parse(input: &str) -> SeatingHappiness {
    let mut res = SeatingHappiness::default();
    let regex = Regex::new(PARSING_REGEX).expect("Could not compile regex");
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let m = regex.captures(line.trim()).expect("Could not match regex");
        let p1 = m.get(1).expect("Group 1").as_str().to_owned();
        let p2 = m.get(4).expect("Group 4").as_str().to_owned();
        let mut happiness: i64 = m
            .get(3)
            .expect("Group 3")
            .as_str()
            .parse()
            .expect("Could not parse group 3");
        if m.get(2).expect("Group 2").as_str() == "lose" {
            happiness = -happiness;
        }

        *res.entry(p1.clone())
            .or_default()
            .entry(p2.clone())
            .or_insert(0) += happiness;

        *res.entry(p2).or_default().entry(p1).or_insert(0) += happiness;
    }

    res
}

#[aoc(day13, part1)]
fn part1(input: &SeatingHappiness) -> i64 {
    input.maximized_happiness()
}

#[aoc(day13, part2)]
fn part2(input: &SeatingHappiness) -> i64 {
    input.maximized_happiness()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(&EXAMPLE)), 330);
    }
}

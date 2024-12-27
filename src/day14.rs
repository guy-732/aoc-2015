use fnv::FnvHashMap;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Reindeer {
    name: String,
    speed: u64,
    fly_time: u64,
    rest_time: u64,
}

impl Reindeer {
    fn distance_after_n_seconds(&self, n: u64) -> u64 {
        let cycle_time = self.fly_time + self.rest_time;
        let full_cycles = n / cycle_time;
        let current_cycle_progress = n % cycle_time;

        let distance = full_cycles * self.speed * self.fly_time;
        distance + (current_cycle_progress.min(self.fly_time) * self.speed)
    }
}

fn race_for_n_seconds(reindeers: &[Reindeer], n: u64) -> FnvHashMap<Reindeer, u64> {
    let mut result: FnvHashMap<Reindeer, u64> = reindeers
        .iter()
        .map(|reindeer| (reindeer.clone(), 0))
        .collect();

    for secs in 1..(n + 1) {
        let distances = reindeers
            .iter()
            .map(|reindeer| (reindeer, reindeer.distance_after_n_seconds(secs)))
            .collect_vec();

        let max_dist = distances
            .iter()
            .map(|(_, dist)| *dist)
            .max()
            .expect("No reindeers");
        for (reindeer, _) in distances.into_iter().filter(|(_, dist)| *dist == max_dist) {
            *result.get_mut(reindeer).unwrap() += 1;
        }
    }

    result
}

const PARSING_REGEX: &str = r#"^(\S+).+?(\d+)\s+km/s.+?(\d+) seconds?.+?(\d+) seconds?\.$"#;

#[aoc_generator(day14)]
fn parse(input: &str) -> Vec<Reindeer> {
    let regex = Regex::new(PARSING_REGEX).expect("Could not compile parsing regex");
    let mut res = vec![];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        let m = regex.captures(line.trim()).expect("Could not match line");
        let name = m.get(1).expect("Group 1").as_str().to_owned();
        let speed = m
            .get(2)
            .expect("Group 2")
            .as_str()
            .parse()
            .expect("Could not parse u64");
        let fly_time = m
            .get(3)
            .expect("Group 3")
            .as_str()
            .parse()
            .expect("Could not parse u64");
        let rest_time = m
            .get(4)
            .expect("Group 4")
            .as_str()
            .parse()
            .expect("Could not parse u64");

        res.push(Reindeer {
            name,
            speed,
            fly_time,
            rest_time,
        });
    }

    res
}

const MAX_SECONDS: u64 = 2503;

#[aoc(day14, part1)]
fn part1(reindeers: &[Reindeer]) -> u64 {
    reindeers
        .iter()
        .map(|reindeer| reindeer.distance_after_n_seconds(MAX_SECONDS))
        .max()
        .expect("No reindeers")
}

#[aoc(day14, part2)]
fn part2(reindeers: &[Reindeer]) -> u64 {
    race_for_n_seconds(reindeers, MAX_SECONDS)
        .into_values()
        .max()
        .expect("No reindeers")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let comet = Reindeer {
            name: "Comet".to_owned(),
            speed: 14,
            fly_time: 10,
            rest_time: 127,
        };
        let dancer = Reindeer {
            name: "Dancer".to_owned(),
            speed: 16,
            fly_time: 11,
            rest_time: 162,
        };

        assert_eq!(comet.distance_after_n_seconds(1000), 1120);
        assert_eq!(dancer.distance_after_n_seconds(1000), 1056);
    }

    #[test]
    fn part2_example() {
        let comet = Reindeer {
            name: "Comet".to_owned(),
            speed: 14,
            fly_time: 10,
            rest_time: 127,
        };
        let dancer = Reindeer {
            name: "Dancer".to_owned(),
            speed: 16,
            fly_time: 11,
            rest_time: 162,
        };

        let pts = race_for_n_seconds(&[comet.clone(), dancer.clone()], 1000);
        assert_eq!(pts[&comet], 312);
        assert_eq!(pts[&dancer], 689);
    }
}

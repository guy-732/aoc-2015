use std::{error, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Gift {
    height: u32,
    width: u32,
    length: u32,
}

impl Gift {
    pub(crate) fn calculate_wrapping_required(&self) -> u64 {
        let sides = [
            (self.length as u64 * self.width as u64),
            (self.width as u64 * self.height as u64),
            (self.length as u64 * self.height as u64),
        ];
        (2 * sides.iter().sum::<u64>()) + sides.into_iter().min().unwrap()
    }

    pub(crate) const fn ribbon_for_bow(&self) -> u64 {
        self.height as u64 * self.length as u64 * self.width as u64
    }

    pub(crate) const fn ribbon_required(&self) -> u64 {
        let (shortest_1, largest) = if self.height < self.width {
            (self.height, self.width)
        } else {
            (self.width, self.height)
        };

        let shortest_2 = if largest < self.length {
            largest
        } else {
            self.length
        };

        2 * (shortest_1 as u64 + shortest_2 as u64) + self.ribbon_for_bow()
    }
}

impl FromStr for Gift {
    type Err = Box<dyn error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut splits = s.split('x');
        Ok(Self {
            height: splits
                .next()
                .ok_or_else(|| format!("{:?} has -1 parts when split on 'x'?", s))?
                .parse()?,
            width: splits
                .next()
                .ok_or_else(|| format!("{:?} could not be split on 'x'", s))?
                .parse()?,
            length: splits
                .next()
                .ok_or_else(|| format!("{:?} could not be split twice on 'x'?", s))?
                .parse()?,
        })
    }
}

#[aoc_generator(day2)]
fn parser(input: &str) -> Result<Vec<Gift>, Box<dyn error::Error>> {
    input
        .lines()
        .filter_map(|line| {
            let line = line.trim();
            if line.is_empty() {
                None
            } else {
                Some(line.parse())
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(gifts: &[Gift]) -> u64 {
    gifts
        .iter()
        .map(|gift| gift.calculate_wrapping_required())
        .sum()
}

#[aoc(day2, part2)]
fn part2(gifts: &[Gift]) -> u64 {
    gifts.iter().map(|gift| gift.ribbon_required()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_1() {
        assert_eq!(part1(&parser("2x3x4").unwrap()), 58);
    }

    #[test]
    fn part1_example_2() {
        assert_eq!(part1(&parser("1x1x10").unwrap()), 43);
    }

    #[test]
    fn part2_example_1() {
        assert_eq!(part2(&parser("2x3x4").unwrap()), 34);
    }

    #[test]
    fn part2_example_2() {
        assert_eq!(part2(&parser("1x1x10").unwrap()), 14);
    }
}

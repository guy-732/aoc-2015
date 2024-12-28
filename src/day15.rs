use regex::Regex;

#[derive(Debug, Clone, Copy)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl std::ops::Add<Ingredient> for Ingredient {
    type Output = Ingredient;

    fn add(mut self, rhs: Ingredient) -> Self::Output {
        self += rhs;
        self
    }
}

impl std::ops::AddAssign<Ingredient> for Ingredient {
    fn add_assign(&mut self, rhs: Ingredient) {
        self.durability += rhs.durability;
        self.capacity += rhs.capacity;
        self.flavor += rhs.flavor;
        self.texture += rhs.texture;
        self.calories += rhs.calories;
    }
}

impl std::ops::Mul<i64> for Ingredient {
    type Output = Ingredient;

    fn mul(self, rhs: i64) -> Self::Output {
        Ingredient {
            capacity: self.capacity * rhs,
            durability: self.durability * rhs,
            flavor: self.flavor * rhs,
            texture: self.texture * rhs,
            calories: self.calories * rhs,
        }
    }
}

impl std::ops::MulAssign<i64> for Ingredient {
    fn mul_assign(&mut self, rhs: i64) {
        *self = *self * rhs;
    }
}

impl Ingredient {
    fn score_p1(&self) -> i64 {
        if self.capacity <= 0 || self.durability <= 0 || self.flavor <= 0 || self.texture <= 0 {
            return 0;
        }

        self.capacity * self.durability * self.flavor * self.texture
    }

    fn score_p2(&self) -> i64 {
        if self.calories == 500 {
            self.score_p1()
        } else {
            0
        }
    }
}

const PARSING_REGEX: &str = r"^(\S+):\D+?(-?\d+)\D+?(-?\d+)\D+?(-?\d+)\D+?(-?\d+)\D+?(-?\d+)";

#[aoc_generator(day15)]
fn parse(input: &str) -> Vec<(String, Ingredient)> {
    let regex = Regex::new(PARSING_REGEX).expect("Could not compile regex");

    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let m = regex.captures(line).expect("Could not match regex");
            (
                m.get(1).unwrap().as_str().to_owned(),
                Ingredient {
                    capacity: m.get(2).unwrap().as_str().parse().unwrap(),
                    durability: m.get(3).unwrap().as_str().parse().unwrap(),
                    flavor: m.get(4).unwrap().as_str().parse().unwrap(),
                    texture: m.get(5).unwrap().as_str().parse().unwrap(),
                    calories: m.get(6).unwrap().as_str().parse().unwrap(),
                },
            )
        })
        .collect()
}

fn part1_rec(left: i64, current_ingredients: Ingredient, input: &[(String, Ingredient)]) -> i64 {
    if input.len() == 1 {
        return (current_ingredients + (input[0].1 * left)).score_p1();
    }

    let mut max = 0;

    for i in 0..left {
        let current = current_ingredients + (input[0].1 * i);
        let res = part1_rec(left - i, current, &input[1..]);
        if res > max {
            max = res;
        }
    }

    max
}

#[aoc(day15, part1)]
fn part1(input: &[(String, Ingredient)]) -> i64 {
    (0..100)
        .map(|i| part1_rec(100 - i, input[0].1 * i, &input[1..]))
        .max()
        .expect("No values")
}

fn part2_rec(left: i64, current_ingredients: Ingredient, input: &[(String, Ingredient)]) -> i64 {
    if input.len() == 1 {
        return (current_ingredients + (input[0].1 * left)).score_p2();
    }

    let mut max = 0;

    for i in 0..left {
        let current = current_ingredients + (input[0].1 * i);
        let res = part2_rec(left - i, current, &input[1..]);
        if res > max {
            max = res;
        }
    }

    max
}

#[aoc(day15, part2)]
fn part2(input: &[(String, Ingredient)]) -> i64 {
    (0..100)
        .map(|i| part2_rec(100 - i, input[0].1 * i, &input[1..]))
        .max()
        .expect("No values")
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str =
        "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 62842880);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 57600000);
    }
}

use std::num::ParseIntError;

#[aoc_generator(day20)]
fn parse(input: &str) -> Result<usize, ParseIntError> {
    input.trim().parse()
}

#[aoc(day20, part1)]
fn part1(threshold: &usize) -> usize {
    let threshold = *threshold / 10 + (*threshold % 10 != 0) as usize;
    let mut houses = vec![0; threshold];
    for i in 1..threshold {
        for j in (i..threshold).step_by(i) {
            houses[j] += i;
        }
    }

    houses
        .into_iter()
        .enumerate()
        .find_map(|(i, g)| (g >= threshold).then_some(i))
        .expect("Could not find house")
}

#[aoc(day20, part2)]
fn part2(threshold: &usize) -> usize {
    let house_count = *threshold / 10 + (*threshold % 10 != 0) as usize;
    let mut houses = vec![0; house_count];
    for i in 1..house_count {
        for j in (i..house_count).step_by(i).take(50) {
            houses[j] += i * 11;
        }
    }

    houses
        .into_iter()
        .enumerate()
        .find_map(|(i, g)| (g >= *threshold).then_some(i))
        .expect("Could not find house")
}

// #[cfg(test)]
// mod tests {
//     use super::*;
// }

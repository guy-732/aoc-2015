use fnv::FnvHashSet;

#[aoc(day3, part1)]
fn part1(input: &str) -> u64 {
    let mut delivered = FnvHashSet::default();
    let mut current_row: i32 = 0;
    let mut current_column: i32 = 0;

    delivered.insert((current_row, current_column));

    for c in input.chars() {
        match c {
            '^' => current_row -= 1,
            '>' => current_column += 1,
            'v' => current_row += 1,
            '<' => current_column -= 1,
            _ => (),
        }

        delivered.insert((current_row, current_column));
    }

    delivered.len() as u64
}

#[aoc(day3, part2)]
fn part2(input: &str) -> u64 {
    let mut delivered = FnvHashSet::default();
    let mut current_row: i32 = 0;
    let mut current_column: i32 = 0;
    let mut current_row_robot: i32 = 0;
    let mut current_column_robot: i32 = 0;

    delivered.insert((current_row, current_column));

    for (i, c) in input.chars().enumerate() {
        if i % 2 == 0 {
            match c {
                '^' => current_row -= 1,
                '>' => current_column += 1,
                'v' => current_row += 1,
                '<' => current_column -= 1,
                _ => (),
            }

            delivered.insert((current_row, current_column));
        } else {
            match c {
                '^' => current_row_robot -= 1,
                '>' => current_column_robot += 1,
                'v' => current_row_robot += 1,
                '<' => current_column_robot -= 1,
                _ => (),
            }

            delivered.insert((current_row_robot, current_column_robot));
        }
    }

    delivered.len() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example1() {
        assert_eq!(part1(">"), 2);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1("^>v<"), 4);
    }

    #[test]
    fn part1_example3() {
        assert_eq!(part1("^v^v^v^v^v"), 2);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2("^v"), 3);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2("^>v<"), 3);
    }

    #[test]
    fn part2_example3() {
        assert_eq!(part2("^v^v^v^v^v"), 11);
    }
}

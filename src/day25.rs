use regex::Regex;

const PARSING_REGEX: &str = r#"\D(\d+)\D+(\d+)\D"#;

#[aoc_generator(day25)]
fn parse(input: &str) -> (u32, u32) {
    let regex = Regex::new(PARSING_REGEX).expect("Failed to compile regex");
    let m = regex.captures(input).expect("Could not parse input");
    (
        m.get(1)
            .expect("Group 1")
            .as_str()
            .parse()
            .expect("Could not parse 1st number"),
        m.get(2)
            .expect("Group 2")
            .as_str()
            .parse()
            .expect("Could not parse 2nd number"),
    )
}

#[aoc(day25, part1)]
fn solve(grid_pos: &(u32, u32)) -> u64 {
    let in_seq = pos_to_sequence_index(*grid_pos);
    compute_nth_seq(in_seq)
}

const INITIAL_VALUE: u64 = 20151125;
const MULTIPLY: u64 = 252533;
const MODULO: u64 = 33554393;

fn compute_nth_seq(nth: u32) -> u64 {
    let mut current = INITIAL_VALUE;
    for _ in 1..nth {
        current = current.wrapping_mul(MULTIPLY);
        current %= MODULO;
    }

    current
}

fn pos_to_sequence_index(grid_pos: (u32, u32)) -> u32 {
    let row_idx = grid_pos.0 + grid_pos.1 - 2;
    let idx_before_row_idx = row_idx * (row_idx + 1) / 2;
    idx_before_row_idx + grid_pos.1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sequence_idx_examples() {
        assert_eq!(pos_to_sequence_index((4, 3)), 18);
        assert_eq!(pos_to_sequence_index((3, 4)), 19);
        assert_eq!(pos_to_sequence_index((4, 2)), 12);
        assert_eq!(pos_to_sequence_index((1, 5)), 15);
    }

    #[test]
    fn compute_nth_seq_examples() {
        let mut current = INITIAL_VALUE;
        for i in 1..50 {
            assert_eq!(compute_nth_seq(i), current);

            current = current.wrapping_mul(MULTIPLY) % MODULO;
        }
    }
}

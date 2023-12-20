use std::{error::Error, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum InstructionKind {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Instruction {
    kind: InstructionKind,
    start_pos: (u32, u32),
    end_pos: (u32, u32),
}

#[inline]
fn parse_position(s: &str) -> Result<(u32, u32), Box<dyn Error>> {
    let Some((first, second)) = s.split_once(',') else {
        return Err(format!(r#"{:?} did not contain ','"#, s).into());
    };

    Ok((first.parse()?, second.parse()?))
}

#[inline]
fn parse_range(s: &str) -> Result<((u32, u32), (u32, u32)), Box<dyn Error>> {
    let Some((first, second)) = s.split_once(" through ") else {
        return Err(format!(r#"{:?} did not contain " through ""#, s).into());
    };

    Ok((parse_position(first)?, parse_position(second)?))
}

impl FromStr for Instruction {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(rest) = s.strip_prefix("turn on ") {
            let (start_pos, end_pos) = parse_range(rest)?;
            Ok(Self {
                kind: InstructionKind::TurnOn,
                start_pos,
                end_pos,
            })
        } else if let Some(rest) = s.strip_prefix("turn off ") {
            let (start_pos, end_pos) = parse_range(rest)?;
            Ok(Self {
                kind: InstructionKind::TurnOff,
                start_pos,
                end_pos,
            })
        } else if let Some(rest) = s.strip_prefix("toggle ") {
            let (start_pos, end_pos) = parse_range(rest)?;
            Ok(Self {
                kind: InstructionKind::Toggle,
                start_pos,
                end_pos,
            })
        } else {
            Err(format!(
                r#"{:?} did not start with any of "turn on ", "turn off " or "toggle ""#,
                s
            )
            .into())
        }
    }
}

#[aoc_generator(day6)]
fn parse(input: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    input.lines().map(|line| line.parse()).collect()
}

#[aoc(day6, part1, whole_grid)]
fn part1_whole_grid(instructions: &[Instruction]) -> u64 {
    let mut grid = (0..1000)
        .map(|_| (0..1000).map(|_| false).collect::<Box<[_]>>())
        .collect::<Box<_>>();

    for instr in instructions {
        // dbg!(instr);
        for row in instr.start_pos.0..=instr.end_pos.0 {
            for col in instr.start_pos.1..=instr.end_pos.1 {
                match instr.kind {
                    InstructionKind::TurnOn => grid[row as usize][col as usize] = true,
                    InstructionKind::TurnOff => grid[row as usize][col as usize] = false,
                    InstructionKind::Toggle => {
                        grid[row as usize][col as usize] = !grid[row as usize][col as usize]
                    }
                }
            }
        }
    }

    grid.iter()
        .flat_map(|row| row.iter())
        .filter(|&&is_on| is_on)
        .count() as u64
}

#[aoc(day6, part2, whole_grid)]
fn part2_whole_grid(instructions: &[Instruction]) -> u64 {
    let mut grid = (0..1000)
        .map(|_| (0..1000).map(|_| 0_u32).collect::<Box<[_]>>())
        .collect::<Box<_>>();

    for instr in instructions {
        // dbg!(instr);
        for row in instr.start_pos.0..=instr.end_pos.0 {
            for col in instr.start_pos.1..=instr.end_pos.1 {
                match instr.kind {
                    InstructionKind::TurnOn => grid[row as usize][col as usize] += 1,
                    InstructionKind::TurnOff => {
                        if let Some(new) = grid[row as usize][col as usize].checked_sub(1) {
                            grid[row as usize][col as usize] = new;
                        }
                    }
                    InstructionKind::Toggle => grid[row as usize][col as usize] += 2,
                }
            }
        }
    }

    grid.iter()
        .flat_map(|row| row.iter())
        .cloned()
        .map(u64::from)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_whole_grid_example1() {
        assert_eq!(
            part1_whole_grid(&parse("turn on 0,0 through 999,999").unwrap()),
            1_000_000
        );
    }

    #[test]
    fn part1_whole_grid_example2() {
        assert_eq!(
            part1_whole_grid(
                &parse("turn on 0,0 through 999,999\ntoggle 0,0 through 999,0").unwrap()
            ),
            1_000_000 - 1_000
        );
    }

    #[test]
    fn part1_whole_grid_example3() {
        assert_eq!(
            part1_whole_grid(&parse("turn on 0,0 through 999,999\ntoggle 0,0 through 999,0\nturn off 499,499 through 500,500").unwrap()),
            1_000_000 - 1_000 - 4
        );
    }

    #[test]
    fn part2_whole_grid_example1() {
        assert_eq!(
            part2_whole_grid(&parse("turn on 0,0 through 0,0").unwrap()),
            1
        );
    }

    #[test]
    fn part2_whole_grid_example2() {
        assert_eq!(
            part2_whole_grid(&parse("toggle 0,0 through 999,999").unwrap()),
            2_000_000
        );
    }
}

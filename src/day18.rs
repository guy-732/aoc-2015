#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position(isize, isize);

const DIRECT_NEIGHBORS: [Position; 8] = [
    Position(1, 0),
    Position(1, 1),
    Position(0, 1),
    Position(-1, 1),
    Position(-1, 0),
    Position(-1, -1),
    Position(0, -1),
    Position(1, -1),
];

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Mul<isize> for Position {
    type Output = Position;

    fn mul(self, rhs: isize) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl std::ops::MulAssign<isize> for Position {
    fn mul_assign(&mut self, rhs: isize) {
        *self = *self * rhs;
    }
}

#[derive(Debug, Clone)]
struct LightGrid {
    grid: Vec<Vec<bool>>,
}

impl LightGrid {
    fn count_active(&self) -> u64 {
        self.grid.iter().flatten().filter(|&&light| light).count() as u64
    }

    fn get(&self, pos: Position) -> Option<&bool> {
        if pos.0 < 0 || pos.1 < 0 {
            return None;
        }

        self.grid
            .get(pos.0 as usize)
            .and_then(|row| row.get(pos.1 as usize))
    }

    fn get_mut(&mut self, pos: Position) -> Option<&mut bool> {
        if pos.0 < 0 || pos.1 < 0 {
            return None;
        }

        self.grid
            .get_mut(pos.0 as usize)
            .and_then(|row| row.get_mut(pos.1 as usize))
    }

    fn next_state_from(&mut self, other: &Self) {
        let row_len = self.grid[0].len() as isize;
        for i in 0..self.grid.len() as isize {
            for j in 0..row_len {
                let current = Position(i, j);
                let active_neighbours = DIRECT_NEIGHBORS
                    .into_iter()
                    .map(|delta| current + delta)
                    .filter(|&neighbour| other[neighbour])
                    .count();

                if other[current] {
                    self[current] = matches!(active_neighbours, 2 | 3);
                } else {
                    self[current] = active_neighbours == 3;
                }
            }
        }
    }

    fn force_corners_on(&mut self) {
        let len = self.grid.len() - 1;
        let row_len = self.grid[0].len() - 1;
        self.grid[0][0] = true;
        self.grid[0][row_len] = true;
        self.grid[len][0] = true;
        self.grid[len][row_len] = true;
    }
}

impl std::fmt::Display for LightGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for &b in row {
                if b {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl std::ops::Index<Position> for LightGrid {
    type Output = bool;

    fn index(&self, index: Position) -> &Self::Output {
        self.get(index).unwrap_or(&false)
    }
}

impl std::ops::IndexMut<Position> for LightGrid {
    fn index_mut(&mut self, index: Position) -> &mut Self::Output {
        self.get_mut(index)
            .unwrap_or_else(|| panic!("Could not index mut position {index} in the grid"))
    }
}

fn state_after_n_steps(orig: &LightGrid, n: u64) -> LightGrid {
    let mut current = orig.clone();
    let mut next = orig.clone();

    for _ in 0..n {
        next.next_state_from(&current);
        std::mem::swap(&mut current, &mut next);
    }

    current
}

fn state_after_n_steps_p2(orig: &LightGrid, n: u64) -> LightGrid {
    let mut current = orig.clone();
    current.force_corners_on();
    let mut next = orig.clone();

    for _ in 0..n {
        next.next_state_from(&current);
        next.force_corners_on();
        std::mem::swap(&mut current, &mut next);
    }

    current
}

#[aoc_generator(day18)]
fn parse(input: &str) -> LightGrid {
    let mut lines = vec![];
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }

        lines.push(line.trim().as_bytes().iter().map(|&b| b == b'#').collect());
    }

    LightGrid { grid: lines }
}

#[aoc(day18, part1)]
fn part1(grid: &LightGrid) -> u64 {
    state_after_n_steps(grid, 100).count_active()
}

#[aoc(day18, part2)]
fn part2(grid: &LightGrid) -> u64 {
    state_after_n_steps_p2(grid, 100).count_active()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";

    #[test]
    fn part1_example() {
        let grid = parse(EXAMPLE);
        let after_steps = state_after_n_steps(&grid, 4);
        println!("{after_steps}");
        assert_eq!(after_steps.count_active(), 4);
    }

    #[test]
    fn part2_example() {
        let grid = parse(EXAMPLE);
        let after_steps = state_after_n_steps_p2(&grid, 5);
        println!("{after_steps}");
        assert_eq!(after_steps.count_active(), 17);
    }
}

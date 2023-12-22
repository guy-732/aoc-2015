use std::{io, error::Error};

use fnv::{FnvHashMap, FnvHashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Edge {
    weight: u64,
    target_vertex: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Graph {
    vertices: Vec<(String, FnvHashSet<Edge>)>,
}

impl Graph {
    #[allow(dead_code)]
    fn write_as_gv<W: io::Write>(&self, writer: &mut W) -> io::Result<()> {
        writeln!(writer, "graph {{\n    layout = \"circo\"\n")?;
        for (vertex_id, (vertex, _)) in self.vertices.iter().enumerate() {
            writeln!(writer, "    v{} [label={:?}]", vertex_id, vertex)?;
        }

        writeln!(writer)?;
        for (vertex_id, (_, adjacents)) in self.vertices.iter().enumerate() {
            for edge in adjacents.iter() {
                if vertex_id < edge.target_vertex {
                    writeln!(
                        writer,
                        r#"    v{} -- v{} [label="{}"]"#,
                        vertex_id, edge.target_vertex, edge.weight
                    )?;
                }
            }
        }

        writeln!(writer, "}}")?;

        Ok(())
    }

    /// Assumes `self` is a complete graph
    fn find_shortest_hamiltonian_path(&self) -> u64 {
        (0..self.vertices.len())
            .map(|i| self.shortest_hamiltonian_impl(i, FnvHashSet::default()))
            .min()
            .unwrap()
    }

    fn shortest_hamiltonian_impl(&self, current: usize, mut visited: FnvHashSet<usize>) -> u64 {
        visited.insert(current);
        self.vertices[current].1
            .iter()
            .filter_map(|edge| {
                if visited.contains(&edge.target_vertex) {
                    None
                } else {
                    Some(
                        edge.weight
                            + self.shortest_hamiltonian_impl(edge.target_vertex, visited.clone())
                    )
                }
            })
            .min()
            .unwrap_or(0)
    }

    /// Assumes `self` is a complete graph
    fn find_longest_hamiltonian_path(&self) -> u64 {
        (0..self.vertices.len())
            .map(|i| self.longest_hamiltonian_impl(i, FnvHashSet::default()))
            .max()
            .unwrap()
    }

    fn longest_hamiltonian_impl(&self, current: usize, mut visited: FnvHashSet<usize>) -> u64 {
        visited.insert(current);
        self.vertices[current].1
            .iter()
            .filter_map(|edge| {
                if visited.contains(&edge.target_vertex) {
                    None
                } else {
                    Some(
                        edge.weight
                            + self.longest_hamiltonian_impl(edge.target_vertex, visited.clone())
                    )
                }
            })
            .max()
            .unwrap_or(0)
    }
}

impl<'s> FromIterator<&'s str> for Graph {
    fn from_iter<T: IntoIterator<Item = &'s str>>(iter: T) -> Self {
        let mut vertex_name_to_id = FnvHashMap::default();
        let mut vertices = vec![];
        for line in iter {
            let (destinations, distance) = line
                .split_once('=')
                .ok_or_else(|| format!("{:?} did not contain an '=' sign", line))
                .unwrap();

            let (dest1, dest2) = destinations
                .split_once(" to ")
                .ok_or_else(|| {
                    format!(r#"{:?} did not contain the substring " to ""#, destinations)
                })
                .unwrap();

            let dest1 = dest1.trim();
            let dest2 = dest2.trim();

            let dest1_id = if let Some(&id) = vertex_name_to_id.get(dest1) {
                id
            } else {
                let new_id = vertices.len();
                vertices.push((dest1.to_owned(), FnvHashSet::default()));
                vertex_name_to_id.insert(dest1.to_owned(), new_id);

                new_id
            };

            let dest2_id = if let Some(&id) = vertex_name_to_id.get(dest2) {
                id
            } else {
                let new_id = vertices.len();
                vertices.push((dest2.to_owned(), FnvHashSet::default()));
                vertex_name_to_id.insert(dest2.to_owned(), new_id);

                new_id
            };

            let distance = distance.trim().parse().unwrap();
            vertices[dest1_id].1.insert(Edge {
                weight: distance,
                target_vertex: dest2_id,
            });
            vertices[dest2_id].1.insert(Edge {
                weight: distance,
                target_vertex: dest1_id,
            });
        }

        Self { vertices }
    }
}

#[aoc_generator(day9)]
fn parse(input: &str) -> Graph {
    input.lines().collect()
}

#[aoc(day9, part1)]
fn part1(graph: &Graph) -> Result<u64, Box<dyn Error>> {
    // graph.write_as_gv(&mut io::stdout())?;
    Ok(graph.find_shortest_hamiltonian_path())
}

#[aoc(day9, part2)]
fn part2(graph: &Graph) -> u64 {
    graph.find_longest_hamiltonian_path()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE1: &str = r#"London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141"#;

    #[test]
    fn part1_example1() {
        assert_eq!(part1(&parse(EXAMPLE1)).unwrap(), 605);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2(&parse(EXAMPLE1)), 982);
    }
}

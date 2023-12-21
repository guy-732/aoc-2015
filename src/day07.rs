use core::fmt;
use fnv::FnvHashMap;
use itertools::Itertools;
use std::{
    cell::RefCell,
    error::Error,
    ops::{BitAnd, BitOr, Not, Shl, Shr},
    str::FromStr,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum LogicGate {
    And(String, String),
    Or(String, String),
    Not(String),
    LeftShift(String, u8),
    RightShift(String, u8),
    Constant(String),
}

impl LogicGate {
    #[inline]
    fn get_wire_value(&self, wires: &FnvHashMap<String, Wire>) -> u16 {
        match self {
            Self::Constant(constant) => wires
                .get(constant)
                .map(|wire| wire.get_wire_value(wires))
                .unwrap_or_else(|| constant.parse().unwrap()),
            Self::Not(ident) => wires
                .get(ident)
                .map(|wire| wire.get_wire_value(wires))
                .unwrap_or_else(|| ident.parse().unwrap())
                .not(),
            Self::LeftShift(ident, shift) => wires
                .get(ident)
                .map(|wire| wire.get_wire_value(wires))
                .unwrap_or_else(|| ident.parse().unwrap())
                .shl(shift),
            Self::RightShift(ident, shift) => wires
                .get(ident)
                .map(|wire| wire.get_wire_value(wires))
                .unwrap_or_else(|| ident.parse().unwrap())
                .shr(shift),
            Self::And(ident1, ident2) => {
                let left = wires
                    .get(ident1)
                    .map(|wire| wire.get_wire_value(wires))
                    .unwrap_or_else(|| ident1.parse().unwrap());
                let right = wires
                    .get(ident2)
                    .map(|wire| wire.get_wire_value(wires))
                    .unwrap_or_else(|| ident2.parse().unwrap());
                left.bitand(right)
            }
            Self::Or(ident1, ident2) => {
                let left = wires
                    .get(ident1)
                    .map(|wire| wire.get_wire_value(wires))
                    .unwrap_or_else(|| ident1.parse().unwrap());
                let right = wires
                    .get(ident2)
                    .map(|wire| wire.get_wire_value(wires))
                    .unwrap_or_else(|| ident2.parse().unwrap());
                left.bitor(right)
            }
        }
    }
}

impl FromStr for LogicGate {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(ident) = s.strip_prefix("NOT") {
            Ok(Self::Not(ident.trim().to_owned()))
        } else if let Some((ident1, ident2)) = s.split_once("AND") {
            Ok(Self::And(
                ident1.trim().to_owned(),
                ident2.trim().to_owned(),
            ))
        } else if let Some((ident1, ident2)) = s.split_once("OR") {
            Ok(Self::Or(ident1.trim().to_owned(), ident2.trim().to_owned()))
        } else if let Some((ident, shift_arg)) = s.split_once("RSHIFT") {
            Ok(Self::RightShift(
                ident.trim().to_owned(),
                shift_arg.trim().parse()?,
            ))
        } else if let Some((ident, shift_arg)) = s.split_once("LSHIFT") {
            Ok(Self::LeftShift(
                ident.trim().to_owned(),
                shift_arg.trim().parse()?,
            ))
        } else {
            Ok(Self::Constant(s.trim().to_owned()))
        }
    }
}

impl fmt::Display for LogicGate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::And(ident1, ident2) => write!(f, "{} AND {}", ident1, ident2),
            Self::Or(ident1, ident2) => write!(f, "{} OR {}", ident1, ident2),
            Self::Not(ident) => write!(f, "NOT {}", ident),
            Self::LeftShift(ident, shift) => write!(f, "{} LSHIFT {}", ident, shift),
            Self::RightShift(ident, shift) => write!(f, "{} RSHIFT {}", ident, shift),
            Self::Constant(constant) => write!(f, "{}", constant),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Wire {
    logic_gate: LogicGate,
    cached_result: RefCell<Option<u16>>,
}

impl Wire {
    #[inline]
    fn get_wire_value(&self, wires: &FnvHashMap<String, Wire>) -> u16 {
        let borrow = self.cached_result.borrow();
        if let Some(cached) = *borrow {
            cached
        } else {
            drop(borrow);
            let result = self.logic_gate.get_wire_value(wires);
            *self.cached_result.borrow_mut() = Some(result);
            result
        }
    }
}

impl FromStr for Wire {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            logic_gate: s.parse()?,
            cached_result: RefCell::new(None),
        })
    }
}

#[aoc_generator(day7)]
fn parse(input: &str) -> Result<FnvHashMap<String, Wire>, Box<dyn Error>> {
    let elements: Vec<_> = input
        .lines()
        .map(|line| {
            let (wire, dest) = line
                .split_once("->")
                .ok_or_else(|| format!(r#"Could not split {:?} on "->""#, line))?;
            Ok::<_, Box<dyn Error>>((dest.trim().to_owned(), wire.trim().parse()?))
        })
        .try_collect()?;

    Ok(FnvHashMap::from_iter(elements))
}

#[aoc(day7, part1)]
fn part1(wires: &FnvHashMap<String, Wire>) -> Result<u16, Box<dyn Error>> {
    let wires = wires.clone();
    Ok(get_wire_value(&wires, "a")?)
}

#[aoc(day7, part2)]
fn part2(wires: &FnvHashMap<String, Wire>) -> Result<u16, Box<dyn Error>> {
    let new_wires = wires.clone();
    let result = part1(wires)?;

    {
        let wire_b = new_wires.get("b").unwrap();
        let mut borrow = wire_b.cached_result.borrow_mut();
        *borrow = Some(result);
    }

    Ok(get_wire_value(&new_wires, "a")?)
}

fn get_wire_value(wires: &FnvHashMap<String, Wire>, wire: &str) -> Result<u16, String> {
    let wire = wires
        .get(wire)
        .ok_or_else(|| format!("Key {:?} does not exist in the hashmap", wire))?;
    Ok(wire.get_wire_value(wires))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#"123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i"#;

    const EXAMPLE_1_RESULT: [(&str, u16); 8] = [
        ("d", 72),
        ("e", 507),
        ("f", 492),
        ("g", 114),
        ("h", 65412),
        ("i", 65079),
        ("x", 123),
        ("y", 456),
    ];

    #[test]
    fn part1_example1() {
        let wires = parse(EXAMPLE_1).unwrap();
        for (wire, result) in EXAMPLE_1_RESULT {
            assert_eq!(get_wire_value(&wires, wire).unwrap(), result);
        }
    }
}

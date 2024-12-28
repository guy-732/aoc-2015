use std::u64;

#[derive(Debug, Clone, Copy, Eq)]
struct Sue {
    num: u64,
    children: u64,
    cats: u64,
    samoyeds: u64,
    pomeranians: u64,
    akitas: u64,
    vizslas: u64,
    goldfish: u64,
    trees: u64,
    cars: u64,
    perfumes: u64,
}

impl Sue {
    const fn new_p1(num: u64) -> Self {
        Self {
            num,
            ..Self::searched()
        }
    }

    const fn searched() -> Self {
        Self {
            num: u64::MAX,
            children: 3,
            cats: 7,
            samoyeds: 2,
            pomeranians: 3,
            akitas: 0,
            vizslas: 0,
            goldfish: 5,
            trees: 3,
            cars: 2,
            perfumes: 1,
        }
    }

    const fn new_p2(num: u64) -> Self {
        Self {
            num,
            cats: u64::MAX,
            trees: u64::MAX,
            pomeranians: 0,
            goldfish: 0,
            ..Self::searched()
        }
    }

    fn parse_line_1(line: &str) -> Self {
        macro_rules! parse {
            ($to_match:expr, $default:expr, $sue:ident, $count:expr, $( $f:ident ),*) => {
                match $to_match {
                    $(stringify!($f) => $sue.$f = $count,)*
                    _ => $default,
                }
            };
        }

        let (num, rest) = line.split_once(':').expect("Could not split on ':'");
        let num = num.trim_start_matches("Sue").trim().parse().unwrap();
        let mut sue = Self::new_p1(num);
        for field in rest.split(',') {
            let (field, count) = field.split_once(':').expect("Could not split on ':'");
            let count = count.trim().parse().unwrap();
            parse!(
                field.trim(),
                panic!("Unrecognized field {:?}", field.trim()),
                sue,
                count,
                children,
                cats,
                samoyeds,
                pomeranians,
                akitas,
                vizslas,
                goldfish,
                trees,
                cars,
                perfumes
            );
        }

        sue
    }

    fn parse_line_2(line: &str) -> Self {
        macro_rules! parse {
            ($to_match:expr, $default:expr, $sue:ident, $count:expr, $( $f:ident ),*) => {
                match $to_match {
                    $(stringify!($f) => $sue.$f = $count,)*
                    _ => $default,
                }
            };
        }

        let (num, rest) = line.split_once(':').expect("Could not split on ':'");
        let num = num.trim_start_matches("Sue").trim().parse().unwrap();
        let mut sue = Self::new_p2(num);
        for field in rest.split(',') {
            let (field, count) = field.split_once(':').expect("Could not split on ':'");
            let count = count.trim().parse().unwrap();
            parse!(
                field.trim(),
                panic!("Unrecognized field {:?}", field.trim()),
                sue,
                count,
                children,
                cats,
                samoyeds,
                pomeranians,
                akitas,
                vizslas,
                goldfish,
                trees,
                cars,
                perfumes
            );
        }

        sue
    }

    fn real_eq(&self, searched: &Self) -> bool {
        macro_rules! false_expr {
            ($field:ident, $op:tt) => {
                if self.$field $op searched.$field {
                    return false;
                }
            };
        }

        false_expr!(children, !=);
        false_expr!(cats, <=);
        false_expr!(samoyeds, !=);
        false_expr!(pomeranians, >=);
        false_expr!(akitas, !=);
        false_expr!(vizslas, !=);
        false_expr!(goldfish, >=);
        false_expr!(trees, <=);
        false_expr!(cars, !=);
        false_expr!(perfumes, !=);

        true
    }
}

impl PartialEq for Sue {
    fn eq(&self, other: &Self) -> bool {
        self.children == other.children
            && self.cats == other.cats
            && self.samoyeds == other.samoyeds
            && self.pomeranians == other.pomeranians
            && self.akitas == other.akitas
            && self.vizslas == other.vizslas
            && self.goldfish == other.goldfish
            && self.trees == other.trees
            && self.cars == other.cars
            && self.perfumes == other.perfumes
    }
}

#[aoc_generator(day16, part1)]
fn parse_p1(input: &str) -> Vec<Sue> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Sue::parse_line_1)
        .collect()
}

#[aoc_generator(day16, part2)]
fn parse_p2(input: &str) -> Vec<Sue> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(Sue::parse_line_2)
        .collect()
}

const SEARCHING_FOR: Sue = Sue::searched();

#[aoc(day16, part1)]
fn part1(input: &[Sue]) -> u64 {
    input
        .iter()
        .find(|&sue| sue == &SEARCHING_FOR)
        .expect("Could not find a corresponding Aunt Sue")
        .num
}

#[aoc(day16, part2)]
fn part2(input: &[Sue]) -> u64 {
    input
        .iter()
        .find(|&sue| sue.real_eq(&SEARCHING_FOR))
        .expect("Could not find a corresponding Aunt Sue")
        .num
}

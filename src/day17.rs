use fnv::FnvHashMap;

#[aoc_generator(day17)]
fn parse(input: &str) -> Vec<u64> {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().parse().expect("Could not parse u64"))
        .collect()
}

#[aoc(day17, part1)]
fn part1(buckets: &[u64]) -> u64 {
    solve_p1_with_target(buckets, 150)
}

fn solve_p1_with_target(buckets: &[u64], target: u64) -> u64 {
    let mut cache = FnvHashMap::default();
    solve_p1_with_target_rec(buckets, 0, target, &mut cache)
}

fn solve_p1_with_target_rec(
    buckets: &[u64],
    bucket_idx: usize,
    target: u64,
    cache: &mut FnvHashMap<(u64, usize), u64>,
) -> u64 {
    if target == 0 {
        return 1;
    }

    if bucket_idx >= buckets.len() {
        return 0;
    }

    if let Some(cached) = cache.get(&(target, bucket_idx)) {
        return *cached;
    }

    let res = solve_p1_with_target_rec_impl(buckets, bucket_idx, target, cache);
    cache.insert((target, bucket_idx), res);
    res
}

fn solve_p1_with_target_rec_impl(
    buckets: &[u64],
    bucket_idx: usize,
    target: u64,
    cache: &mut FnvHashMap<(u64, usize), u64>,
) -> u64 {
    let res = solve_p1_with_target_rec(buckets, bucket_idx + 1, target, cache);
    if buckets[bucket_idx] > target {
        return res;
    }

    res + solve_p1_with_target_rec(buckets, bucket_idx + 1, target - buckets[bucket_idx], cache)
}

#[aoc(day17, part2)]
fn part2(buckets: &[u64]) -> u64 {
    solve_p2_with_target(buckets, 150).1
}

fn solve_p2_with_target(buckets: &[u64], target: u64) -> (usize, u64) {
    let mut cache = FnvHashMap::default();
    solve_p2_rec(buckets, 0, 0, target, &mut cache)
}

fn solve_p2_rec(
    buckets: &[u64],
    bucket_idx: usize,
    selected: usize,
    target: u64,
    cache: &mut FnvHashMap<(u64, usize, usize), (usize, u64)>,
) -> (usize, u64) {
    if target == 0 {
        return (selected, 1);
    }

    if bucket_idx >= buckets.len() {
        return (usize::MAX, 0);
    }

    if let Some(cached) = cache.get(&(target, bucket_idx, selected)) {
        return *cached;
    }

    let res = solve_p2_impl(buckets, bucket_idx, selected, target, cache);
    cache.insert((target, bucket_idx, selected), res);
    res
}

fn solve_p2_impl(
    buckets: &[u64],
    bucket_idx: usize,
    selected: usize,
    target: u64,
    cache: &mut FnvHashMap<(u64, usize, usize), (usize, u64)>,
) -> (usize, u64) {
    let res = solve_p2_rec(buckets, bucket_idx + 1, selected, target, cache);
    if buckets[bucket_idx] <= target {
        let res2 = solve_p2_rec(
            buckets,
            bucket_idx + 1,
            selected + 1,
            target - buckets[bucket_idx],
            cache,
        );
        if res.0 > res2.0 {
            return res2;
        }

        if res.0 == res2.0 {
            return (res.0, res.1 + res2.1);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_BUCKETS: [u64; 5] = [20, 15, 10, 5, 5];

    #[test]
    fn part1_example() {
        assert_eq!(solve_p1_with_target(&EXAMPLE_BUCKETS, 25), 4);
    }

    #[test]
    fn part2_example() {
        assert_eq!(solve_p2_with_target(&EXAMPLE_BUCKETS, 25), (2, 3));
    }
}

use serde_json::Value;

#[inline]
fn sum_all_nums(value: &Value) -> i64 {
    match value {
        Value::Number(num) => num.as_i64().unwrap(),
        Value::Array(array) => array.iter().map(sum_all_nums).sum(),
        Value::Object(obj) => obj.values().map(sum_all_nums).sum(),
        _ => 0,
    }
}

#[inline]
fn is_red(value: &Value) -> bool {
    match value {
        Value::String(s) => s == "red",
        _ => false,
    }
}

#[inline]
fn sum_ignore_red(value: &Value) -> i64 {
    match value {
        Value::Number(num) => num.as_i64().unwrap(),
        Value::Array(array) => array.iter().map(sum_ignore_red).sum(),
        Value::Object(obj) => {
            if obj.values().any(is_red) {
                0
            } else {
                obj.values().map(sum_ignore_red).sum()
            }
        },
        _ => 0,
    }
}

#[aoc_generator(day12)]
fn parse(input: &str) -> serde_json::Result<Value> {
    serde_json::from_str(input)
}

#[aoc(day12, part1)]
fn part1(value: &Value) -> i64 {
    sum_all_nums(value)
}

#[aoc(day12, part2)]
fn part2(value: &Value) -> i64 {
    sum_ignore_red(value)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn part1_examples() {
        assert_eq!(part1(&json!([1, 2, 3])), 6);
        assert_eq!(part1(&json!({"a": 2, "b": 4})), 6);
        assert_eq!(part1(&json!([[[3]]])), 3);
        assert_eq!(part1(&json!({"a": {"b": 4}, "c": -1})), 3);
        assert_eq!(part1(&json!({"a":[-1, 1]})), 0);
        assert_eq!(part1(&json!([-1, {"a": 1}])), 0);
        assert_eq!(part1(&json!([])), 0);
        assert_eq!(part1(&json!({})), 0);
    }

    #[test]
    fn part2_examples() {
        assert_eq!(part2(&json!([1, 2, 3])), 6);
        assert_eq!(part2(&json!([1, {"c": "red", "b": 2}, 3])), 4);
        assert_eq!(part2(&json!({"d": "red", "e": [1, 2, 3, 4], "f": 5})), 0);
        assert_eq!(part2(&json!([1, "red", 5])), 6);
    }
}

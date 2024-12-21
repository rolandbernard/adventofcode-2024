use std::collections::HashMap;

pub fn parse_input() -> Vec<String> {
    std::io::stdin().lines().map(|line| line.unwrap()).collect()
}

fn helper(
    upper: [i64; 2],
    from: [i64; 2],
    to: [i64; 2],
    level: i64,
    cache: &mut HashMap<([[i64; 2]; 3], i64), i64>,
) -> i64 {
    if level == 0 {
        1
    } else if from == to {
        helper([0, 2], upper, [0, 2], level - 1, cache)
    } else if let Some(&len) = cache.get(&([upper, from, to], level)) {
        len
    } else {
        let mut val = i64::MAX;
        for (relevant, button, [dy, dx]) in [
            (from[0] < to[0], [0, 1], [1, 0]),
            (from[0] > to[0], [-1, 1], [-1, 0]),
            (from[1] > to[1], [-1, 0], [0, -1]),
            (from[1] < to[1], [-1, 2], [0, 1]),
        ] {
            let next = [from[0] + dy, from[1] + dx];
            if relevant && next != [0, 0] {
                val = val.min(
                    helper([0, 2], upper, button, level - 1, cache)
                        .saturating_add(helper(button, next, to, level, cache)),
                );
            }
        }
        cache.insert(([upper, from, to], level), val);
        val
    }
}

pub fn complexities(line: &str, level: i64) -> i64 {
    let mut cache = HashMap::new();
    let mut cnt = 0;
    let mut last = [0, 2];
    for c in line.chars() {
        let next = match c.to_digit(11).unwrap() as i64 {
            0x0 => [0, 1],
            0xA => [0, 2],
            v => [1 + (v - 1) / 3, (v - 1) % 3],
        };
        cnt += helper([0, 2], last, next, level, &mut cache);
        last = next;
    }
    cnt * line
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<i64>()
        .unwrap()
}

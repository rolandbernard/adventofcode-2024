use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

fn num_possible<'a>(
    pattern: &'a str,
    base: &HashSet<&str>,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if pattern.is_empty() {
        1
    } else if cache.contains_key(pattern) {
        cache[pattern]
    } else {
        let mut sum = 0;
        for i in (1..=pattern.len()).rev() {
            if base.contains(&pattern[..i]) {
                sum += num_possible(&pattern[i..], base, cache);
            }
        }
        cache.insert(pattern, sum);
        sum
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (avail, targets) = input.split_once("\n\n").unwrap();
    let mut base = HashSet::new();
    for pattern in avail.split(", ") {
        base.insert(pattern);
    }
    let mut result = 0;
    let mut cache = HashMap::new();
    for pattern in targets.lines() {
        result += num_possible(pattern, &base, &mut cache);
    }
    println!("Result: {}", result);
}

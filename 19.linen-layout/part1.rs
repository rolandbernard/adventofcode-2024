use std::{collections::HashSet, io::Read};

fn is_possible<'a>(pattern: &'a str, cache: &mut HashSet<&'a str>) -> bool {
    if pattern.is_empty() {
        return true;
    }
    for i in (1..=pattern.len()).rev() {
        if cache.contains(&pattern[..i]) && is_possible(&pattern[i..], cache) {
            cache.insert(pattern);
            return true;
        }
    }
    false
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (avail, targets) = input.split_once("\n\n").unwrap();
    let mut cache = HashSet::new();
    for pattern in avail.split(", ") {
        cache.insert(pattern);
    }
    let mut result = 0;
    for pattern in targets.lines() {
        if is_possible(pattern, &mut cache) {
            result += 1;
        }
    }
    println!("Result: {}", result);
}

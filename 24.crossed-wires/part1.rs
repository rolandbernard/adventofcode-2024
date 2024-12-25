use std::{
    collections::HashMap,
    io::Read,
    ops::{BitAnd, BitOr, BitXor},
};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (values, gates) = input.split_once("\n\n").unwrap();
    let mut values = values
        .trim()
        .lines()
        .map(|l| l.trim().split_once(": ").unwrap())
        .map(|(n, v)| (n, v == "1"))
        .collect::<HashMap<_, _>>();
    let gates = gates
        .trim()
        .lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .map(|(gate, name)| {
            (
                gate.split_whitespace()
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
                name,
            )
        })
        .map(|([od1, op, od2], out): ([&str; 3], &str)| {
            (
                [od1, od2, out],
                match op {
                    "AND" => <bool as BitAnd>::bitand,
                    "OR" => <bool as BitOr>::bitor,
                    "XOR" => <bool as BitXor>::bitxor,
                    _ => unreachable!(),
                },
            )
        })
        .collect::<Vec<_>>();
    let mut used_in = HashMap::<_, Vec<_>>::new();
    for (i, ([od1, od2, _], _)) in gates.iter().enumerate() {
        used_in.entry(od1).or_default().push(i);
        used_in.entry(od2).or_default().push(i);
    }
    let mut queue = values.keys().copied().collect::<Vec<_>>();
    while let Some(var) = queue.pop() {
        if let Some(uses) = used_in.remove(&var) {
            for idx in uses {
                let ([a, b, c], f) = gates[idx];
                if values.contains_key(a) && values.contains_key(b) && !values.contains_key(c) {
                    values.insert(c, f(values[a], values[b]));
                    queue.push(c);
                }
            }
        }
    }
    let mut result = 0u128;
    for (name, value) in values {
        if value && name.starts_with('z') {
            let idx = name[1..].parse::<u32>().unwrap();
            eprintln!("{name}");
            result |= 1 << idx;
        }
    }
    println!("Result: {}", result);
}

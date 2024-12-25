use std::{
    collections::HashMap,
    io::Read,
    ops::{BitAnd, BitOr, BitXor},
};

type Circuit<'a> = Vec<([&'a str; 3], fn(bool, bool) -> bool)>;

fn run_circuit(gates: &Circuit, swaps: &HashMap<&str, &str>, x: u64, y: u64) -> u64 {
    let mut values = HashMap::new();
    for &([i0, i1, _], _) in gates {
        for i in [i0, i1] {
            if let Some(num) = i.strip_prefix('x') {
                let idx = num.parse::<u32>().unwrap();
                values.insert(i, ((x >> idx) & 1) == 1);
            } else if let Some(num) = i.strip_prefix('y') {
                let idx = num.parse::<u32>().unwrap();
                values.insert(i, ((y >> idx) & 1) == 1);
            }
        }
    }
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
                let c = swaps.get(&c).copied().unwrap_or(c);
                if values.contains_key(a) && values.contains_key(b) && !values.contains_key(c) {
                    values.insert(c, f(values[a], values[b]));
                    queue.push(c);
                }
            }
        }
    }
    let mut result = 0u64;
    for (name, value) in values {
        if let Some(num) = name.strip_prefix('z') {
            let idx = num.parse::<u32>().unwrap();
            result |= (value as u64) << idx;
        }
    }
    result
}

fn find_examples(gates: &Circuit, swaps: &HashMap<&str, &str>) -> Vec<[u64; 2]> {
    let mut state = 0u64;
    let mut examples = Vec::new();
    while examples.len() < 32 {
        let x = state % ((1 << 45) - 1);
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let y = state % ((1 << 45) - 1);
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let z = run_circuit(gates, swaps, x, y);
        if x + y != z {
            examples.push([x, y]);
        }
    }
    while examples.len() < 64 {
        let x = state % ((1 << 45) - 1);
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let y = state % ((1 << 45) - 1);
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let z = run_circuit(gates, swaps, x, y);
        if x + y == z {
            examples.push([x, y]);
        }
    }
    examples
}

fn is_good(gates: &Circuit, swaps: &HashMap<&str, &str>) -> bool {
    let mut state = 0u64;
    for _ in 0..10_000 {
        let x = state % ((1 << 45) - 1);
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let y = state % ((1 << 45) - 1);
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let z = run_circuit(gates, swaps, x, y);
        if x + y != z {
            return false;
        }
    }
    true
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (_, gates) = input.split_once("\n\n").unwrap();
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
    let mut last_best = HashMap::new();
    while !is_good(&gates, &last_best) {
        let examples = find_examples(&gates, &last_best);
        let mut best_swap = last_best.clone();
        let mut best_score = 0;
        for key in last_best.keys() {
            let mut mapping = last_best.clone();
            mapping.remove(key);
            mapping.remove(last_best[key]);
            let mut score = 0;
            for &[x, y] in &examples {
                score += (!(run_circuit(&gates, &mapping, x, y) ^ (x + y))).count_ones();
            }
            if score > best_score {
                eprintln!("new best score {score}");
                best_score = score;
                best_swap = mapping;
            }
        }
        last_best = best_swap;
        for _ in last_best.len()..4 {
            let examples = find_examples(&gates, &last_best);
            let mut best_swap = last_best.clone();
            let mut best_score = 0;
            for i in 0..gates.len() {
                eprintln!("{i}");
                for j in 0..gates.len() {
                    let mut mapping = last_best.clone();
                    mapping.insert(gates[i].0[2], gates[j].0[2]);
                    mapping.insert(gates[j].0[2], gates[i].0[2]);
                    let mut score = 0;
                    for &[x, y] in &examples {
                        score += (!(run_circuit(&gates, &mapping, x, y) ^ (x + y))).count_ones();
                    }
                    if score > best_score {
                        eprintln!("new best score {score}");
                        best_score = score;
                        best_swap = mapping;
                    }
                }
            }
            eprintln!("{best_swap:?}");
            last_best = best_swap;
        }
    }
    let mut result = last_best.keys().copied().collect::<Vec<_>>();
    result.sort();
    println!("Result: {}", result.join(","));
}

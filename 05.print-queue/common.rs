use std::{collections::HashMap, io::Read};

pub fn parse_input() -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).ok();
    let (rules_str, orders_str) = input.split_once("\n\n").unwrap();
    let rules = rules_str
        .lines()
        .map(|l| l.split_once('|').unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect();
    let orders = orders_str
        .lines()
        .map(|order_str| order_str.split(',').map(|p| p.parse().unwrap()).collect())
        .collect();
    (rules, orders)
}

pub fn is_ordered(rules: &[(i32, i32)], order: &[i32]) -> Result<(), HashMap<usize, Vec<usize>>> {
    let mut to_idx = HashMap::new();
    for (i, &n) in order.iter().enumerate() {
        to_idx.insert(n, i);
    }
    let mut arules = HashMap::new();
    let mut valid = true;
    for (l, r) in rules {
        if let (Some(&li), Some(&ri)) = (to_idx.get(l), to_idx.get(r)) {
            arules.entry(li).or_insert(Vec::new()).push(ri);
            if li > ri {
                valid = false;
            }
        }
    }
    if valid {
        Ok(())
    } else {
        Err(arules)
    }
}

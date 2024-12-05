use std::{collections::HashMap, io::Read};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).ok();
    let (rules_str, orders_str) = input.split_once("\n\n").unwrap();
    let rules = rules_str
        .lines()
        .map(|l| l.split_once('|').unwrap())
        .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
        .collect::<Vec<(i32, i32)>>();
    let mut result = 0;
    for order_str in orders_str.lines() {
        let order = order_str
            .split(',')
            .map(|p| p.parse().unwrap())
            .collect::<Vec<i32>>();
        let mut to_idx = HashMap::new();
        for (i, &n) in order.iter().enumerate() {
            to_idx.insert(n, i);
        }
        let mut valid = true;
        for (l, r) in &rules {
            if let (Some(li), Some(ri)) = (to_idx.get(l), to_idx.get(r)) {
                if li > ri {
                    valid = false;
                }
            }
        }
        if valid {
            assert!(order.len() % 2 == 1);
            result += order[(order.len() - 1) / 2];
        }
    }
    println!("Result: {}", result);
}

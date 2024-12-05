use std::{
    collections::{HashMap, VecDeque},
    io::Read,
};

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
        assert!(order.len() % 2 == 1);
        let mut to_idx = HashMap::new();
        for (i, &n) in order.iter().enumerate() {
            to_idx.insert(n, i);
        }
        let mut arules = HashMap::new();
        let mut valid = true;
        for (l, r) in &rules {
            if let (Some(&li), Some(&ri)) = (to_idx.get(l), to_idx.get(r)) {
                arules.entry(li).or_insert(Vec::new()).push(ri);
                if li > ri {
                    valid = false;
                }
            }
        }
        if !valid {
            let mut counts = vec![0; order.len()];
            for ris in arules.values() {
                for &ri in ris {
                    counts[ri] += 1;
                }
            }
            let mut queue = counts
                .iter()
                .enumerate()
                .filter(|(_, &c)| c == 0)
                .map(|(i, _)| i)
                .collect::<VecDeque<_>>();
            let mut correct = Vec::new();
            while let Some(li) = queue.pop_front() {
                assert!(queue.is_empty());
                correct.push(order[li]);
                if let Some(ris) = arules.get(&li) {
                    for &ri in ris {
                        counts[ri] -= 1;
                        if counts[ri] == 0 {
                            queue.push_back(ri);
                        }
                    }
                }
            }
            assert_eq!(order.len(), correct.len());
            result += correct[(correct.len() - 1) / 2];
        }
    }
    println!("Result: {}", result);
}

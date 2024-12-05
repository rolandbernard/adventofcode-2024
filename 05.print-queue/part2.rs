use std::collections::VecDeque;

mod common;

fn main() {
    let (rules, orders) = common::parse_input();
    let mut result = 0;
    for order in orders {
        if let Err(arules) = common::is_ordered(&rules, &order) {
            let mut counts = vec![0; order.len()];
            for ris in arules.values() {
                for &ri in ris {
                    counts[ri] += 1;
                }
            }
            let mut queue = (0..counts.len())
                .filter(|&i| counts[i] == 0)
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
            assert!(order.len() % 2 == 1);
            assert_eq!(order.len(), correct.len());
            result += correct[(correct.len() - 1) / 2];
        }
    }
    println!("Result: {}", result);
}

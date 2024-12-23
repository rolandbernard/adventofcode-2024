use std::collections::HashSet;

mod common;

fn main() {
    let input = common::read_input();
    let adj = common::parse_input(&input);
    let mut party = adj.keys().map(|&e| vec![e]).collect::<HashSet<_>>();
    while party.len() > 1 {
        party = common::enlarge_cliques(&adj, &party);
    }
    assert!(party.len() == 1);
    let party = party.into_iter().next().unwrap();
    println!("Result: {}", party.join(","));
}

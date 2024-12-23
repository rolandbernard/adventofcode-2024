mod common;

fn main() {
    let input = common::read_input();
    let adj = common::parse_input(&input);
    let party = adj
        .keys()
        .filter(|e| e.starts_with('t'))
        .map(|&e| vec![e])
        .collect();
    let party = common::enlarge_cliques(&adj, &party);
    let party = common::enlarge_cliques(&adj, &party);
    println!("Result: {}", party.len());
}

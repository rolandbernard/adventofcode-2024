use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

pub fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    input
}

pub fn parse_input(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut adj = HashMap::<_, HashSet<_>>::new();
    for line in input.lines() {
        let (from, to) = line.split_once('-').unwrap();
        adj.entry(from).or_default().insert(to);
        adj.entry(to).or_default().insert(from);
    }
    adj
}

pub fn enlarge_cliques<'a>(
    adj: &HashMap<&'a str, HashSet<&'a str>>,
    prev: &HashSet<Vec<&'a str>>,
) -> HashSet<Vec<&'a str>> {
    let mut next = HashSet::new();
    for elem in prev {
        for &n in &adj[elem[0]] {
            let neigh = &adj[n];
            if elem.iter().all(|e| neigh.contains(e)) {
                let mut new = elem.to_owned();
                new.push(n);
                new.sort();
                next.insert(new);
            }
        }
    }
    next
}

use std::collections::{HashMap, HashSet};

fn main() {
    let mut cons = HashMap::<_, HashSet<_>>::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (from, to) = line.split_once('-').unwrap();
        cons.entry(from.to_owned())
            .or_default()
            .insert(to.to_owned());
        cons.entry(to.to_owned())
            .or_default()
            .insert(from.to_owned());
    }
    let mut result = 0;
    for (n0, cs) in &cons {
        if n0.starts_with('t') {
            for n1 in cs {
                for n2 in cs {
                    if n1 < n2
                        && (n0 < n1 || !n1.starts_with('t'))
                        && (n0 < n2 || !n2.starts_with('t'))
                        && cons[n1].contains(n2)
                    {
                        eprintln!("{n0},{n1},{n2}");
                        result += 1;
                    }
                }
            }
        }
    }
    println!("Result: {}", result);
}

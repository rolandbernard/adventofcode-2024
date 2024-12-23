use std::{
    collections::{HashMap, HashSet},
    io::Read,
};

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut cons = HashSet::new();
    for line in input.lines() {
        let (from, to) = line.split_once('-').unwrap();
        cons.insert([from.min(to), to.max(from)]);
    }
    let mut by_first = HashMap::<_, HashSet<_>>::new();
    let mut by_last = HashMap::<_, HashSet<_>>::new();
    for &[a, b] in &cons {
        by_first.entry(a).or_default().insert(Vec::from([a, b]));
        by_last.entry(b).or_default().insert(Vec::from([a, b]));
    }
    for i in 2.. {
        eprintln!("{i}");
        let mut new_first = HashMap::<_, HashSet<_>>::new();
        let mut new_last = HashMap::<_, HashSet<_>>::new();
        for (_, ps) in by_first.iter().chain(by_last.iter()) {
            for a in ps {
                for b in ps {
                    let mut ok = true;
                    let mut na = "";
                    let mut nb = "";
                    let mut i = 0;
                    let mut j = 0;
                    while i < a.len() || j < b.len() {
                        if i < a.len() && j < b.len() && a[i] == b[j] {
                            i += 1;
                            j += 1;
                        } else if j == b.len() || (i < a.len() && a[i] < b[j]) {
                            if na.is_empty() {
                                na = a[i];
                                i += 1;
                            } else {
                                ok = false;
                                break;
                            }
                        } else if nb.is_empty() {
                            nb = b[j];
                            j += 1;
                        } else {
                            ok = false;
                            break;
                        }
                    }
                    if ok && cons.contains(&[na.min(nb), nb.max(na)]) {
                        let mut new = a.clone();
                        new.push(nb);
                        new.sort();
                        new_first.entry(new[0]).or_default().insert(new.clone());
                        new_last.entry(new[new.len() - 1]).or_default().insert(new);
                    }
                }
            }
        }
        if new_first.is_empty() {
            break;
        }
        by_first = new_first;
        by_last = new_last;
    }
    let party = by_first
        .into_values()
        .next()
        .unwrap()
        .into_iter()
        .next()
        .unwrap();
    println!("Result: {}", party.join(","));
}

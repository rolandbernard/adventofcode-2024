use std::{
    collections::{HashMap, HashSet},
    ops::AddAssign,
};

fn next_number(mut x: i64) -> i64 {
    x ^= x * 64;
    x %= 16777216;
    x ^= x / 32;
    x %= 16777216;
    x ^= x * 2048;
    x %= 16777216;
    x
}

fn main() {
    let seeds = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().parse::<i64>().unwrap());
    let mut price = HashMap::<_, i64>::new();
    for mut x in seeds {
        let mut seen = HashSet::new();
        let mut change = [0, 0, 0, 0];
        let mut last = x % 10;
        for i in 0..2000 {
            x = next_number(x);
            change.copy_within(1.., 0);
            change[3] = (x % 10) - last;
            if i >= 3 && !seen.contains(&change) {
                seen.insert(change);
                price.entry(change).or_default().add_assign(x % 10);
            }
            last = x % 10;
        }
    }
    let result = price.values().max().unwrap();
    println!("Result: {result:?}");
}

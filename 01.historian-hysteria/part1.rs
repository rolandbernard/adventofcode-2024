fn main() {
    let mut lists = [Vec::new(), Vec::new()];
    for line in std::io::stdin().lines() {
        for (i, num) in line
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse::<i64>().unwrap())
            .enumerate()
        {
            lists[i].push(num);
        }
    }
    lists[0].sort();
    lists[1].sort();
    let result = lists[0]
        .iter()
        .zip(lists[1].iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i64>();
    println!("Result: {}", result);
}

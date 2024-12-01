pub fn parse_input() -> [Vec<i64>; 2] {
    let mut lists = [Vec::new(), Vec::new()];
    for line in std::io::stdin().lines() {
        for (i, num) in line
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .enumerate()
        {
            lists[i].push(num);
        }
    }
    lists[0].sort();
    lists[1].sort();
    lists
}

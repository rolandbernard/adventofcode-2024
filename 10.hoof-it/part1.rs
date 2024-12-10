fn main() {
    let map: Vec<Vec<_>> = std::io::stdin()
        .lines()
        .map(|line| {
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect();
    let mut result = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == 0 {
                let mut reachable = vec![vec![false; map[0].len()]; map.len()];
                let mut queue = vec![(i, j, 0)];
                reachable[i][j] = true;
                while let Some((i, j, h)) = queue.pop() {
                    for (di, dj) in [(1, 0), (usize::MAX, 0), (0, 1), (0, usize::MAX)] {
                        let ni = i.wrapping_add(di);
                        let nj = j.wrapping_add(dj);
                        if map.get(ni).and_then(|r| r.get(nj)) == Some(&(h + 1))
                            && !reachable[ni][nj]
                        {
                            reachable[ni][nj] = true;
                            if h + 1 == 9 {
                                result += 1;
                            } else {
                                queue.push((ni, nj, h + 1));
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Result: {}", result);
}

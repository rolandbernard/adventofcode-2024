use std::collections::VecDeque;

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
                let mut comb = vec![vec![0; map[0].len()]; map.len()];
                let mut queue = VecDeque::new();
                queue.push_back((i, j, 0));
                comb[i][j] = 1;
                while let Some((i, j, h)) = queue.pop_front() {
                    for (di, dj) in [(1, 0), (usize::MAX, 0), (0, 1), (0, usize::MAX)] {
                        let ni = i.wrapping_add(di);
                        let nj = j.wrapping_add(dj);
                        if map.get(ni).and_then(|r| r.get(nj)) == Some(&(h + 1)) {
                            if h + 1 == 9 {
                                result += comb[i][j];
                            } else {
                                if comb[ni][nj] == 0 {
                                    queue.push_back((ni, nj, h + 1));
                                }
                                comb[ni][nj] += comb[i][j];
                            }
                        }
                    }
                }
            }
        }
    }
    println!("Result: {}", result);
}

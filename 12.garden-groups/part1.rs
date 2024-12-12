fn main() {
    let map: Vec<Vec<_>> = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let mut counted = vec![vec![false; map[0].len()]; map.len()];
    let mut result = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if !counted[i][j] {
                let mut area = 1;
                let mut perim = 0;
                counted[i][j] = true;
                let mut queue = Vec::new();
                queue.push((i, j));
                while let Some((i, j)) = queue.pop() {
                    for (di, dj) in [(1, 0), (usize::MAX, 0), (0, 1), (0, usize::MAX)] {
                        let ni = i.wrapping_add(di);
                        let nj = j.wrapping_add(dj);
                        if map.get(ni).and_then(|r| r.get(nj)) == Some(cell) {
                            if !counted[ni][nj] {
                                area += 1;
                                counted[ni][nj] = true;
                                queue.push((ni, nj));
                            }
                        } else {
                            perim += 1;
                        }
                    }
                }
                result += area * perim;
            }
        }
    }
    println!("Result: {}", result);
}

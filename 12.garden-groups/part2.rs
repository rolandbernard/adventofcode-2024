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
                let mut perims = vec![Vec::new(); 4];
                counted[i][j] = true;
                let mut queue = Vec::new();
                queue.push((i, j));
                while let Some((i, j)) = queue.pop() {
                    for (d, &(di, dj)) in [(1, 0), (usize::MAX, 0), (0, 1), (0, usize::MAX)]
                        .iter()
                        .enumerate()
                    {
                        let ni = i.wrapping_add(di);
                        let nj = j.wrapping_add(dj);
                        if map.get(ni).and_then(|r| r.get(nj)) == Some(cell) {
                            if !counted[ni][nj] {
                                area += 1;
                                counted[ni][nj] = true;
                                queue.push((ni, nj));
                            }
                        } else {
                            perims[d].push(if d <= 1 { (i, j) } else { (j, i) });
                        }
                    }
                }
                let mut perim = 0;
                for mut p in perims {
                    p.sort();
                    perim += 1;
                    let mut last = p[0];
                    for &next in &p[1..] {
                        if last.0.abs_diff(next.0) + last.1.abs_diff(next.1) != 1 {
                            perim += 1;
                        }
                        last = next;
                    }
                }
                result += area * perim;
            }
        }
    }
    println!("Result: {}", result);
}

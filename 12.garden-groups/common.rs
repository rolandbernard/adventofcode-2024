pub fn parse_input() -> Vec<Vec<char>> {
    std::io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

pub fn shapes_iter(
    map: &[Vec<char>],
) -> impl '_ + Iterator<Item = (usize, Vec<Vec<(usize, usize)>>)> {
    let mut counted = vec![vec![false; map[0].len()]; map.len()];
    map.iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &cell)| (i, j, cell)))
        .filter_map(move |(i, j, cell)| {
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
                        if map.get(ni).and_then(|r| r.get(nj)) == Some(&cell) {
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
                Some((area, perims))
            } else {
                None
            }
        })
}

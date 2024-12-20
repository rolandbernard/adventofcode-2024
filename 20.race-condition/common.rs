use std::collections::VecDeque;

pub fn parse_input() -> (Vec<Vec<char>>, [usize; 2]) {
    let map = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect::<Vec<Vec<_>>>();
    let (start, _) = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &cell)| ([i, j], cell)))
        .find(|&(_, c)| c == 'S')
        .unwrap();
    (map, start)
}

pub fn count_cheats(map: &[Vec<char>], start: [usize; 2], max: usize, min: usize) -> usize {
    let max = max as isize;
    let mut count = 0;
    let mut dist = vec![vec![usize::MAX; map[0].len()]; map.len()];
    let mut queue = VecDeque::new();
    dist[start[0]][start[1]] = 0;
    queue.push_back((start, 0));
    while let Some(([i, j], d)) = queue.pop_front() {
        for di in -max..=max {
            for dj in -max + di.abs()..=max - di.abs() {
                if di != 0 || dj != 0 {
                    let ni = i.wrapping_add(di as usize);
                    let nj = j.wrapping_add(dj as usize);
                    let d = (di.abs() + dj.abs()) as usize;
                    if let Some('.' | 'S' | 'E') = map.get(ni).and_then(|r| r.get(nj)) {
                        if dist[ni][nj] != usize::MAX && dist[ni][nj] + d + min <= dist[i][j] {
                            count += 1;
                        }
                    }
                }
            }
        }
        for [ni, nj] in [[i + 1, j], [i - 1, j], [i, j + 1], [i, j - 1]] {
            if map[ni][nj] != '#' && d + 1 < dist[ni][nj] {
                dist[ni][nj] = d + 1;
                queue.push_back(([ni, nj], d + 1));
            }
        }
    }
    count
}

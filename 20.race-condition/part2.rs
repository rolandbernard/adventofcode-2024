use std::{
    collections::{HashMap, VecDeque},
    ops::AddAssign,
};

fn main() {
    let map = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect::<Vec<Vec<_>>>();
    let cells = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &cell)| ([i, j], cell)));
    let (start, _) = cells.clone().find(|&(_, c)| c == 'S').unwrap();
    let (end, _) = cells.clone().find(|&(_, c)| c == 'E').unwrap();
    let mut dist = vec![vec![usize::MAX; map[0].len()]; map.len()];
    let mut queue = VecDeque::new();
    dist[start[0]][start[1]] = 0;
    queue.push_back((start, 0));
    while let Some(([i, j], d)) = queue.pop_front() {
        for [ni, nj] in [[i + 1, j], [i - 1, j], [i, j + 1], [i, j - 1]] {
            if map[ni][nj] != '#' && d + 1 < dist[ni][nj] {
                dist[ni][nj] = d + 1;
                queue.push_back(([ni, nj], d + 1));
            }
        }
    }
    let mut hist = HashMap::<usize, usize>::new();
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if dist[i][j] != usize::MAX {
                for di in -20isize..=20 {
                    for dj in -20 + di.abs()..=20 - di.abs() {
                        if di != 0 || dj != 0 {
                            let ni = i.wrapping_add(di as usize);
                            let nj = j.wrapping_add(dj as usize);
                            let d = (di.abs() + dj.abs()) as usize;
                            if let Some('.' | 'S' | 'E') = map.get(ni).and_then(|r| r.get(nj)) {
                                if dist[ni][nj] != usize::MAX && dist[ni][nj] > dist[i][j] + d {
                                    hist.entry(dist[ni][nj] - dist[i][j] - d)
                                        .or_default()
                                        .add_assign(1);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let result = hist
        .into_iter()
        .filter_map(|(k, v)| if k >= 100 { Some(v) } else { None })
        .sum::<usize>();
    println!("Result: {result}");
}

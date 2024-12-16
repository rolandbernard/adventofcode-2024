use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    let mut map = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect::<Vec<Vec<_>>>();
    let cells = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &cell)| ([i, j], cell)));
    let (start, _) = cells.clone().find(|&(_, c)| c == 'S').unwrap();
    let (end, _) = cells.clone().find(|&(_, c)| c == 'E').unwrap();
    let mut dist = vec![vec![[u64::MAX; 4]; map[0].len()]; map.len()];
    let mut prev = vec![vec![vec![Vec::new(); 4]; map[0].len()]; map.len()];
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), start, 0));
    dist[start[0]][start[1]][0] = 0;
    while let Some((Reverse(d), pos, dir)) = queue.pop() {
        for (npos, ndir, nd) in [
            (pos, (dir + 3) % 4, d + 1000),
            (pos, (dir + 1) % 4, d + 1000),
            (
                [
                    pos[0].wrapping_add([0, 1, 0, usize::MAX][dir]),
                    pos[1].wrapping_add([1, 0, usize::MAX, 0][dir]),
                ],
                dir,
                d + 1,
            ),
        ] {
            if map[npos[0]][npos[1]] != '#' && nd <= dist[npos[0]][npos[1]][ndir] {
                if nd < dist[npos[0]][npos[1]][ndir] {
                    prev[npos[0]][npos[1]][ndir].clear();
                }
                prev[npos[0]][npos[1]][ndir].push((pos, dir));
                dist[npos[0]][npos[1]][ndir] = nd;
                queue.push((Reverse(nd), npos, ndir));
            }
        }
    }
    let mut seen = vec![vec![[false; 4]; map[0].len()]; map.len()];
    let mut stack = Vec::new();
    let min_dist = *dist[end[0]][end[1]].iter().min().unwrap();
    for d in 0..4 {
        if dist[end[0]][end[1]][d] == min_dist {
            stack.push((end, d));
            seen[end[0]][end[1]][d] = true;
        }
    }
    let mut result = 1;
    while let Some((pos, dir)) = stack.pop() {
        for &(ppos, pdir) in &prev[pos[0]][pos[1]][dir] {
            if !seen[ppos[0]][ppos[1]][pdir] {
                if seen[ppos[0]][ppos[1]].iter().all(|e| !e) {
                    result += 1;
                    map[ppos[0]][ppos[1]] = 'O';
                }
                seen[ppos[0]][ppos[1]][pdir] = true;
                stack.push((ppos, pdir));
            }
        }
    }
    println!("Result: {}", result);
}

use std::{cmp::Reverse, collections::BinaryHeap};

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
    let mut dist = vec![vec![[u64::MAX; 4]; map[0].len()]; map.len()];
    let mut queue = BinaryHeap::new();
    queue.push((Reverse(0), start, 0));
    dist[start[0]][start[1]][0] = 0;
    while let Some((Reverse(d), pos, dir)) = queue.pop() {
        if pos == end {
            break;
        }
        for (pos, dir, d) in [
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
            if map[pos[0]][pos[1]] != '#' && d < dist[pos[0]][pos[1]][dir] {
                dist[pos[0]][pos[1]][dir] = d;
                queue.push((Reverse(d), pos, dir));
            }
        }
    }
    println!("Result: {}", dist[end[0]][end[1]].iter().min().unwrap());
}

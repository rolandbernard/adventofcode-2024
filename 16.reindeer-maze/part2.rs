mod common;

fn main() {
    let (map, start, end) = common::parse_input();
    let dist = common::shortest_paths(&map, start);
    let mut seen = vec![vec![[false; 4]; map[0].len()]; map.len()];
    let mut stack = Vec::new();
    let min_dist = *dist[end[0]][end[1]].iter().min().unwrap();
    for d in 0..4 {
        if dist[end[0]][end[1]][d] == min_dist {
            stack.push((end, d, 0));
            seen[end[0]][end[1]][d] = true;
        }
    }
    while let Some((pos, dir, d)) = stack.pop() {
        for (ppos, pdir, pd) in [
            (pos, (dir + 3) % 4, d + 1000),
            (pos, (dir + 1) % 4, d + 1000),
            (
                [
                    pos[0].wrapping_add([0, usize::MAX, 0, 1][dir]),
                    pos[1].wrapping_add([usize::MAX, 0, 1, 0][dir]),
                ],
                dir,
                d + 1,
            ),
        ] {
            if dist[ppos[0]][ppos[1]][pdir].saturating_add(pd) == min_dist
                && !seen[ppos[0]][ppos[1]][pdir]
            {
                seen[ppos[0]][ppos[1]][pdir] = true;
                stack.push((ppos, pdir, pd));
            }
        }
    }
    let result = seen
        .iter()
        .map(|row| row.iter().filter(|c| c.iter().any(|&e| e)).count())
        .sum::<usize>();
    println!("Result: {}", result);
}

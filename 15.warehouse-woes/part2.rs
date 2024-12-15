use std::mem;

use common::sum_of_gps;

mod common;

fn main() {
    let (mut map, instr, [mut i, mut j]) = common::parse_input();
    for row in &mut map {
        let old = mem::take(row);
        for c in old {
            match c {
                'O' => row.extend(['[', ']']),
                '@' => row.extend(['@', '.']),
                _ => row.extend([c, c]),
            }
        }
    }
    j *= 2;
    for [di, dj] in instr {
        let (ni, nj) = (i.wrapping_add(di), j.wrapping_add(dj));
        let mut seen = vec![vec![false; map[0].len()]; map.len()];
        let mut moving = Vec::new();
        let mut pushed = Vec::new();
        let mut blocked = false;
        pushed.push((ni, nj));
        seen[ni][nj] = true;
        while let Some((i, j)) = pushed.pop() {
            let (ni, nj) = (i.wrapping_add(di), j.wrapping_add(dj));
            if map[i][j] == '#' {
                blocked = true;
                break;
            } else if map[i][j] != '.' {
                moving.push((i, j, ni, nj, map[i][j]));
                if !seen[ni][nj] {
                    seen[ni][nj] = true;
                    pushed.push((ni, nj));
                }
                if map[i][j] == '[' && !seen[i][j + 1] {
                    seen[ni][nj] = true;
                    pushed.push((i, j + 1));
                }
                if map[i][j] == ']' && !seen[i][j - 1] {
                    seen[ni][nj] = true;
                    pushed.push((i, j - 1));
                }
            }
        }
        if !blocked {
            for &(i, j, _, _, _) in &moving {
                map[i][j] = '.';
            }
            map[i][j] = '.';
            for &(_, _, ni, nj, c) in &moving {
                map[ni][nj] = c;
            }
            map[ni][nj] = '@';
            (i, j) = (ni, nj);
        }
    }
    println!("Result: {}", sum_of_gps(&map));
}

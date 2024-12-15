use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (map, instr) = input.split_once("\n\n").unwrap();
    let mut map = map
        .lines()
        .map(|l| {
            l.chars()
                .flat_map(|c| match c {
                    'O' => ['[', ']'],
                    '@' => ['@', '.'],
                    _ => [c, c],
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>();
    let (mut i, mut j, _) = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &cell)| (i, j, cell)))
        .find(|&(_, _, cell)| cell == '@')
        .unwrap();
    for instr in instr.chars().filter(|c| "<^v>".contains(*c)) {
        let (di, dj) = match instr {
            'v' => (1, 0),
            '^' => (usize::MAX, 0),
            '>' => (0, 1),
            '<' => (0, usize::MAX),
            _ => unreachable!(),
        };
        let (ni, nj) = (i.wrapping_add(di), j.wrapping_add(dj));
        let mut to_push = Vec::new();
        let mut to_check = Vec::new();
        let mut checked = vec![vec![false; map[0].len()]; map.len()];
        let mut blocked = false;
        to_check.push((ni, nj));
        checked[ni][nj] = true;
        while let Some((i, j)) = to_check.pop() {
            let (ni, nj) = (i.wrapping_add(di), j.wrapping_add(dj));
            if map[i][j] == '#' {
                blocked = true;
                break;
            } else if map[i][j] != '.' {
                to_push.push((i, j, ni, nj, map[i][j]));
                if !checked[ni][nj] {
                    to_check.push((ni, nj));
                    checked[ni][nj] = true;
                }
                if map[i][j] == '[' && !checked[i][j + 1] {
                    to_check.push((i, j + 1));
                    checked[ni][nj] = true;
                }
                if map[i][j] == ']' && !checked[i][j - 1] {
                    to_check.push((i, j - 1));
                    checked[ni][nj] = true;
                }
            }
        }
        if !blocked {
            for &(i, j, _, _, _) in &to_push {
                map[i][j] = '.';
            }
            map[i][j] = '.';
            for &(_, _, ni, nj, c) in &to_push {
                map[ni][nj] = c;
            }
            map[ni][nj] = '@';
            (i, j) = (ni, nj);
        }
    }
    let mut result = 0;
    for (i, row) in map.into_iter().enumerate() {
        for (j, cell) in row.into_iter().enumerate() {
            if cell == '[' {
                result += 100 * i + j;
            }
        }
    }
    println!("Result: {result}");
}

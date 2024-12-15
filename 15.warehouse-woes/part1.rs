use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (map, instr) = input.split_once("\n\n").unwrap();
    let mut map = map
        .lines()
        .map(|l| l.chars().collect())
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
        let (mut nni, mut nnj) = (ni, nj);
        while map[nni][nnj] == 'O' {
            (nni, nnj) = (nni.wrapping_add(di), nnj.wrapping_add(dj));
        }
        if map[nni][nnj] == '.' {
            map[i][j] = '.';
            map[nni][nnj] = 'O';
            map[ni][nj] = '@';
            (i, j) = (ni, nj);
        }
    }
    let mut result = 0;
    for (i, row) in map.into_iter().enumerate() {
        for (j, cell) in row.into_iter().enumerate() {
            if cell == 'O' {
                result += 100 * i + j;
            }
        }
    }
    println!("Result: {result}");
}

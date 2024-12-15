use std::io::Read;

pub fn parse_input() -> (Vec<Vec<char>>, Vec<[usize; 2]>, [usize; 2]) {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (map, instr) = input.split_once("\n\n").unwrap();
    let map = map
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<_>>>();
    let (pos, _) = map
        .iter()
        .enumerate()
        .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, &cell)| ([i, j], cell)))
        .find(|&(_, cell)| cell == '@')
        .unwrap();
    let instr = instr
        .chars()
        .filter(|c| "<^v>".contains(*c))
        .map(|c| match c {
            'v' => [1, 0],
            '^' => [usize::MAX, 0],
            '>' => [0, 1],
            '<' => [0, usize::MAX],
            _ => unreachable!(),
        })
        .collect();
    (map, instr, pos)
}

pub fn sum_of_gps(map: &[Vec<char>]) -> usize {
    let mut result = 0;
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == 'O' || cell == '[' {
                result += 100 * i + j;
            }
        }
    }
    result
}

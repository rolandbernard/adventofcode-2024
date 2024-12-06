pub fn parse_input() -> (Vec<Vec<char>>, (i32, i32)) {
    let map: Vec<Vec<char>> = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let mut pos = (0, 0);
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '^' {
                pos = (i as i32, j as i32);
            }
        }
    }
    (map, pos)
}

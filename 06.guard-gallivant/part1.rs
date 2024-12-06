fn main() {
    let map: Vec<Vec<char>> = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let mut visited = vec![vec![0; map[0].len()]; map.len()];
    let mut pos = (0, 0);
    let mut dir = (-1, 0);
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell == '^' {
                pos = (i as i32, j as i32);
            }
        }
    }
    while pos.0 > 0 && pos.1 > 0 && pos.0 < map.len() as i32 && pos.1 < map[0].len() as i32 {
        visited[pos.0 as usize][pos.1 as usize] = 1;
        let mut new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        while map
            .get(new_pos.0 as usize)
            .and_then(|row| row.get(new_pos.1 as usize))
            == Some(&'#')
        {
            dir = (dir.1, -dir.0);
            new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        }
        pos = new_pos;
    }
    println!("Result: {}", visited.iter().flatten().sum::<i32>());
}

fn is_looping(map: &[Vec<char>], mut pos: (i32, i32)) -> bool {
    let mut visited = vec![vec![vec![false; 9]; map[0].len()]; map.len()];
    let mut dir = (-1, 0);
    while pos.0 > 0 && pos.1 > 0 && pos.0 < map.len() as i32 && pos.1 < map[0].len() as i32 {
        let visited_entry =
            &mut visited[pos.0 as usize][pos.1 as usize][((dir.0 + 1) + 3 * (dir.1 + 1)) as usize];
        if *visited_entry {
            return true;
        }
        *visited_entry = true;
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
    false
}

fn main() {
    let mut map: Vec<Vec<char>> = std::io::stdin()
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
    assert!(!is_looping(&map, pos));
    let mut result = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            if map[i][j] == '.' {
                map[i][j] = '#';
                if is_looping(&map, pos) {
                    result += 1;
                }
                map[i][j] = '.';
            }
        }
    }
    println!("Result: {}", result);
}

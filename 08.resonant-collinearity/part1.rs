fn main() {
    let map: Vec<Vec<char>> = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let mut antenna = Vec::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell != '.' {
                antenna.push((i as i64, j as i64, cell));
            }
        }
    }
    let mut result = 0;
    for i in 0..map.len() as i64 {
        for j in 0..map[i as usize].len() as i64 {
            let mut has_antinode = false;
            for &(ai, aj, f) in &antenna {
                let ni = 2 * ai - i;
                let nj = 2 * aj - j;
                if (i != ai || j != aj)
                    && map.get(ni as usize).and_then(|r| r.get(nj as usize)) == Some(&f)
                {
                    has_antinode = true;
                    break;
                }
            }
            if has_antinode {
                result += 1;
            }
        }
    }
    println!("Result: {}", result);
}

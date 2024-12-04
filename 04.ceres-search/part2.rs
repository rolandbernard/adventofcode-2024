fn main() {
    let map: Vec<Vec<char>> = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let mut num_xmas = 0;
    for i in 0..map.len() as i64 {
        for j in 0..map[i as usize].len() as i64 {
            for (di, dj) in [(1, 1), (-1, -1), (1, -1), (-1, 1)] {
                if "MAS"
                    .chars()
                    .enumerate()
                    .map(|(k, c)| (k as i64 - 1, c))
                    .all(|(k, c)| {
                        map.get((i + k * di) as usize)
                            .and_then(|row| row.get((j + k * dj) as usize))
                            == Some(&c)
                            && map
                                .get((i + k * dj) as usize)
                                .and_then(|row| row.get((j - k * di) as usize))
                                == Some(&c)
                    })
                {
                    num_xmas += 1;
                }
            }
        }
    }
    println!("Result: {}", num_xmas);
}

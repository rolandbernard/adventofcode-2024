use std::collections::HashMap;

pub fn parse_input() -> Vec<Vec<char>> {
    std::io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

pub fn count_antinodes(map: &[Vec<char>], harmonics: impl IntoIterator<Item = i64> + Clone) -> i64 {
    let mut antennas = HashMap::new();
    for (i, row) in map.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell != '.' {
                antennas
                    .entry(cell)
                    .or_insert(Vec::new())
                    .push((i as i64, j as i64));
            }
        }
    }
    let mut antinodes = vec![vec![0; map[0].len()]; map.len()];
    for antenna in antennas.values() {
        for (i1, j1) in antenna {
            for (i2, j2) in antenna {
                if i1 != i2 || j1 != j2 {
                    for r in harmonics.clone() {
                        let ni = i2 + r * (i2 - i1);
                        let nj = j2 + r * (j2 - j1);
                        if ni >= 0 && nj >= 0 && ni < map.len() as i64 && nj < map[0].len() as i64 {
                            antinodes[ni as usize][nj as usize] = 1;
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    antinodes.into_iter().flatten().sum::<i64>()
}

pub fn parse_input() -> Vec<Vec<char>> {
    std::io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

pub fn match_word(
    map: &[Vec<char>],
    word: &str,
    i: usize,
    j: usize,
    di: i64,
    dj: i64,
    p: usize,
) -> bool {
    word.chars()
        .enumerate()
        .map(|(k, c)| (k as i64 - p as i64, c))
        .all(|(k, c)| {
            map.get((i as i64 + k * di) as usize)
                .and_then(|row| row.get((j as i64 + k * dj) as usize))
                == Some(&c)
        })
}

fn main() {
    let map: Vec<Vec<char>> = std::io::stdin()
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect();
    let mut num_xmas = 0;
    for i in 0..map.len() as i64 {
        for j in 0..map[i as usize].len() as i64 {
            for (di, dj) in [-1, 0, 1].iter().flat_map(|&di| {
                [-1, 0, 1]
                    .iter()
                    .map(move |&dj| (di, dj))
                    .filter(|&(di, dj)| di != 0 || dj != 0)
            }) {
                if "XMAS".chars().enumerate().all(|(k, c)| {
                    map.get((i + k as i64 * di) as usize)
                        .and_then(|row| row.get((j + k as i64 * dj) as usize))
                        == Some(&c)
                }) {
                    num_xmas += 1;
                }
            }
        }
    }
    println!("Result: {}", num_xmas);
}

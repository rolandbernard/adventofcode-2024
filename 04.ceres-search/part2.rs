mod common;

fn main() {
    let map = common::parse_input();
    let mut num_xmas = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            for (di, dj) in [(1, 1), (-1, -1), (1, -1), (-1, 1)] {
                if common::match_word(&map, "MAS", i, j, di, dj, 1)
                    && common::match_word(&map, "MAS", i, j, dj, -di, 1)
                {
                    num_xmas += 1;
                }
            }
        }
    }
    println!("Result: {}", num_xmas);
}

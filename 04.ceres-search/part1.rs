mod common;

fn main() {
    let map = common::parse_input();
    let mut num_xmas = 0;
    for i in 0..map.len() {
        for j in 0..map[i].len() {
            for (di, dj) in [-1, 0, 1]
                .iter()
                .flat_map(|&di| [-1, 0, 1].iter().map(move |&dj| (di, dj)))
                .filter(|&(di, dj)| di != 0 || dj != 0)
            {
                if common::match_word(&map, "XMAS", i, j, di, dj, 0) {
                    num_xmas += 1;
                }
            }
        }
    }
    println!("Result: {}", num_xmas);
}

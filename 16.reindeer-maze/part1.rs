mod common;

fn main() {
    let (map, start, end) = common::parse_input();
    let dist = common::shortest_paths(&map, start);
    println!("Result: {}", dist[end[0]][end[1]].iter().min().unwrap());
}

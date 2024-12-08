mod common;

fn main() {
    let map = common::parse_input();
    println!("Result: {}", common::count_antinodes(&map, 0..));
}

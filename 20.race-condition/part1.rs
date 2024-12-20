mod common;

fn main() {
    let (map, start) = common::parse_input();
    let result = common::count_cheats(&map, start, 2, 100);
    println!("Result: {result}");
}

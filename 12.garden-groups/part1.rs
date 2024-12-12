mod common;

fn main() {
    let map = common::parse_input();
    let mut result = 0;
    for (area, perims) in common::shapes_iter(&map) {
        result += area * perims.into_iter().map(|e| e.len()).sum::<usize>();
    }
    println!("Result: {}", result);
}

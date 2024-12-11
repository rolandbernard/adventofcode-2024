mod common;

fn main() {
    let before = common::parse_input();
    let after = common::perform_blinks(before, 75);
    let result = after.values().sum::<usize>();
    println!("Result: {}", result);
}
use std::io::Read;

mod common;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).ok();
    println!(
        "Result: {}",
        common::execute(&input, &["mul(", "do()", "don't()"])
    );
}

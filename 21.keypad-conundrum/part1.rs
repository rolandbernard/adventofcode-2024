mod common;

fn main() {
    let mut result = 0;
    for line in common::parse_input() {
        result += common::complexities(&line, 3);
    }
    println!("Result: {result}");
}

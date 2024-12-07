mod common;

fn main() {
    let mut result = 0;
    for (target, operands) in common::parse_input() {
        if common::is_solvable(target, &operands, true) {
            result += target;
        }
    }
    println!("Result: {}", result);
}

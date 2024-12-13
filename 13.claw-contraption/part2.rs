mod common;

const OFFSET: i64 = 10000000000000;

fn main() {
    let mut result = 0;
    for [a, b, [c1, c2]] in common::parse_input() {
        if let Some(cost) = common::minimal_cost([a, b, [c1 + OFFSET, c2 + OFFSET]]) {
            result += cost;
        }
    }
    println!("Result: {}", result);
}

mod common;

fn main() {
    let mut result = 0;
    for machine in common::parse_input() {
        if let Some(cost) = common::minimal_cost(machine) {
            result += cost;
        }
    }
    println!("Result: {}", result);
}

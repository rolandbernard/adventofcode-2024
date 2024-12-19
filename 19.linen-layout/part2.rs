mod common;

fn main() {
    let input = common::read_input();
    let (base, patterns) = common::parse_input(&input);
    let mut result = 0;
    for pattern in patterns {
        result += common::num_possible(pattern, &base);
    }
    println!("Result: {}", result);
}

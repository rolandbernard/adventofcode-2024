mod common;

fn main() {
    let input = common::read_input();
    let (base, patterns) = common::parse_input(&input);
    let mut result = 0;
    for pattern in patterns {
        if common::num_possible(pattern, &base) > 0 {
            result += 1;
        }
    }
    println!("Result: {}", result);
}

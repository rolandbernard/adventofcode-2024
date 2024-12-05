mod common;

fn main() {
    let (rules, orders) = common::parse_input();
    let mut result = 0;
    for order in orders {
        if let Ok(()) = common::is_ordered(&rules, &order) {
            assert!(order.len() % 2 == 1);
            result += order[(order.len() - 1) / 2];
        }
    }
    println!("Result: {}", result);
}

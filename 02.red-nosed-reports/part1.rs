mod common;

fn main() {
    let reports = common::parse_input();
    let mut num_safe = 0;
    for report in reports {
        if common::is_safe(&report) {
            num_safe += 1;
        }
    }
    println!("Result: {}", num_safe);
}

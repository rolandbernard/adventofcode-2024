mod common;

fn main() {
    let reports = common::parse_input();
    let mut num_safe = 0;
    for report in reports {
        if common::is_safe(&report)
            || (0..report.len()).any(|i| {
                let mut copy = report.clone();
                copy.remove(i);
                common::is_safe(&copy)
            })
        {
            num_safe += 1;
        }
    }
    println!("Result: {}", num_safe);
}

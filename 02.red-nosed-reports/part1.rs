fn main() {
    let mut reports = Vec::<Vec<i64>>::new();
    for line in std::io::stdin().lines() {
        reports.push(
            line.unwrap()
                .split_whitespace()
                .map(str::parse)
                .map(Result::unwrap)
                .collect(),
        );
    }
    let mut num_safe = 0;
    'report: for report in reports {
        let mut last = report[0];
        let dir = (report[1] - last).signum();
        if dir == 0 {
            continue 'report;
        }
        for &level in &report[1..] {
            let diff = level - last;
            if diff.signum() != dir || diff.abs() > 3 {
                continue 'report;
            }
            last = level;
        }
        num_safe += 1;
    }
    println!("Result: {}", num_safe);
}

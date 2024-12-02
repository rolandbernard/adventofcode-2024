fn is_safe(report: &[i64]) -> bool {
    let mut last = report[0];
    let dir = (report[1] - last).signum();
    if dir == 0 {
        return false;
    }
    for &level in &report[1..] {
        let diff = level - last;
        if diff.signum() != dir || diff.abs() > 3 {
            return false;
        }
        last = level;
    }
    true
}

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
    for report in reports {
        if is_safe(&report)
            || (0..report.len()).any(|i| {
                let mut copy = report.clone();
                copy.remove(i);
                is_safe(&copy)
            })
        {
            num_safe += 1;
        }
    }
    println!("Result: {}", num_safe);
}

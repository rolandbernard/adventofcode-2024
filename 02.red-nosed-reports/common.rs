pub fn parse_input() -> Vec<Vec<i64>> {
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
    reports
}

pub fn is_safe(report: &[i64]) -> bool {
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

pub fn parse_input() -> Vec<(i64, Vec<i64>)> {
    std::io::stdin()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let (res_str, op_str) = line.split_once(": ").unwrap();
            (
                res_str.parse().unwrap(),
                op_str
                    .split_whitespace()
                    .map(|e| e.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

pub fn is_solvable(target: i64, ops: &[i64], concat: bool) -> bool {
    let len = ops.len();
    let last_op = ops[len - 1];
    let last_len = 10i64.pow(last_op.ilog10() + 1);
    if len == 1 {
        target == last_op
    } else {
        is_solvable(target - last_op, &ops[..len - 1], concat)
            || (target % last_op == 0 && is_solvable(target / last_op, &ops[..len - 1], concat))
            || (concat
                && target % last_len == last_op
                && is_solvable(target / last_len, &ops[..len - 1], concat))
    }
}

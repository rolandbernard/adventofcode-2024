fn is_solvable(target: i64, ops: &[i64]) -> bool {
    let len = ops.len();
    let last_op = ops[len - 1];
    let last_len = 10i64.pow(last_op.ilog10() + 1);
    if len == 1 {
        target == last_op
    } else {
        is_solvable(target - last_op, &ops[..len - 1])
            || (target % last_op == 0 && is_solvable(target / last_op, &ops[..len - 1]))
            || (target % last_len == last_op && is_solvable(target / last_len, &ops[..len - 1]))
    }
}

fn main() {
    let mut result = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (res_str, op_str) = line.split_once(": ").unwrap();
        let target = res_str.parse::<i64>().unwrap();
        let operands = op_str
            .split_whitespace()
            .map(|e| e.parse::<i64>().unwrap())
            .collect::<Vec<_>>();
        if is_solvable(target, &operands) {
            result += target;
        }
    }
    println!("Result: {}", result);
}

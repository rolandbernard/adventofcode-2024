use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).ok();
    let mut enabled = true;
    let mut sum = 0;
    let mut idx = 0;
    while let Some(start) = {
        let mut min_idx = None;
        for instr in ["mul(", "do()", "don't()"] {
            if let Some(start) = input[idx..].find(instr) {
                if min_idx.is_none() || min_idx.unwrap() > start {
                    min_idx = Some(start);
                }
            }
        }
        min_idx
    } {
        if input[idx + start..].starts_with("do()") {
            idx += start + 4;
            enabled = true;
        } else if input[idx + start..].starts_with("don't()") {
            idx += start + 7;
            enabled = false;
        } else {
            idx += start + 4;
            if enabled {
                let mut val0 = 0;
                while let Some(digit) = input[idx..].chars().next().and_then(|c| c.to_digit(10)) {
                    val0 = 10 * val0 + digit;
                    idx += 1;
                }
                if !input[idx..].starts_with(',') {
                    continue;
                }
                idx += 1;
                let mut val1 = 0;
                while let Some(digit) = input[idx..].chars().next().and_then(|c| c.to_digit(10)) {
                    val1 = 10 * val1 + digit;
                    idx += 1;
                }
                if !input[idx..].starts_with(')') {
                    continue;
                }
                idx += 1;
                sum += val0 * val1;
            }
        }
    }
    println!("Result: {}", sum);
}

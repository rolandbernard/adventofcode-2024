use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).ok();
    let mut sum = 0;
    let mut idx = 0;
    while let Some(start) = input[idx..].find("mul(") {
        idx += start + 4;
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
    println!("Result: {}", sum);
}

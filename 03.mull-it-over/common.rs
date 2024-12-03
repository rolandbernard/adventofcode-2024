pub fn execute(input: &str, instr: &[&str]) -> u32 {
    let mut enabled = true;
    let mut sum = 0;
    let mut idx = 0;
    while let Some(start) = instr.iter().filter_map(|e| input[idx..].find(e)).min() {
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
    sum
}

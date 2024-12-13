use std::io::Read;

pub fn parse_input() -> Vec<[[i64; 2]; 3]> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    input
        .split("\n\n")
        .map(|machine| {
            machine
                .lines()
                .map(|line| {
                    line.split(": ")
                        .nth(1)
                        .unwrap()
                        .split(", ")
                        .map(|e| e[2..].parse().unwrap())
                        .collect::<Vec<_>>()
                        .try_into()
                        .unwrap()
                })
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect()
}

pub fn minimal_cost([[a1, a2], [b1, b2], [c1, c2]]: [[i64; 2]; 3]) -> Option<i64> {
    let div = a1 * b2 - a2 * b1;
    let num1 = c1 * b2 - c2 * b1;
    let num2 = c1 * a2 - c2 * a1;
    let mut x1 = 0;
    let mut x2 = 0;
    if div != 0 {
        (x1, x2) = (num1 / div, num2 / -div);
    } else if num1 == 0 && num2 == 0 {
        unimplemented!();
    }
    if x1 >= 0 && x2 >= 0 && x1 * a1 + x2 * b1 == c1 && x1 * a2 + x2 * b2 == c2 {
        Some(3 * x1 + x2)
    } else {
        None
    }
}

use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut result = 0;
    for machine in input.split("\n\n") {
        let values: [[i64; 2]; 3] = machine
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
            .unwrap();
        let [[a1, a2], [b1, b2], [c1, c2]] = values;
        if 3 * b1 > a1 {
            for x1 in (0..=c1 / a1).rev() {
                let x2 = (c1 - x1 * a1) / b1;
                if x1 * a1 + x2 * b1 == c1 && x1 * a2 + x2 * b2 == c2 {
                    result += 3 * x1 + x2;
                    break;
                }
            }
        } else {
            for x1 in 0..=c1 / a1 {
                let x2 = (c1 - x1 * a1) / b1;
                if x1 * a1 + x2 * b1 == c1 && x1 * a2 + x2 * b2 == c2 {
                    result += 3 * x1 + x2;
                    break;
                }
            }
        }
    }
    println!("Result: {}", result);
}

fn next_number(mut x: u64) -> u64 {
    x ^= x * 64;
    x %= 16777216;
    x ^= x / 32;
    x %= 16777216;
    x ^= x * 2048;
    x %= 16777216;
    x
}

fn main() {
    let seeds = std::io::stdin()
        .lines()
        .map(|l| l.unwrap().parse::<u64>().unwrap());
    let mut result = 0;
    for mut x in seeds {
        for _ in 0..2000 {
            x = next_number(x);
        }
        result += x;
    }
    println!("Result: {result}");
}

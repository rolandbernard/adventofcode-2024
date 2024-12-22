pub fn parse_input() -> Vec<i64> {
    std::io::stdin()
        .lines()
        .map(|l| l.unwrap().parse().unwrap())
        .collect()
}

fn mix_number(mut x: i64) -> i64 {
    x ^= x << 6;
    x &= 0xff_ffff;
    x ^= x >> 5;
    x &= 0xff_ffff;
    x ^= x << 11;
    x &= 0xff_ffff;
    x
}

pub fn secret_numbers(mut x: i64) -> impl Iterator<Item = i64> {
    (0..2000).map(move |_| {
        x = mix_number(x);
        x
    })
}

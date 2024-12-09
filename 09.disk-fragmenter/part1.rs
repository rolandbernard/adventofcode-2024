use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut map = Vec::new();
    for (i, c) in input.chars().filter(|c| c.is_ascii_digit()).enumerate() {
        for _ in 0..c.to_digit(10).unwrap() {
            if i % 2 == 0 {
                map.push((i / 2) as i64);
            } else {
                map.push(-1);
            }
        }
    }
    let mut result = 0;
    let mut head = 0;
    while head < map.len() {
        if map[head] == -1 {
            map[head] = map.pop().unwrap();
        } else {
            result += head as i64 * map[head];
            head += 1;
        }
    }
    println!("Result: {}", result);
}

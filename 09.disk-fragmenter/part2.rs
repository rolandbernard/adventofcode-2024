use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut map = Vec::new();
    let mut files = Vec::new();
    let mut pos = 0;
    for (i, c) in input.chars().filter(|c| c.is_ascii_digit()).enumerate() {
        let size = c.to_digit(10).unwrap() as usize;
        for _ in 0..c.to_digit(10).unwrap() {
            if i % 2 == 0 {
                map.push((i / 2) as i64);
            } else {
                map.push(-1);
            }
        }
        files.push((pos, size));
        pos += size;
    }
    for (pos, size) in files.into_iter().rev() {
        for i in 0..pos {
            if map[i..i + size].iter().all(|&x| x == -1) {
                for j in 0..size {
                    map[i + j] = map[pos + j];
                    map[pos + j] = -1;
                }
                break;
            }
        }
    }
    let mut result = 0;
    for (i, v) in map.into_iter().enumerate() {
        if v != -1 {
            result += i * v as usize;
        }
    }
    println!("Result: {}", result);
}

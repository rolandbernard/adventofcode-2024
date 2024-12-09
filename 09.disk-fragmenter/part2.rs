use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut free = Vec::new();
    let mut files = Vec::new();
    let mut pos = 0;
    for (i, c) in input.chars().filter(|c| c.is_ascii_digit()).enumerate() {
        let size = c.to_digit(10).unwrap() as usize;
        if i % 2 == 0 {
            files.push((pos, size));
        } else {
            free.push((pos, size));
        }
        pos += size;
    }
    let mut result = 0;
    let mut heads = [0; 10];
    for (id, (mut pos, size)) in files.into_iter().enumerate().rev() {
        while heads[size] < free.len() && free[heads[size]].0 < pos {
            if free[heads[size]].1 >= size {
                pos = free[heads[size]].0;
                free[heads[size]].0 += size;
                free[heads[size]].1 -= size;
                break;
            } else {
                heads[size] += 1;
            }
        }
        result += id * size * (2 * pos + size - 1) / 2;
    }
    println!("Result: {}", result);
}

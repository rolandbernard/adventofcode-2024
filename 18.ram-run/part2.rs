use std::collections::{HashSet, VecDeque};

pub const SIZE: i64 = 70;

fn has_path(bytes: &[[i64; 2]]) -> bool {
    let mut seen = HashSet::new();
    for &[x, y] in bytes {
        seen.insert([x, y]);
    }
    let mut queue = VecDeque::new();
    seen.insert([0, 0]);
    queue.push_back([0, 0]);
    while let Some([x, y]) = queue.pop_front() {
        if x == SIZE && y == SIZE {
            return true;
        }
        for [nx, ny] in [[x + 1, y], [x - 1, y], [x, y + 1], [x, y - 1]] {
            if nx >= 0 && ny >= 0 && nx <= SIZE && ny <= SIZE && !seen.contains(&[nx, ny]) {
                seen.insert([nx, ny]);
                queue.push_back([nx, ny]);
            }
        }
    }
    false
}

fn main() {
    let mut bytes = Vec::<[i64; 2]>::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (l, r) = line.split_once(',').unwrap();
        bytes.push([l.parse().unwrap(), r.parse().unwrap()]);
    }
    let mut l = 0;
    let mut h = bytes.len();
    while l != h {
        let m = (l + h + 1) / 2;
        if has_path(&bytes[..m]) {
            l = m;
        } else {
            h = m - 1;
        }
    }
    println!("Result: {},{}", bytes[l][0], bytes[l][1]);
}

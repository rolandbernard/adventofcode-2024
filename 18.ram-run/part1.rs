use std::collections::{HashMap, VecDeque};

pub const SIZE: i64 = 70;
pub const FALLEN: usize = 1024;

fn main() {
    let mut bytes = Vec::<[i64; 2]>::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let (l, r) = line.split_once(',').unwrap();
        bytes.push([l.parse().unwrap(), r.parse().unwrap()]);
    }
    let mut dist = HashMap::new();
    for &[x, y] in &bytes[..FALLEN] {
        dist.insert([x, y], usize::MAX);
    }
    let mut queue = VecDeque::new();
    dist.insert([0, 0], 0);
    queue.push_back([0, 0]);
    while let Some([x, y]) = queue.pop_front() {
        if x == SIZE && y == SIZE {
            break;
        }
        for [nx, ny] in [[x + 1, y], [x - 1, y], [x, y + 1], [x, y - 1]] {
            if nx >= 0 && ny >= 0 && nx <= SIZE && ny <= SIZE && !dist.contains_key(&[nx, ny]) {
                dist.insert([nx, ny], dist[&[x, y]] + 1);
                queue.push_back([nx, ny]);
            }
        }
    }
    println!("Result: {}", dist[&[SIZE, SIZE]]);
}

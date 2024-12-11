use std::{collections::HashMap, ops::AddAssign};

fn main() {
    let mut numbers = HashMap::<usize, usize>::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        for num in line.split_whitespace() {
            numbers
                .entry(num.parse().unwrap())
                .or_default()
                .add_assign(1);
        }
    }
    for _ in 0..75 {
        let mut next = HashMap::<usize, usize>::new();
        for (num, count) in numbers {
            if num == 0 {
                next.entry(1).or_default().add_assign(count);
            } else if num.ilog10() % 2 == 1 {
                let div = 10usize.pow(num.ilog10() / 2 + 1);
                next.entry(num / div).or_default().add_assign(count);
                next.entry(num % div).or_default().add_assign(count);
            } else {
                next.entry(num * 2024).or_default().add_assign(count);
            }
        }
        numbers = next;
    }
    let result = numbers.values().sum::<usize>();
    println!("Result: {}", result);
}

use std::{collections::HashMap, ops::AddAssign};

pub fn parse_input() -> HashMap<usize, usize> {
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
    numbers
}

pub fn perform_blinks(mut state: HashMap<usize, usize>, blinks: usize) -> HashMap<usize, usize> {
    for _ in 0..blinks {
        let mut next = HashMap::<usize, usize>::new();
        for (num, count) in state {
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
        state = next;
    }
    state
}

use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for schematic in input.split("\n\n") {
        let mut numbers = [0; 5];
        for line in schematic.lines() {
            for (i, _) in line.chars().enumerate().filter(|&(_, c)| c == '#') {
                numbers[i] += 1;
            }
        }
        if schematic.lines().next().unwrap().chars().all(|c| c == '#') {
            locks.push(numbers);
        } else {
            keys.push(numbers);
        }
    }
    let mut result = 0;
    for key in &keys {
        for lock in &locks {
            if key.iter().zip(lock.iter()).all(|(k, l)| k + l <= 7) {
                result += 1;
            }
        }
    }
    println!("Result: {result}");
}

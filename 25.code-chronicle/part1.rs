use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for schematic in input.split("\n\n") {
        let schematic = schematic
            .trim()
            .lines()
            .map(|e| e.chars().collect())
            .collect::<Vec<Vec<_>>>();
        let numbers = (0..schematic[0].len())
            .map(|i| {
                (0..schematic.len())
                    .filter(|&j| schematic[j][i] == '#')
                    .count()
            })
            .collect::<Vec<_>>();
        if schematic[0].iter().all(|&c| c == '#') {
            locks.push(numbers);
        } else {
            keys.push(
                numbers
                    .into_iter()
                    .map(|e| schematic.len() - e)
                    .collect::<Vec<_>>(),
            );
        }
    }
    let mut result = 0;
    for key in &keys {
        for lock in &locks {
            if key.iter().zip(lock.iter()).all(|(k, l)| k >= l) {
                result += 1;
            }
        }
    }
    println!("Result: {result}");
}

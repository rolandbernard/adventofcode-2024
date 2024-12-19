use std::io::Read;

pub fn read_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    input
}

pub fn parse_input(input: &str) -> (Vec<Vec<&[u8]>>, Vec<&[u8]>) {
    let (avail, targets) = input.split_once("\n\n").unwrap();
    let mut base = vec![Vec::new(); 256];
    for pattern in avail.split(", ") {
        let bytes = pattern.as_bytes();
        base[bytes[0] as usize].push(bytes);
    }
    let patterns = targets.lines().map(str::as_bytes).collect();
    (base, patterns)
}

pub fn num_possible(pattern: &[u8], base: &[Vec<&[u8]>]) -> usize {
    let len = pattern.len();
    let mut combs = vec![0; len + 1];
    combs[0] = 1;
    for i in 0..pattern.len() {
        for &base in &base[pattern[i] as usize] {
            if i + base.len() <= len && base == &pattern[i..i + base.len()] {
                combs[i + base.len()] += combs[i];
            }
        }
    }
    combs[pattern.len()]
}

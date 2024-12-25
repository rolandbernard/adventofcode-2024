mod common;

struct Random(u64);

impl Random {
    fn next(&mut self, bits: usize) -> u64 {
        let res = (self.0 >> (64 - bits)) & ((1 << bits) - 1);
        self.0 = self.0.wrapping_mul(6364136223846793005);
        self.0 = self.0.wrapping_add(1442695040888963407);
        res
    }
}

impl common::Circuit {
    pub fn run(&self, mut x: u64, mut y: u64) -> u64 {
        let mut values = vec![None; 2 * self.xybits + self.gates.len()];
        for i in 0..self.xybits {
            values[i] = Some((x & 1) == 1);
            values[self.xybits + i] = Some((y & 1) == 1);
            x >>= 1;
            y >>= 1;
        }
        self.run_with(&mut values)
    }

    pub fn swap(&mut self, [i, j]: [usize; 2]) {
        self.gates.swap(i, j);
    }
}

fn find_counter(circuit: &common::Circuit, rand: &mut Random) -> Option<[u64; 2]> {
    for _ in 0..100_000 {
        let x = rand.next(circuit.xybits);
        let y = rand.next(circuit.xybits);
        let z = circuit.run(x, y);
        if x + y != z {
            return Some([x, y]);
        }
    }
    None
}

fn main() {
    let (_, mut circuit) = common::parse_input();
    let mut rand = Random(0);
    let mut swaps = Vec::<[usize; 2]>::new();
    let mut examples = Vec::new();
    while let Some(counter) = find_counter(&circuit, &mut rand) {
        examples.push(counter);
        if swaps.len() == 4 {
            for &s in swaps.iter().rev() {
                circuit.swap(s);
            }
            swaps.clear();
        }
        let mut best_swap = [0, 0];
        let mut best_score = 0;
        for i in 0..circuit.gates.len() {
            for j in 0..i {
                if swaps
                    .iter()
                    .all(|&[a, b]| a != i && a != j && b != i && b != j)
                {
                    circuit.swap([i, j]);
                    let mut score = 0;
                    for &[x, y] in &examples {
                        score += u64::count_ones(!circuit.run(x, y) ^ (x + y));
                    }
                    if score > best_score {
                        best_score = score;
                        best_swap = [i, j];
                    }
                    circuit.swap([i, j]);
                }
            }
        }
        circuit.swap(best_swap);
        swaps.push(best_swap);
    }
    assert!(swaps.len() == 4);
    let mut result = swaps
        .into_iter()
        .flatten()
        .map(|i| circuit.names[2 * circuit.xybits + i].as_str())
        .collect::<Vec<_>>();
    result.sort();
    println!("Result: {}", result.join(","));
}

mod common;

impl common::Circuit {
    pub fn run(&self, value: &[(String, bool)]) -> u64 {
        let mut values = vec![None; 2 * self.xybits + self.gates.len()];
        for (idx, name) in self.names.iter().enumerate() {
            if let Some(&(_, v)) = value.iter().find(|(a, _)| a == name) {
                values[idx] = Some(v);
            }
        }
        self.run_with(&mut values)
    }
}

fn main() {
    let (values, circuit) = common::parse_input();
    println!("Result: {}", circuit.run(&values));
}

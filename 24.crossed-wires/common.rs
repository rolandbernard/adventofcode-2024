use std::{collections::HashMap, io::Read, iter};

#[derive(Clone, Copy)]
pub enum Gate {
    And(usize, usize),
    Or(usize, usize),
    Xor(usize, usize),
}

pub struct Circuit {
    pub names: Vec<String>,
    pub xybits: usize,
    pub zbits: Vec<usize>,
    pub gates: Vec<Gate>,
}

pub fn parse_input() -> (Vec<(String, bool)>, Circuit) {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (values, gates) = input.split_once("\n\n").unwrap();
    let values = values
        .trim()
        .lines()
        .map(|l| l.trim().split_once(": ").unwrap())
        .map(|(n, v)| (n.to_owned(), v == "1"))
        .collect::<Vec<_>>();
    let gates = gates
        .trim()
        .lines()
        .map(|l| l.split_once(" -> ").unwrap())
        .map(|(gate, name)| gate.split_whitespace().chain(iter::once(name)).collect())
        .collect::<Vec<Vec<_>>>();
    let mut names = Vec::new();
    for gate in &gates {
        for name in [gate[0], gate[2]] {
            if name.starts_with('x') || name.starts_with('y') {
                names.push(name.to_owned());
            }
        }
    }
    names.sort();
    names.dedup();
    let mut names_loc = names
        .iter()
        .enumerate()
        .map(|(i, n)| (n.to_owned(), i))
        .collect::<HashMap<_, _>>();
    let xybits = names.len() / 2;
    let mut zbits = Vec::new();
    for gate in &gates {
        if let Some(num) = gate[3].strip_prefix('z') {
            let idx = num.parse::<usize>().unwrap();
            if zbits.len() <= idx {
                zbits.resize(idx + 1, 0);
            }
            zbits[idx] = names.len();
        }
        names_loc.insert(gate[3].to_owned(), names.len());
        names.push(gate[3].to_owned());
    }
    let gates = gates
        .into_iter()
        .map(|gate| match gate[1] {
            "AND" => Gate::And(names_loc[gate[0]], names_loc[gate[2]]),
            "OR" => Gate::Or(names_loc[gate[0]], names_loc[gate[2]]),
            "XOR" => Gate::Xor(names_loc[gate[0]], names_loc[gate[2]]),
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();
    (
        values,
        Circuit {
            names,
            xybits,
            zbits,
            gates,
        },
    )
}

impl Circuit {
    fn compute_gate(&self, i: usize, values: &mut [Option<bool>]) -> bool {
        if let Some(z) = values[i] {
            z
        } else {
            assert!(i >= 2 * self.xybits);
            values[i] = Some(false);
            let gate = self.gates[i - 2 * self.xybits];
            let (Gate::And(x, y) | Gate::Or(x, y) | Gate::Xor(x, y)) = gate;
            let x = self.compute_gate(x, values);
            let y = self.compute_gate(y, values);
            let z = match gate {
                Gate::And(_, _) => x & y,
                Gate::Or(_, _) => x | y,
                Gate::Xor(_, _) => x ^ y,
            };
            values[i] = Some(z);
            z
        }
    }

    pub fn run_with(&self, values: &mut [Option<bool>]) -> u64 {
        let mut z = 0;
        for &idx in self.zbits.iter().rev() {
            z <<= 1;
            if self.compute_gate(idx, values) {
                z |= 1;
            }
        }
        z
    }
}

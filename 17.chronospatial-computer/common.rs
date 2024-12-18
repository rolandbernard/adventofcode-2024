use std::io::Read;

pub fn parse_input() -> ([u64; 3], Vec<u8>) {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let (regs, program) = input.split_once("\n\n").unwrap();
    let regs = regs
        .lines()
        .map(|e| e.split_once(": ").unwrap().1.parse().unwrap())
        .collect::<Vec<_>>();
    let program = program
        .split_once(": ")
        .unwrap()
        .1
        .split(',')
        .map(|e| e.trim().parse().unwrap())
        .collect();
    (regs.try_into().unwrap(), program)
}

pub fn run_program(mut regs: [u64; 3], program: &[u8]) -> Vec<u8> {
    let mut output = Vec::new();
    let mut ip = 0;
    while let Some(opc) = program.get(ip) {
        let operand = program[ip + 1] as u64;
        let combo = if (4..=6).contains(&operand) {
            regs[operand as usize - 4]
        } else {
            operand
        };
        ip += 2;
        match opc {
            0 => regs[0] = if combo >= 64 { 0 } else { regs[0] >> combo },
            1 => regs[1] ^= operand,
            2 => regs[1] = combo % 8,
            3 => {
                if regs[0] != 0 {
                    ip = operand as usize
                }
            }
            4 => regs[1] ^= regs[2],
            5 => output.push(combo as u8 % 8),
            6 => regs[1] = if combo >= 64 { 0 } else { regs[0] >> combo },
            7 => regs[2] = if combo >= 64 { 0 } else { regs[0] >> combo },
            _ => unreachable!(),
        }
    }
    output
}

mod common;

fn main() {
    let (regs, program) = common::parse_input();
    let output = common::run_program(regs, &program);
    println!(
        "Result: {}",
        output
            .iter()
            .map(u8::to_string)
            .collect::<Vec<_>>()
            .join(",")
    );
}

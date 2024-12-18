mod common;

fn main() {
    let ([_, b, c], program) = common::parse_input();
    let len = program.len();
    assert!(program.chunks(2).filter(|c| c[0] == 5).count() == 1);
    assert!(program.chunks(2).filter(|c| c[0] == 0).count() == 1);
    assert!(program.chunks(2).filter(|c| c == &[0, 3]).count() == 1);
    assert!(program.chunks(2).filter(|c| c[0] == 3).count() == 1);
    assert!(program[len - 2..] == [3, 0]);
    let mut min = u64::MAX;
    let mut stack = Vec::new();
    stack.push((0, 1));
    while let Some((a, i)) = stack.pop() {
        let out = common::run_program([a, b, c], &program);
        if (a & 0b111) != 0b111 {
            stack.push((a + 1, i));
        }
        if out.len() >= i && program[len - i] == out[out.len() - i] {
            if i == len {
                min = min.min(a);
            } else {
                stack.push((8 * a, i + 1));
            }
        }
    }
    println!("Result: {min}");
}

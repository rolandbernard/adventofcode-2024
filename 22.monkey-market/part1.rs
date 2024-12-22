mod common;

fn main() {
    let result = common::parse_input()
        .into_iter()
        .map(|seed| common::secret_numbers(seed).last().unwrap())
        .sum::<i64>();
    println!("Result: {result}");
}

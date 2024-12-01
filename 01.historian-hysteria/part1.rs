mod common;

fn main() {
    let lists = common::parse_input();
    let result = lists[0]
        .iter()
        .zip(lists[1].iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i64>();
    println!("Result: {}", result);
}

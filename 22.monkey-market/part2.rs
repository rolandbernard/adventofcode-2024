mod common;

fn main() {
    let mut price = vec![0; 19 * 19 * 19 * 19];
    for x in common::parse_input() {
        let mut seen = vec![false; 19 * 19 * 19 * 19];
        let mut change = 0;
        let mut last = x % 10;
        for (i, x) in common::secret_numbers(x).enumerate() {
            change = 19 * change % price.len() + (9 + x % 10 - last) as usize;
            if i >= 3 && !seen[change] {
                seen[change] = true;
                price[change] += x % 10;
            }
            last = x % 10;
        }
    }
    let result = price.into_iter().max().unwrap();
    println!("Result: {result:?}");
}

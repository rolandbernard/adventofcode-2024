mod common;

fn main() {
    let map = common::parse_input();
    let mut result = 0;
    for (area, perims) in common::shapes_iter(&map) {
        let mut perim = 0;
        for mut p in perims {
            p.sort();
            perim += 1;
            let mut last = p[0];
            for &next in &p[1..] {
                if last.0.abs_diff(next.0) + last.1.abs_diff(next.1) != 1 {
                    perim += 1;
                }
                last = next;
            }
        }
        result += area * perim;
    }
    println!("Result: {}", result);
}

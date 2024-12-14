mod common;

fn variance(values: impl Iterator<Item = f64>) -> f64 {
    let values = values.collect::<Vec<_>>();
    let recip = f64::recip(values.len() as f64);
    let mean = values.iter().sum::<f64>() * recip;
    values
        .iter()
        .map(|&x| recip * (x - mean).powi(2))
        .sum::<f64>()
}

fn main() {
    let mut robots = common::parse_input();
    let mut min_var = f64::INFINITY;
    let mut min_idx = 0;
    for i in 0..common::WIDTH * common::HEIGHT {
        let var = variance(robots.iter().map(|&[[x, _], _]| x as f64))
            + variance(robots.iter().map(|&[[_, y], _]| y as f64));
        if var < min_var {
            min_var = var;
            min_idx = i;
        }
        common::move_robots(&mut robots, 1);
    }
    println!("Result: {min_idx}");
}

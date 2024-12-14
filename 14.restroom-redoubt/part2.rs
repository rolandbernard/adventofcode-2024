use core::f64;

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

fn move_robots(robots: &mut [[[i64; 2]; 2]], step: i64) {
    for [[x, y], [dx, dy]] in robots {
        *x = (*x + *dx * step).rem_euclid(WIDTH);
        *y = (*y + *dy * step).rem_euclid(HEIGHT);
    }
}

fn variance(values: impl Iterator<Item = f64>) -> f64 {
    let values = values.collect::<Vec<_>>();
    let recip = f64::recip(values.len() as f64);
    let mean = values.iter().sum::<f64>() * recip;
    values
        .iter()
        .map(|&x| recip * (x - mean).powi(2))
        .sum::<f64>()
}

fn position_variance(robots: &[[[i64; 2]; 2]]) -> f64 {
    variance(robots.iter().map(|&[[x, _], _]| x as f64))
        + variance(robots.iter().map(|&[[_, y], _]| y as f64))
}

fn main() {
    let mut robots = Vec::new();
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let robot = line
            .split(' ')
            .map(|e| {
                e.split('=')
                    .nth(1)
                    .unwrap()
                    .split(',')
                    .map(|c| c.parse::<i64>().unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        robots.push(robot);
    }
    let mut min_var = f64::INFINITY;
    let mut min_idx = 0;
    for i in 0..WIDTH * HEIGHT {
        let var = position_variance(&robots);
        if var < min_var {
            min_var = var;
            min_idx = i;
        }
        move_robots(&mut robots, 1);
    }
    println!("Result: {min_idx}");
}

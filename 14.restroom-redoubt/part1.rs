const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;
const STEPS: i64 = 100;

fn main() {
    let mut quads = [[0; 2]; 2];
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let [[x, y], [dx, dy]] = line
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
        let x = (x + dx * STEPS).rem_euclid(WIDTH);
        let y = (y + dy * STEPS).rem_euclid(HEIGHT);
        if x != WIDTH / 2 && y != HEIGHT / 2 {
            quads[(x * 2 / WIDTH) as usize][(y * 2 / HEIGHT) as usize] += 1;
        }
    }
    let result = quads[0][0] * quads[0][1] * quads[1][0] * quads[1][1];
    println!("Result: {}", result);
}

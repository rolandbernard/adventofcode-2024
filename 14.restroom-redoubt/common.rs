pub const WIDTH: i64 = 101;
pub const HEIGHT: i64 = 103;

pub fn parse_input() -> Vec<[[i64; 2]; 2]> {
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
                    .map(|c| c.parse().unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        robots.push(robot);
    }
    robots
}

pub fn move_robots(robots: &mut [[[i64; 2]; 2]], step: i64) {
    for [[x, y], [dx, dy]] in robots {
        *x = (*x + *dx * step).rem_euclid(WIDTH);
        *y = (*y + *dy * step).rem_euclid(HEIGHT);
    }
}

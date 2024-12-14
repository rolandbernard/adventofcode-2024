mod common;

fn main() {
    let mut robots = common::parse_input();
    common::move_robots(&mut robots, 100);
    let mut quads = [[0; 2]; 2];
    for [[x, y], _] in robots {
        if x != common::WIDTH / 2 && y != common::HEIGHT / 2 {
            let qx = (x * 2 / common::WIDTH) as usize;
            let qy = (y * 2 / common::HEIGHT) as usize;
            quads[qx][qy] += 1;
        }
    }
    let result = quads[0][0] * quads[0][1] * quads[1][0] * quads[1][1];
    println!("Result: {result}");
}

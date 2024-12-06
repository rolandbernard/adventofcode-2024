mod common;

fn main() {
    let (map, mut pos) = common::parse_input();
    let mut visited = vec![vec![0; map[0].len()]; map.len()];
    let mut dir = (-1, 0);
    loop {
        visited[pos.0 as usize][pos.1 as usize] = 1;
        let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
        if !(0..map.len() as i32).contains(&new_pos.0)
            || !(0..map[0].len() as i32).contains(&new_pos.1)
        {
            break;
        } else if map[new_pos.0 as usize][new_pos.1 as usize] == '#' {
            dir = (dir.1, -dir.0);
        } else {
            pos = new_pos;
        }
    }
    println!("Result: {}", visited.iter().flatten().sum::<i32>());
}

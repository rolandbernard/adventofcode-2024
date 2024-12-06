mod common;

fn solve(
    map: &mut [Vec<char>],
    pos: (i32, i32),
    dir: usize,
    can_place: bool,
    visited: &mut [Vec<Vec<bool>>],
) -> i32 {
    let new_pos = (pos.0 + [-1, 0, 1, 0][dir], pos.1 + [0, 1, 0, -1][dir]);
    if !(0..map.len() as i32).contains(&new_pos.0) || !(0..map[0].len() as i32).contains(&new_pos.1)
    {
        0
    } else if visited[pos.0 as usize][pos.1 as usize][dir] {
        assert!(!can_place);
        1
    } else {
        let mut result;
        visited[pos.0 as usize][pos.1 as usize][dir] = true;
        if map[new_pos.0 as usize][new_pos.1 as usize] == '#' {
            result = solve(map, pos, (dir + 1) % 4, can_place, visited)
        } else {
            result = solve(map, new_pos, dir, can_place, visited);
            let visited_cell = &mut visited[new_pos.0 as usize][new_pos.1 as usize];
            if can_place && visited_cell.iter().all(|e| !e) {
                map[new_pos.0 as usize][new_pos.1 as usize] = '#';
                result += solve(map, pos, (dir + 1) % 4, false, visited);
                map[new_pos.0 as usize][new_pos.1 as usize] = '.';
            }
        }
        visited[pos.0 as usize][pos.1 as usize][dir] = false;
        result
    }
}

fn main() {
    let (mut map, pos) = common::parse_input();
    let mut visited = vec![vec![vec![false; 4]; map[0].len()]; map.len()];
    println!("Result: {}", solve(&mut map, pos, 0, true, &mut visited));
}

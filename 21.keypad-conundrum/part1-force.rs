use std::collections::{HashMap, VecDeque};

fn main() {
    let mut result = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let loc = line.chars().collect::<Vec<char>>();
        let mut dist = HashMap::new();
        let mut queue = VecDeque::new();
        let start = (0, [[0, 2], [0, 2], [0, 2], [0, 2], [0, 2], [0, 2], [3, 2]]);
        dist.insert(start, 0);
        queue.push_back(start);
        while let Some(state) = queue.pop_front() {
            if state.0 == loc.len() {
                eprintln!("{}", dist[&state]);
                result += dist[&state]
                    * loc
                        .iter()
                        .filter(|c| c.is_ascii_digit())
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();
                break;
            }
            for [di, dj] in [[0, 0], [1, 0], [-1, 0], [0, 1], [0, -1]] {
                let mut n_state = state;
                if [di, dj] == [0, 0] {
                    match n_state.1[0] {
                        [1, 0] => n_state.1[1][1] -= 1,
                        [1, 1] => n_state.1[1][0] += 1,
                        [1, 2] => n_state.1[1][1] += 1,
                        [0, 1] => n_state.1[1][0] -= 1,
                        [0, 2] => match n_state.1[1] {
                            [1, 0] => n_state.1[2][1] -= 1,
                            [1, 1] => n_state.1[2][0] += 1,
                            [1, 2] => n_state.1[2][1] += 1,
                            [0, 1] => n_state.1[2][0] -= 1,
                            [0, 2] => match n_state.1[2] {
                                [1, 0] => n_state.1[3][1] -= 1,
                                [1, 1] => n_state.1[3][0] += 1,
                                [1, 2] => n_state.1[3][1] += 1,
                                [0, 1] => n_state.1[3][0] -= 1,
                                [0, 2] => match n_state.1[3] {
                                    [1, 0] => n_state.1[4][1] -= 1,
                                    [1, 1] => n_state.1[4][0] += 1,
                                    [1, 2] => n_state.1[4][1] += 1,
                                    [0, 1] => n_state.1[4][0] -= 1,
                                    [0, 2] => match n_state.1[4] {
                                        [1, 0] => n_state.1[5][1] -= 1,
                                        [1, 1] => n_state.1[5][0] += 1,
                                        [1, 2] => n_state.1[5][1] += 1,
                                        [0, 1] => n_state.1[5][0] -= 1,
                                        [0, 2] => match n_state.1[5] {
                                            [1, 0] => n_state.1[6][1] -= 1,
                                            [1, 1] => n_state.1[6][0] += 1,
                                            [1, 2] => n_state.1[6][1] += 1,
                                            [0, 1] => n_state.1[6][0] -= 1,
                                            [0, 2] => {
                                                let c = match n_state.1[6] {
                                                    [0, 0] => '7',
                                                    [0, 1] => '8',
                                                    [0, 2] => '9',
                                                    [1, 0] => '4',
                                                    [1, 1] => '5',
                                                    [1, 2] => '6',
                                                    [2, 0] => '1',
                                                    [2, 1] => '2',
                                                    [2, 2] => '3',
                                                    [3, 1] => '0',
                                                    [3, 2] => 'A',
                                                    _ => unreachable!(),
                                                };
                                                if c != loc[n_state.0] {
                                                    continue;
                                                }
                                                n_state.0 += 1;
                                            }
                                            _ => unreachable!(),
                                        },
                                        _ => unreachable!(),
                                    },
                                    _ => unreachable!(),
                                },
                                _ => unreachable!(),
                            },
                            _ => unreachable!(),
                        },
                        _ => unreachable!(),
                    }
                } else {
                    n_state.1[0][0] += di;
                    n_state.1[0][1] += dj;
                }
                if n_state.1[0] == [0, 0]
                    || !(0..=1).contains(&n_state.1[0][0])
                    || !(0..=2).contains(&n_state.1[0][1])
                    || n_state.1[1] == [0, 0]
                    || !(0..=1).contains(&n_state.1[1][0])
                    || !(0..=2).contains(&n_state.1[1][1])
                    || n_state.1[2] == [0, 0]
                    || !(0..=1).contains(&n_state.1[2][0])
                    || !(0..=2).contains(&n_state.1[2][1])
                    || n_state.1[3] == [0, 0]
                    || !(0..=1).contains(&n_state.1[3][0])
                    || !(0..=2).contains(&n_state.1[3][1])
                    || n_state.1[4] == [0, 0]
                    || !(0..=1).contains(&n_state.1[4][0])
                    || !(0..=2).contains(&n_state.1[4][1])
                    || n_state.1[5] == [0, 0]
                    || !(0..=1).contains(&n_state.1[5][0])
                    || !(0..=2).contains(&n_state.1[5][1])
                    || n_state.1[6] == [3, 0]
                    || !(0..=3).contains(&n_state.1[6][0])
                    || !(0..=2).contains(&n_state.1[6][1])
                {
                    continue;
                }
                if !dist.contains_key(&n_state) {
                    dist.insert(n_state, dist[&state] + 1);
                    queue.push_back(n_state);
                }
            }
        }
    }
    println!("Result: {}", result);
}

use std::{
    collections::{HashMap, HashSet},
    iter,
    ops::AddAssign,
};

fn main() {
    let mut result = 0;
    for line in std::io::stdin().lines() {
        let line = line.unwrap();
        let mut blocked = [3, 0];
        let mut start = [3, 2];
        let mut loc = line
            .chars()
            .map(|c| match c {
                '7' => [0, 0],
                '8' => [0, 1],
                '9' => [0, 2],
                '4' => [1, 0],
                '5' => [1, 1],
                '6' => [1, 2],
                '1' => [2, 0],
                '2' => [2, 1],
                '3' => [2, 2],
                '0' => [3, 1],
                'A' => [3, 2],
                _ => unreachable!(),
            })
            .collect::<Vec<[i64; 2]>>();
        let mut ln = HashMap::<_, usize>::new();
        let mut last = start;
        for next in loc {
            ln.entry([last, next]).or_default().add_assign(1);
            last = next;
        }
        for _ in 0..3 {
            let mut n_ln = HashMap::<_, usize>::new();
            for ([last, next], c) in ln {
                let mov = [next[0] - last[0], next[1] - last[1]];
                if mov == [0, 0] {
                    n_ln.entry([[0, 2], [0, 2]]).or_default().add_assign(c);
                } else {
                    let h_pos = [1, if mov[1] < 0 { 0 } else { 2 }];
                    let v_pos = [if mov[0] > 0 { 1 } else { 0 }, 1];
                    if mov[0] == 0 {
                        n_ln.entry([[0, 2], h_pos]).or_default().add_assign(c);
                        n_ln.entry([h_pos, h_pos])
                            .or_default()
                            .add_assign(c * (mov[1].unsigned_abs() as usize - 1));
                        n_ln.entry([h_pos, [0, 2]]).or_default().add_assign(c);
                    } else if mov[1] == 0 {
                        n_ln.entry([[0, 2], v_pos]).or_default().add_assign(c);
                        n_ln.entry([v_pos, v_pos])
                            .or_default()
                            .add_assign(c * (mov[0].unsigned_abs() as usize - 1));
                        n_ln.entry([v_pos, [0, 2]]).or_default().add_assign(c);
                    } else {
                        let h_can_first = last[0] != blocked[0] || next[1] != blocked[1];
                        let v_can_first = last[1] != blocked[1] || next[0] != blocked[0];
                        let h_first = h_can_first && (!v_can_first || mov[0] < 0 || mov[1] < 0);
                        if h_first {
                            n_ln.entry([[0, 2], h_pos]).or_default().add_assign(c);
                            n_ln.entry([h_pos, h_pos])
                                .or_default()
                                .add_assign(c * (mov[1].unsigned_abs() as usize - 1));
                            n_ln.entry([h_pos, v_pos]).or_default().add_assign(c);
                            n_ln.entry([v_pos, v_pos])
                                .or_default()
                                .add_assign(c * (mov[0].unsigned_abs() as usize - 1));
                            n_ln.entry([v_pos, [0, 2]]).or_default().add_assign(c);
                        } else {
                            n_ln.entry([[0, 2], v_pos]).or_default().add_assign(c);
                            n_ln.entry([v_pos, v_pos])
                                .or_default()
                                .add_assign(c * (mov[0].unsigned_abs() as usize - 1));
                            n_ln.entry([v_pos, h_pos]).or_default().add_assign(c);
                            n_ln.entry([h_pos, h_pos])
                                .or_default()
                                .add_assign(c * (mov[1].unsigned_abs() as usize - 1));
                            n_ln.entry([h_pos, [0, 2]]).or_default().add_assign(c);
                        }
                    }
                }
            }
            blocked = [0, 0];
            ln = n_ln;
        }
        let cnt = ln.values().copied().sum::<usize>();
        eprintln!("{cnt}");
        result += cnt
            * line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
    }
    println!("Result: {}", result);
}

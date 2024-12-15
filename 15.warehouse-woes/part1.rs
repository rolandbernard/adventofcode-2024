mod common;

fn main() {
    let (mut map, instr, [mut i, mut j]) = common::parse_input();
    for [di, dj] in instr {
        let [ni, nj] = [i.wrapping_add(di), j.wrapping_add(dj)];
        let [mut nni, mut nnj] = [ni, nj];
        while map[nni][nnj] == 'O' {
            [nni, nnj] = [nni.wrapping_add(di), nnj.wrapping_add(dj)];
        }
        if map[nni][nnj] == '.' {
            map[nni][nnj] = 'O';
            map[ni][nj] = '@';
            map[i][j] = '.';
            [i, j] = [ni, nj];
        }
    }
    println!("Result: {}", common::sum_of_gps(&map));
}

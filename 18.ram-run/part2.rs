mod common;

fn main() {
    let bytes = common::parse_input();
    let mut l = 0;
    let mut h = bytes.len();
    while l != h {
        let m = (l + h + 1) / 2;
        if common::shortest_path(&bytes[..m]).is_some() {
            l = m;
        } else {
            h = m - 1;
        }
    }
    println!("Result: {},{}", bytes[l][0], bytes[l][1]);
}

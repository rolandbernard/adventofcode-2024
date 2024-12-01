mod common;

fn main() {
    let lists = common::parse_input();
    let mut score = 0;
    let mut lidx = 0;
    let mut ridx = 0;
    while lidx < lists[0].len() && ridx < lists[1].len() {
        let val = i64::min(lists[0][lidx], lists[1][ridx]);
        let mut lcnt = 0;
        let mut rcnt = 0;
        while lists[0].get(lidx) == Some(&val) {
            lcnt += 1;
            lidx += 1;
        }
        while lists[1].get(ridx) == Some(&val) {
            rcnt += 1;
            ridx += 1;
        }
        score += lcnt * rcnt * val;
    }
    println!("Result: {}", score);
}

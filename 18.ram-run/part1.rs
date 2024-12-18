mod common;

fn main() {
    let bytes = common::parse_input();
    let result = common::shortest_path(&bytes[..1024]).unwrap();
    println!("Result: {result}");
}

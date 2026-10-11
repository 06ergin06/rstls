use std::env;
use std::fs::read_dir;

fn main() {
    let target_dir = env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let list = read_dir(target_dir).expect("ERROR");
    for entry in list {
        let entry = entry.expect("ERROR");
        let file_name = entry.file_name();
        println!("{}", file_name.to_string_lossy());
    }
}

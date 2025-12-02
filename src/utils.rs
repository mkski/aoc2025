use std::fs::File;
use std::io::Read;

pub fn read_input(filename: &str) -> String {
    let mut file = File::open(filename).expect("Failed to open input file");
    let mut input = String::new();
    file.read_to_string(&mut input)
        .expect("Failed to read input file");
    input
}

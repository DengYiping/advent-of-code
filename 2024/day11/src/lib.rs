use std::io;

pub fn read() -> Vec<u64> {
    // Read a single line with space delimited numbers
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.split_whitespace()
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

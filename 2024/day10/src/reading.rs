use std::io::{stdin, BufRead};

pub fn read() -> Vec<Vec<u8>> {
    let mut result = vec![];
    for line in stdin().lock().lines() {
        if let Ok(line) = line {
            let line = line.trim().as_bytes();
            if !line.is_empty() {
                result.push(line.to_vec());
            } else {
                return result
            }
        }
    }
    result
}

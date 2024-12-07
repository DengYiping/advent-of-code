use std::io::{self, BufRead};

pub fn read() -> Vec<Vec<u8>> {
    let mut result: Vec<Vec<u8>> = vec![];
    for line in io::stdin().lock().lines() {
        if let Ok(line) = line {
            let line = line.trim();
            if line.len() == 0 {
                return result
            }
            result.push(line.as_bytes().iter().map(|x| x.clone()).collect())
        }
    }
    result
}

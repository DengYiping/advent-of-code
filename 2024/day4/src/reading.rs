use std::io::{self, BufRead};

pub fn read() -> Vec<String> {
    let mut all = vec![];
    for line in io::stdin().lock().lines() {
        if let Ok(line) = line {
            let line = line.trim();
            if line.len() == 0 {
                return all
            }
            all.push(line.into())
        }
    }
    all
}

pub fn read_as_matrix() -> Vec<Vec<u8>> {
    read().iter().map(|line| line.as_bytes().iter().map(|x| x.clone()).collect()).collect()
}

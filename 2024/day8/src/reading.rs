use std::io::{stdin, BufRead};

pub fn read() -> Vec<Vec<Option<u8>>> {
    let mut result: Vec<Vec<Option<u8>>> = vec![];
    for line in stdin().lock().lines() {
        if let Ok(line) = line {
            let line = line.trim();
            if line.len() == 0 {
                return result;
            }
            result.push(
                line.as_bytes()
                    .iter()
                    .map(|&x| if x == ('.' as u8) { None } else { Some(x) })
                    .collect(),
            )
        }
    }
    result
}

use std::{
    collections::{HashMap, HashSet},
    io::{self, BufRead},
};

pub struct Reading {
    pub should_not_shown_before: HashMap<u64, HashSet<u64>>,
    pub cases: Vec<Vec<u64>>,
}

pub fn read() -> Reading {
    let mut should_not_shown_before: HashMap<u64, HashSet<u64>> = HashMap::new();
    let mut cases: Vec<Vec<u64>> = vec![];
    let mut read_rule = true;

    for line in io::stdin().lock().lines() {
        if let Ok(line) = line {
            let line = line.trim();
            if line.len() == 0 {
                if read_rule {
                    read_rule = false
                } else {
                    break;
                }
            } else {
                if read_rule {
                    let parts: Vec<&str> = line.split("|").collect();
                    assert_eq!(parts.len(), 2);
                    let first: u64 = parts[0].parse().unwrap();
                    let second: u64 = parts[1].parse().unwrap();
                    match should_not_shown_before.get_mut(&first) {
                        Some(val) => {
                            val.insert(second);
                        }
                        None => {
                            let mut set: HashSet<u64> = HashSet::new();
                            set.insert(second);
                            should_not_shown_before.insert(first, set);
                        }
                    }
                } else {
                    cases.push(line.split(",").map(|x| x.parse::<u64>().unwrap()).collect())
                }
            }
        }
    }

    Reading {
        should_not_shown_before,
        cases,
    }
}

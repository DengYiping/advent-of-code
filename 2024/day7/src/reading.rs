use std::io::{self, BufRead};

use nom::{
    self,
    bytes::complete::tag,
    character::{self, complete::space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub struct Equation {
    pub target: i64,
    pub nums: Vec<i64>,
}

pub fn read() -> Vec<Equation> {
    // Read from stdin and parse
    let mut equations = Vec::new();
    for line in io::stdin().lock().lines() {
        if let Ok(line) = line {
            let line = line.trim();
            if line.is_empty() {
                break;
            }
            let (_, equation) = parse_line(&line).expect("Failed to parse line");
            equations.push(equation);
        }
    }
    equations
}

fn parse_line(input: &str) -> IResult<&str, Equation> {
    let (input, (target, nums)) = separated_pair(
        character::complete::i64,
        tag(": "),
        separated_list1(space1, character::complete::i64),
    )(input)?;
    Ok((input, Equation { target, nums }))
}

use std::{
    collections::HashMap,
    io::{self, stdin, BufRead},
};

use nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::complete::anychar,
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

use regex::Regex;

pub fn day1_1() {
    let mut left: Vec<i64> = vec![];
    let mut right: Vec<i64> = vec![];
    for line in stdin().lock().lines() {
        if let Ok(line) = line {
            let nums: Vec<i64> = line.split(" ").flat_map(|x| x.parse::<i64>()).collect();
            if nums.len() != 2 {
                break;
            }
            left.push(nums[0]);
            right.push(nums[1]);
        }
    }

    left.sort();
    right.sort();

    assert!(left.len() == right.len());
    let sum: i64 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();
    println!("{}", sum);
}

pub fn day1_2() {
    let mut left: Vec<i64> = vec![];
    let mut right: HashMap<i64, i64> = HashMap::new();

    for line in stdin().lock().lines() {
        if let Ok(line) = line {
            if line.trim().len() == 0 {
                break;
            }
            let nums: Vec<i64> = line.split(" ").flat_map(|x| x.parse::<i64>()).collect();
            left.push(nums[0]);
            right.entry(nums[1]).and_modify(|x| *x += 1).or_insert(1);
        }
    }

    let mut sum = 0;
    for num in left {
        sum += match right.get(&num) {
            Some(v) => v * num,
            None => 0,
        }
    }

    print!("Result: {}", sum)
}

pub fn day2_1() {
    let mut num_safe_reports = 0;
    for line in stdin().lock().lines() {
        if let Ok(line) = line {
            if line.trim().len() == 0 {
                break;
            }
            let nums: Vec<i64> = line.split(" ").flat_map(|x| x.parse::<i64>()).collect();
            let mut prev_diff = 0;
            let mut safe = true;

            for i in 1..nums.len() {
                let diff = nums[i] - nums[i - 1];
                if diff.abs() > 3 || diff.abs() < 1 {
                    safe = false
                }
                if prev_diff < 0 && diff > 0 {
                    safe = false
                }

                if prev_diff > 0 && diff < 0 {
                    safe = false
                }
                prev_diff = diff
            }
            if safe {
                num_safe_reports += 1
            }
        }
    }

    print!("Result: {}", num_safe_reports)
}

#[derive(Clone, Debug)]
enum Day2SafeState {
    Inc,
    Dec,
    None,
}

fn day2_2_is_safe(
    arr: &Vec<i64>,
    idx: usize,
    prev: i64,
    state: Day2SafeState,
    has_unsafe: bool,
) -> bool {
    if idx >= arr.len() {
        return true;
    }

    let diff = arr[idx] - prev;
    if diff.abs() < 1 || diff.abs() > 3 {
        if has_unsafe {
            return false;
        }
        return day2_2_is_safe(arr, idx + 1, prev, state, true);
    }

    match state.clone() {
        Day2SafeState::Inc if diff < 0 => {
            if has_unsafe {
                return false;
            } else {
                day2_2_is_safe(arr, idx + 1, prev, state, true)
            }
        }
        Day2SafeState::Dec if diff > 0 => {
            if has_unsafe {
                false
            } else {
                day2_2_is_safe(arr, idx + 1, prev, state, true)
            }
        }
        _ => day2_2_is_safe(
            arr,
            idx + 1,
            arr[idx],
            if diff > 0 {
                Day2SafeState::Inc
            } else {
                Day2SafeState::Dec
            },
            has_unsafe,
        ),
    }
}

pub fn day2_2() {
    let mut num_safe_reports = 0;
    for line in stdin().lock().lines() {
        if let Ok(line) = line {
            if line.trim().len() == 0 {
                break;
            }
            let nums: Vec<i64> = line.split(" ").flat_map(|x| x.parse::<i64>()).collect();
            if day2_2_is_safe(&nums, 1, nums[0], Day2SafeState::None, false) {
                num_safe_reports += 1
            } else if nums.len() > 1 && day2_2_is_safe(&nums, 2, nums[1], Day2SafeState::None, true)
            {
                num_safe_reports += 1
            }
        }
    }

    println!("Result: {}", num_safe_reports)
}

pub fn day3_1() {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let all_stdin = io::read_to_string(stdin()).expect("Unable to read inputs");
    let total: u64 = re
        .captures_iter(&all_stdin)
        .map(|caps| {
            caps.get(1).unwrap().as_str().parse::<u64>().unwrap()
                * caps.get(2).unwrap().as_str().parse::<u64>().unwrap()
        })
        .sum();
    println!("{}", total)
}

#[derive(Clone, Copy)]
enum Day32Instruction {
    Mul(u32, u32),
    Dont,
    Do,
}

enum Day32ShouldProcess {
    Do,
    Dont,
}

fn day3_2_parse_mul(input: &str) -> IResult<&str, Day32Instruction> {
    let (input, _) = tag("mul")(input)?;
    let (input, pair) = delimited(
        tag("("),
        separated_pair(
            nom::character::complete::u32,
            tag(","),
            nom::character::complete::u32,
        ),
        tag(")"),
    )(input)?;
    Ok((input, Day32Instruction::Mul(pair.0, pair.1)))
}

fn day3_2_parse_instruction(input: &str) -> IResult<&str, Day32Instruction> {
    alt((
        value(Day32Instruction::Dont, tag("don't()")),
        value(Day32Instruction::Do, tag("do()")),
        day3_2_parse_mul,
    ))(input)
}

fn day3_2_parse(input: &str) -> IResult<&str, Vec<Day32Instruction>> {
    many1(many_till(anychar, day3_2_parse_instruction).map(|(_, ins)| ins))(input)
}

fn day3_2_process(input: &str) -> u32 {
    let (_, instructions) = day3_2_parse(input).unwrap();
    instructions
        .iter()
        .fold(
            (Day32ShouldProcess::Do, 0),
            |(process, acc), ins| match ins {
                Day32Instruction::Mul(a, b) => match process {
                    Day32ShouldProcess::Do => (process, acc + a * b),
                    Day32ShouldProcess::Dont => (process, acc),
                },
                Day32Instruction::Do => (Day32ShouldProcess::Do, acc),
                Day32Instruction::Dont => (Day32ShouldProcess::Dont, acc),
            },
        )
        .1
}

pub fn day3_2() {
    let all_stdin = io::read_to_string(stdin()).expect("Unable to read inputs");
    println!("Result: {}", day3_2_process(&all_stdin))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_3_2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(day3_2_process(input), 48)
    }
}

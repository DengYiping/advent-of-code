use advent_of_code_2024_day_7::reading::read;

fn can_balance(acc:i64, target: i64, num: &[i64]) -> bool {
    match num {
        [head, tail @ ..] => {
            can_balance(head * acc, target, tail) || can_balance(head + acc, target, tail)
        },
        _ => acc == target
    }
}

fn main() {
    let equations = read();
    let result: i64 = equations.iter()
        .filter(|equation| can_balance(equation.nums[0], equation.target, &equation.nums[1..]))
        .map(|equation| equation.target)
        .sum();
    println!("Result: {}", result);
}


use std::{collections::HashMap, str::FromStr};

enum Op {
    Num(i64),
    Add(String, String),
    Sub(String, String),
    Mul(String, String),
    Div(String, String),
}

#[derive(Debug, PartialEq)]
enum OpErr {
    MissingOperand,
    MissingOperator,
    UnknownOperation,
}

impl FromStr for Op {
    type Err = OpErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(number) = s.parse::<i64>() {
            Ok(Op::Num(number))
        } else {
            let parts = s.split_whitespace().collect::<Vec<_>>();
            let lhs = parts.get(0).ok_or(OpErr::MissingOperand)?.to_string();
            let op = parts.get(1).ok_or(OpErr::MissingOperator)?;
            let rhs = parts.get(2).ok_or(OpErr::MissingOperand)?.to_string();
            match *op {
                "+" => Ok(Op::Add(lhs, rhs)),
                "-" => Ok(Op::Sub(lhs, rhs)),
                "*" => Ok(Op::Mul(lhs, rhs)),
                "/" => Ok(Op::Div(lhs, rhs)),
                _ => Err(OpErr::UnknownOperation),
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum MonkeyErr {
    MissingColon,
    OperationError(OpErr),
    MissingMonkey,
}

fn parse_input(input: &str) -> Result<HashMap<&str, Op>, MonkeyErr> {
    input
        .lines()
        .map(|line| {
            let (name, op) = line.split_once(": ").ok_or(MonkeyErr::MissingColon)?;
            Ok((
                name,
                op.parse::<Op>()
                    .or_else(|e| Err(MonkeyErr::OperationError(e)))?,
            ))
        })
        .collect()
}

fn get_result(target: &str, monkeys: &HashMap<&str, Op>) -> Result<i64, MonkeyErr> {
    match monkeys.get(target).ok_or(MonkeyErr::MissingMonkey)? {
        Op::Add(lhs, rhs) => Ok(get_result(lhs, monkeys)? + get_result(rhs, monkeys)?),
        Op::Sub(lhs, rhs) => Ok(get_result(lhs, monkeys)? - get_result(rhs, monkeys)?),
        Op::Mul(lhs, rhs) => Ok(get_result(lhs, monkeys)? * get_result(rhs, monkeys)?),
        Op::Div(lhs, rhs) => Ok(get_result(lhs, monkeys)? / get_result(rhs, monkeys)?),
        Op::Num(number) => Ok(*number),
    }
}

fn solve(input: &str) -> Result<i64, MonkeyErr> {
    let monkeys = parse_input(input)?;
    return get_result("root", &monkeys);
}

fn main() {
    let result = solve(include_str!("input.txt"));
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_result() {
        let result = solve(include_str!("example.txt"));
        assert_eq!(Ok(152), result);
    }
}

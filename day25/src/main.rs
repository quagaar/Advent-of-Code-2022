use std::{
    error::Error,
    fmt::{Display, Write},
    iter::Sum,
    str::FromStr,
};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct BalancedQuinary(i64);

impl From<i64> for BalancedQuinary {
    fn from(value: i64) -> Self {
        BalancedQuinary(value)
    }
}

impl Sum for BalancedQuinary {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(0, |a, b| a + b.0).into()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ParseBalancedQuinaryError {
    UnknownDigit(char),
}

impl Display for ParseBalancedQuinaryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownDigit(ch) => write!(f, "Unknown digit: {}", ch),
        }
    }
}

impl Error for ParseBalancedQuinaryError {}

impl FromStr for BalancedQuinary {
    type Err = ParseBalancedQuinaryError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.chars()
            .try_fold(0, |mut acc, ch| {
                acc *= 5;
                match ch {
                    '2' => Ok(acc + 2),
                    '1' => Ok(acc + 1),
                    '0' => Ok(acc),
                    '-' => Ok(acc - 1),
                    '=' => Ok(acc - 2),
                    _ => Err(ParseBalancedQuinaryError::UnknownDigit(ch)),
                }
            })
            .map(BalancedQuinary::from)
    }
}

impl Display for BalancedQuinary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut digits = vec![];
        let mut remaining = self.0;

        while remaining != 0 {
            match remaining % 5 {
                0 => digits.push('0'),
                1 => digits.push('1'),
                2 => digits.push('2'),
                3 => {
                    digits.push('=');
                    remaining += 5;
                }
                4 => {
                    digits.push('-');
                    remaining += 5;
                }
                -1 => digits.push('-'),
                -2 => digits.push('='),
                -3 => {
                    digits.push('2');
                    remaining -= 5;
                }
                -4 => {
                    digits.push('1');
                    remaining -= 5;
                }
                _ => panic!("Unexpected remainder: {}", remaining),
            }
            remaining /= 5;
        }

        if digits.is_empty() {
            f.write_char('0')
        } else {
            digits.into_iter().rev().try_for_each(|c| f.write_char(c))
        }
    }
}

fn solve(input: &str) -> BalancedQuinary {
    input
        .lines()
        .map(|s| BalancedQuinary::from_str(s).unwrap())
        .sum()
}

fn main() {
    let result = solve(include_str!("input.txt"));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_result() {
        let result = solve(include_str!("example.txt"));
        assert_eq!("2=-1=0", result.to_string());
    }

    #[test]
    fn puzzle_result() {
        let result = solve(include_str!("input.txt"));
        assert_eq!("20=212=1-12=200=00-1", result.to_string());
    }

    #[test]
    fn parse_balanced_quinary() {
        assert_eq!(BalancedQuinary::from_str("0"), Ok(BalancedQuinary::from(0)));
        assert_eq!(BalancedQuinary::from_str("1"), Ok(BalancedQuinary::from(1)));
        assert_eq!(BalancedQuinary::from_str("2"), Ok(BalancedQuinary::from(2)));
        assert_eq!(
            BalancedQuinary::from_str("1="),
            Ok(BalancedQuinary::from(3))
        );
        assert_eq!(
            BalancedQuinary::from_str("1-"),
            Ok(BalancedQuinary::from(4))
        );
        assert_eq!(
            BalancedQuinary::from_str("10"),
            Ok(BalancedQuinary::from(5))
        );
        assert_eq!(
            BalancedQuinary::from_str("11"),
            Ok(BalancedQuinary::from(6))
        );
        assert_eq!(
            BalancedQuinary::from_str("12"),
            Ok(BalancedQuinary::from(7))
        );
        assert_eq!(
            BalancedQuinary::from_str("2="),
            Ok(BalancedQuinary::from(8))
        );
        assert_eq!(
            BalancedQuinary::from_str("2-"),
            Ok(BalancedQuinary::from(9))
        );
        assert_eq!(
            BalancedQuinary::from_str("20"),
            Ok(BalancedQuinary::from(10))
        );
        assert_eq!(
            BalancedQuinary::from_str("1=0"),
            Ok(BalancedQuinary::from(15))
        );
        assert_eq!(
            BalancedQuinary::from_str("1-0"),
            Ok(BalancedQuinary::from(20))
        );
        assert_eq!(
            BalancedQuinary::from_str("1=11-2"),
            Ok(BalancedQuinary::from(2022))
        );
        assert_eq!(
            BalancedQuinary::from_str("1-0---0"),
            Ok(BalancedQuinary::from(12345))
        );
        assert_eq!(
            BalancedQuinary::from_str("1121-1110-1=0"),
            Ok(BalancedQuinary::from(314159265))
        );
        assert_eq!(
            BalancedQuinary::from_str("--=-1---01-20"),
            Ok(BalancedQuinary::from(-314159265))
        );
    }

    #[test]
    fn format_balanced_quinary() {
        assert_eq!(BalancedQuinary::from(0).to_string(), "0");
        assert_eq!(BalancedQuinary::from(1).to_string(), "1");
        assert_eq!(BalancedQuinary::from(2).to_string(), "2");
        assert_eq!(BalancedQuinary::from(3).to_string(), "1=");
        assert_eq!(BalancedQuinary::from(4).to_string(), "1-");
        assert_eq!(BalancedQuinary::from(5).to_string(), "10");
        assert_eq!(BalancedQuinary::from(6).to_string(), "11");
        assert_eq!(BalancedQuinary::from(7).to_string(), "12");
        assert_eq!(BalancedQuinary::from(8).to_string(), "2=");
        assert_eq!(BalancedQuinary::from(9).to_string(), "2-");
        assert_eq!(BalancedQuinary::from(10).to_string(), "20");
        assert_eq!(BalancedQuinary::from(15).to_string(), "1=0");
        assert_eq!(BalancedQuinary::from(20).to_string(), "1-0");
        assert_eq!(BalancedQuinary::from(2022).to_string(), "1=11-2");
        assert_eq!(BalancedQuinary::from(12345).to_string(), "1-0---0");
        assert_eq!(
            BalancedQuinary::from(314159265).to_string(),
            "1121-1110-1=0"
        );
        assert_eq!(BalancedQuinary::from(-1).to_string(), "-");
        assert_eq!(BalancedQuinary::from(-2).to_string(), "=");
        assert_eq!(BalancedQuinary::from(-3).to_string(), "-2");
        assert_eq!(BalancedQuinary::from(-4).to_string(), "-1");
        assert_eq!(BalancedQuinary::from(-5).to_string(), "-0");
        assert_eq!(BalancedQuinary::from(-6).to_string(), "--");
        assert_eq!(BalancedQuinary::from(-7).to_string(), "-=");
        assert_eq!(BalancedQuinary::from(-8).to_string(), "=2");
        assert_eq!(BalancedQuinary::from(-9).to_string(), "=1");
        assert_eq!(BalancedQuinary::from(-10).to_string(), "=0");
        assert_eq!(BalancedQuinary::from(-15).to_string(), "-20");
        assert_eq!(BalancedQuinary::from(-20).to_string(), "-10");
        assert_eq!(BalancedQuinary::from(-2022).to_string(), "-2--1=");
        assert_eq!(BalancedQuinary::from(-12345).to_string(), "-101110");
        assert_eq!(
            BalancedQuinary::from(-314159265).to_string(),
            "--=-1---01-20"
        );
    }
}

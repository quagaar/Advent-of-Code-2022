fn parse_balanced_quinary(line: &str) -> i64 {
    line.chars().fold(0, |mut acc, ch| {
        acc *= 5;
        match ch {
            '2' => acc += 2,
            '1' => acc += 1,
            '0' => {}
            '-' => acc -= 1,
            '=' => acc -= 2,
            _ => panic!("Unknown digit {}", ch),
        }
        acc
    })
}

fn to_balanced_quinary(number: i64) -> String {
    let mut digits = vec![];
    let mut remaining = number;

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
        return String::from("0");
    } else {
        return String::from_iter(digits.into_iter().rev());
    }
}

fn solve(input: &str) -> String {
    let numbers = input
        .lines()
        .map(parse_balanced_quinary)
        .collect::<Vec<_>>();
    return to_balanced_quinary(numbers.into_iter().sum());
}

fn main() {
    let result = solve(include_str!("input.txt"));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn example_result() {
        let result = solve(include_str!("example.txt"));
        assert_eq!("2=-1=0", result);
    }

    #[test]
    fn puzzle_result() {
        let result = solve(include_str!("input.txt"));
        assert_eq!("20=212=1-12=200=00-1", result);
    }

    #[test_case("0" => 0)]
    #[test_case("1" => 1)]
    #[test_case("2" => 2)]
    #[test_case("1=" => 3)]
    #[test_case("1-" => 4)]
    #[test_case("10" => 5)]
    #[test_case("11" => 6)]
    #[test_case("12" => 7)]
    #[test_case("2=" => 8)]
    #[test_case("2-" => 9)]
    #[test_case("20" => 10)]
    #[test_case("1=0" => 15)]
    #[test_case("1-0" => 20)]
    #[test_case("1=11-2" => 2022)]
    #[test_case("1-0---0" => 12345)]
    #[test_case("1121-1110-1=0" => 314159265)]
    #[test_case("--=-1---01-20" => -314159265)]
    fn test_parse_balanced_quinary(line: &str) -> i64 {
        parse_balanced_quinary(line)
    }

    #[test_case(0 => "0")]
    #[test_case(1 => "1")]
    #[test_case(2 => "2")]
    #[test_case(3 => "1=")]
    #[test_case(4 => "1-")]
    #[test_case(5 => "10")]
    #[test_case(6 => "11")]
    #[test_case(7 => "12")]
    #[test_case(8 => "2=")]
    #[test_case(9 => "2-")]
    #[test_case(10 => "20")]
    #[test_case(15 => "1=0")]
    #[test_case(20 => "1-0")]
    #[test_case(2022 => "1=11-2")]
    #[test_case(12345 => "1-0---0")]
    #[test_case(314159265 => "1121-1110-1=0")]
    #[test_case(-1 => "-"; "convert minus 1")]
    #[test_case(-2 => "="; "convert minus 2")]
    #[test_case(-3 => "-2"; "convert minus 3")]
    #[test_case(-4 => "-1"; "convert minus 4")]
    #[test_case(-5 => "-0"; "convert minus 5")]
    #[test_case(-6 => "--"; "convert minus 6")]
    #[test_case(-7 => "-="; "convert minus 7")]
    #[test_case(-8 => "=2"; "convert minus 8")]
    #[test_case(-9 => "=1"; "convert minus 9")]
    #[test_case(-10 => "=0"; "convert minus 10")]
    #[test_case(-15 => "-20"; "convert minus 15")]
    #[test_case(-20 => "-10"; "convert minus 20")]
    #[test_case(-2022 => "-2--1="; "convert minus 2022")]
    #[test_case(-12345 => "-101110"; "convert minus 12345")]
    #[test_case(-314159265 => "--=-1---01-20"; "convert minus 314159265")]
    fn test_to_balanced_quinary(number: i64) -> String {
        to_balanced_quinary(number)
    }
}

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
        String::from("0")
    } else {
        String::from_iter(digits.into_iter().rev())
    }
}

fn solve(input: &str) -> String {
    to_balanced_quinary(input.lines().map(parse_balanced_quinary).sum())
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
        assert_eq!("2=-1=0", result);
    }

    #[test]
    fn puzzle_result() {
        let result = solve(include_str!("input.txt"));
        assert_eq!("20=212=1-12=200=00-1", result);
    }

    #[test]
    fn test_parse_balanced_quinary() {
        assert_eq!(parse_balanced_quinary("0"), 0);
        assert_eq!(parse_balanced_quinary("1"), 1);
        assert_eq!(parse_balanced_quinary("2"), 2);
        assert_eq!(parse_balanced_quinary("1="), 3);
        assert_eq!(parse_balanced_quinary("1-"), 4);
        assert_eq!(parse_balanced_quinary("10"), 5);
        assert_eq!(parse_balanced_quinary("11"), 6);
        assert_eq!(parse_balanced_quinary("12"), 7);
        assert_eq!(parse_balanced_quinary("2="), 8);
        assert_eq!(parse_balanced_quinary("2-"), 9);
        assert_eq!(parse_balanced_quinary("20"), 10);
        assert_eq!(parse_balanced_quinary("1=0"), 15);
        assert_eq!(parse_balanced_quinary("1-0"), 20);
        assert_eq!(parse_balanced_quinary("1=11-2"), 2022);
        assert_eq!(parse_balanced_quinary("1-0---0"), 12345);
        assert_eq!(parse_balanced_quinary("1121-1110-1=0"), 314159265);
        assert_eq!(parse_balanced_quinary("--=-1---01-20"), -314159265);
    }

    #[test]
    fn test_to_balanced_quinary() {
        assert_eq!(to_balanced_quinary(0), "0");
        assert_eq!(to_balanced_quinary(1), "1");
        assert_eq!(to_balanced_quinary(2), "2");
        assert_eq!(to_balanced_quinary(3), "1=");
        assert_eq!(to_balanced_quinary(4), "1-");
        assert_eq!(to_balanced_quinary(5), "10");
        assert_eq!(to_balanced_quinary(6), "11");
        assert_eq!(to_balanced_quinary(7), "12");
        assert_eq!(to_balanced_quinary(8), "2=");
        assert_eq!(to_balanced_quinary(9), "2-");
        assert_eq!(to_balanced_quinary(10), "20");
        assert_eq!(to_balanced_quinary(15), "1=0");
        assert_eq!(to_balanced_quinary(20), "1-0");
        assert_eq!(to_balanced_quinary(2022), "1=11-2");
        assert_eq!(to_balanced_quinary(12345), "1-0---0");
        assert_eq!(to_balanced_quinary(314159265), "1121-1110-1=0");
        assert_eq!(to_balanced_quinary(-1), "-");
        assert_eq!(to_balanced_quinary(-2), "=");
        assert_eq!(to_balanced_quinary(-3), "-2");
        assert_eq!(to_balanced_quinary(-4), "-1");
        assert_eq!(to_balanced_quinary(-5), "-0");
        assert_eq!(to_balanced_quinary(-6), "--");
        assert_eq!(to_balanced_quinary(-7), "-=");
        assert_eq!(to_balanced_quinary(-8), "=2");
        assert_eq!(to_balanced_quinary(-9), "=1");
        assert_eq!(to_balanced_quinary(-10), "=0");
        assert_eq!(to_balanced_quinary(-15), "-20");
        assert_eq!(to_balanced_quinary(-20), "-10");
        assert_eq!(to_balanced_quinary(-2022), "-2--1=");
        assert_eq!(to_balanced_quinary(-12345), "-101110");
        assert_eq!(to_balanced_quinary(-314159265), "--=-1---01-20");
    }
}

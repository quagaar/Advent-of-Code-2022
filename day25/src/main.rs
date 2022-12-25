fn parse_snafu_number(line: &str) -> i64 {
    line.chars()
        .rev()
        .fold((1, 0), |(multiple, mut total), ch| {
            match ch {
                '2' => total += 2 * multiple,
                '1' => total += multiple,
                '0' => {}
                '-' => total -= multiple,
                '=' => total -= 2 * multiple,
                _ => panic!("Unknown digit {}", ch),
            }
            (multiple * 5, total)
        })
        .1
}

fn to_snafu_number(number: i64) -> String {
    let mut digits = vec![];
    let mut remaining = number;

    while remaining > 0 {
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
            _ => panic!("Unexpected remainder: {}", remaining),
        }
        remaining /= 5;
    }

    return String::from_iter(digits.into_iter().rev());
}

fn solve(input: &str) -> String {
    let numbers = input.lines().map(parse_snafu_number).collect::<Vec<_>>();
    return to_snafu_number(numbers.into_iter().sum());
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
}

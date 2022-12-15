fn get_score(line: &str) -> i32 {
    match line {
        "A X" => 1 + 3,
        "A Y" => 2 + 6,
        "A Z" => 3 + 0,

        "B X" => 1 + 0,
        "B Y" => 2 + 3,
        "B Z" => 3 + 6,

        "C X" => 1 + 6,
        "C Y" => 2 + 0,
        "C Z" => 3 + 3,

        _ => panic!("Unexpected line: {}", line),
    }
}

fn solve(input: &str) -> i32 {
    input.lines().map(get_score).sum()
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
        assert_eq!(15, result);
    }
}

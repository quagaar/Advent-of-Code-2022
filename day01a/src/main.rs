fn total_calories(elf: &str) -> i32 {
    elf.lines().map(|item| item.parse::<i32>().unwrap()).sum()
}

fn solve(input: &str) -> Option<i32> {
    input.split("\n\n").map(total_calories).max()
}

fn main() {
    let result = solve(include_str!("input.txt"));
    println!("{}", result.unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_result() {
        let result = solve(include_str!("example.txt"));
        assert_eq!(Some(24000), result);
    }

    #[test]
    fn puzzle_result() {
        let result = solve(include_str!("input.txt"));
        assert_eq!(Some(72511), result);
    }
}

fn solve(input: &str) -> i32 {
    input
        .split("\n\n")
        .map(|elf| elf.lines().map(|item| item.parse::<i32>().unwrap()).sum())
        .max()
        .unwrap()
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
        assert_eq!(24000, result);
    }

    #[test]
    fn puzzle_result() {
        let result = solve(include_str!("input.txt"));
        assert_eq!(72511, result);
    }
}

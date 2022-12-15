fn solve(input: &str) -> i32 {
    let mut elf_calories: Vec<i32> = input
        .split("\n\n")
        .map(|elf| elf.lines().map(|item| item.parse::<i32>().unwrap()).sum())
        .collect();

    elf_calories.sort_unstable_by(|a, b| b.cmp(a));

    elf_calories.iter().take(3).sum()
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
        assert_eq!(45000, result);
    }
}

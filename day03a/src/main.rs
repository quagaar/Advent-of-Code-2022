fn split_compartments(line: &str) -> (&str, &str) {
    let split = line.len() / 2;
    (&line[..split], &line[split..])
}

fn get_common_item_type((a, b): (&str, &str)) -> char {
    a.chars()
        .find_map(|ch1| b.chars().find(|ch2| ch1.eq(ch2)))
        .unwrap_or_else(|| panic!("Common item type not found: ({}, {})", a, b))
}

fn get_priority(item_type: char) -> i32 {
    match item_type {
        'a'..='z' => 1 + (item_type as i32 - 'a' as i32),
        'A'..='Z' => 27 + (item_type as i32 - 'A' as i32),
        _ => panic!("Unexpected item type: {}", item_type),
    }
}

fn solve(input: &str) -> i32 {
    input
        .lines()
        .map(split_compartments)
        .map(get_common_item_type)
        .map(get_priority)
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
        assert_eq!(157, result);
    }

    #[test]
    fn puzzle_result() {
        let result = solve(include_str!("input.txt"));
        assert_eq!(7701, result);
    }
}

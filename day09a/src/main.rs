use std::collections::HashSet;

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    match (head.0 - tail.0, head.1 - tail.1) {
        (2, _) => (tail.0 + 1, head.1),
        (-2, _) => (tail.0 - 1, head.1),
        (_, 2) => (head.0, tail.1 + 1),
        (_, -2) => (head.0, tail.1 - 1),
        _ => tail,
    }
}

fn solve(input: &str) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);

    for line in input.lines() {
        let (direction, steps) = line.split_once(' ').unwrap();
        let steps: i32 = steps.parse().unwrap();

        for _ in 0..steps {
            match direction {
                "U" => head.1 += 1,
                "D" => head.1 -= 1,
                "L" => head.0 += 1,
                "R" => head.0 -= 1,
                _ => panic!("Unknown direction: {}", line),
            }
            tail = move_tail(head, tail);
            visited.insert(tail);
        }
    }

    visited.len()
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
        assert_eq!(13, result);
    }

    #[test]
    fn puzzle_result() {
        let result = solve(include_str!("input.txt"));
        assert_eq!(6197, result);
    }
}

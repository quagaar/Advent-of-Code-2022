fn build_stacks(desc: &str) -> Vec<Vec<char>> {
    let mut lines = desc.lines().rev();
    let columns = lines.next().unwrap().trim().split_whitespace().count();

    let mut result: Vec<Vec<char>> = vec![vec![]; columns];

    for line in lines {
        for index in 0..columns {
            if let Some(ch) = line.chars().nth(index * 4 + 1) {
                if !ch.is_whitespace() {
                    result[index].push(ch);
                }
            }
        }
    }

    return result;
}

fn move_stacks(stacks: &mut Vec<Vec<char>>, moves: &str) {
    for line in moves.lines() {
        let mut parts = line.split_whitespace();
        assert_eq!(Some("move"), parts.next());
        let number: usize = parts.next().unwrap().parse().unwrap();
        assert_eq!(Some("from"), parts.next());
        let source: usize = parts.next().unwrap().parse().unwrap();
        assert_eq!(Some("to"), parts.next());
        let dest: usize = parts.next().unwrap().parse().unwrap();

        for _ in 0..number {
            let ch = stacks[source - 1].pop().unwrap();
            stacks[dest - 1].push(ch);
        }
    }
}

fn solve(input: &str) -> String {
    let (stacks, moves) = input.split_once("\n\n").unwrap();

    let mut stacks = build_stacks(stacks);

    move_stacks(&mut stacks, moves);

    String::from_iter(stacks.iter().map(|v| v.last().unwrap()))
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
        assert_eq!("CMZ", result);
    }
}

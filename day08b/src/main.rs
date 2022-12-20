fn distance(it: impl Iterator<Item = char> + Clone, target: char) -> usize {
    match it.clone().enumerate().find(|(_, c)| *c >= target) {
        Some((i, _)) => i + 1,
        _ => it.count(),
    }
}

fn scenic_score(trees: &Vec<&str>, x: usize, y: usize) -> usize {
    let row = trees[y];
    let target = row.chars().nth(x).unwrap();
    let before = distance(row[..x].chars().rev(), target);
    let after = distance(row.chars().skip(x + 1), target);
    let above = distance(
        trees
            .iter()
            .take(y)
            .rev()
            .map(|row| row.chars().nth(x).unwrap()),
        target,
    );
    let below = distance(
        trees
            .iter()
            .skip(y + 1)
            .map(|row| row.chars().nth(x).unwrap()),
        target,
    );
    return before * after * above * below;
}

fn solve(input: &str) -> usize {
    let trees: Vec<&str> = input.lines().collect();

    let result = trees
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, _)| (scenic_score(&trees, x, y), x, y))
                .collect::<Vec<(usize, usize, usize)>>()
        })
        .flatten()
        .max()
        .unwrap();

    if cfg!(debug_assertions) {
        println!("x={}, y={}", result.1, result.2);
    }

    return result.0;
}

fn main() {
    let result = solve(include_str!("input.txt"));
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_result() {
        let result = solve(include_str!("example.txt"));
        assert_eq!(8, result);
    }
}

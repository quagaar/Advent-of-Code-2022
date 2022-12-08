fn visible_outside(trees: &Vec<&str>, x: usize, y: usize) -> bool {
    if x == 0 || y == 0 || y + 1 == trees.len() {
        return true;
    }
    let row = trees[y];
    if x + 1 == row.len() {
        return true;
    }
    let target = row.chars().nth(x).unwrap();
    let before = &row[..x];
    if before.chars().all(|c| c < target) {
        return true;
    }
    let after = &row[x + 1..];
    if after.chars().all(|c| c < target) {
        return true;
    }
    let above = ..y;
    if trees[above]
        .iter()
        .all(|row| row.chars().nth(x).unwrap() < target)
    {
        return true;
    }
    let below = y + 1..;
    return trees[below]
        .iter()
        .all(|row| row.chars().nth(x).unwrap() < target);
}

fn main() {
    let trees: Vec<&str> = include_str!("input.txt").lines().collect();

    let result = trees
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(|(x, _)| visible_outside(&trees, x, y))
                .collect::<Vec<bool>>()
        })
        .flatten()
        .filter(|x| *x)
        .count();

    println!("{}", result);
}

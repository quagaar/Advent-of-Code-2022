fn parse_coordinate(line: &str) -> [i32; 3] {
    let parts = line
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    return [parts[0], parts[1], parts[2]];
}

fn is_adjacent(cube: &[i32; 3], other: &[i32; 3]) -> bool {
    match (cube[0] - other[0], cube[1] - other[1], cube[2] - other[2]) {
        (0, 0, 1) | (0, 0, -1) | (0, 1, 0) | (0, -1, 0) | (1, 0, 0) | (-1, 0, 0) => true,
        _ => false,
    }
}

fn solve(input: &str) -> usize {
    let cubes = input.lines().map(parse_coordinate).collect::<Vec<_>>();

    let result: usize = cubes
        .iter()
        .map(|cube| {
            6 - cubes
                .iter()
                .filter(|other| is_adjacent(cube, other))
                .count()
        })
        .sum();

    return result;
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
        assert_eq!(64, result);
    }
}

use std::collections::HashSet;

fn parse_coordinate(line: &str) -> [i32; 3] {
    let mut parts = line.split(',').map(|x| x.parse::<i32>().unwrap());
    [
        parts.next().unwrap(),
        parts.next().unwrap(),
        parts.next().unwrap(),
    ]
}

fn get_exposed_sides(cube: &[i32; 3], cubes: &HashSet<[i32; 3]>) -> usize {
    (0..=2)
        .flat_map(|axis| {
            [(axis, 1), (axis, -1)]
                .into_iter()
                .filter(|(axis, direction)| {
                    let mut pos = *cube;
                    pos[*axis] += direction;
                    !cubes.contains(&pos)
                })
        })
        .count()
}

fn solve(input: &str) -> usize {
    let cubes = input.lines().map(parse_coordinate).collect::<HashSet<_>>();

    cubes
        .iter()
        .map(|cube| get_exposed_sides(cube, &cubes))
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
        assert_eq!(64, result);
    }

    #[test]
    fn puzzle_result() {
        let result = solve(include_str!("input.txt"));
        assert_eq!(4242, result);
    }
}

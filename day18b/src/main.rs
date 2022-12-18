use std::{collections::HashSet, ops::RangeInclusive};

fn parse_coordinate(line: &str) -> [i32; 3] {
    let mut parts = line.split(",").map(|x| x.parse::<i32>().unwrap());
    return [
        parts.next().unwrap(),
        parts.next().unwrap(),
        parts.next().unwrap(),
    ];
}

fn get_cube_ranges(cubes: &HashSet<[i32; 3]>) -> [RangeInclusive<i32>; 3] {
    let min_x = cubes.iter().map(|c| c[0]).min().unwrap();
    let min_y = cubes.iter().map(|c| c[1]).min().unwrap();
    let min_z = cubes.iter().map(|c| c[2]).min().unwrap();

    let max_x = cubes.iter().map(|c| c[0]).max().unwrap();
    let max_y = cubes.iter().map(|c| c[1]).max().unwrap();
    let max_z = cubes.iter().map(|c| c[2]).max().unwrap();

    return [min_x..=max_x, min_y..=max_y, min_z..=max_z];
}

fn generate_steam(cubes: &HashSet<[i32; 3]>) -> HashSet<[i32; 3]> {
    let cube_ranges = get_cube_ranges(&cubes);

    let mut result = HashSet::new();
    let mut candidates = vec![[
        cube_ranges[0].start() - 1,
        cube_ranges[1].start() - 1,
        cube_ranges[2].start() - 1,
    ]];

    while let Some(pos) = candidates.pop() {
        if !cubes.contains(&pos) && !result.contains(&pos) {
            for axis in 0..=2 {
                if pos[axis] >= *cube_ranges[axis].start() {
                    let mut new_pos = pos.clone();
                    new_pos[axis] -= 1;
                    candidates.push(new_pos);
                }
                if pos[axis] <= *cube_ranges[axis].end() {
                    let mut new_pos = pos.clone();
                    new_pos[axis] += 1;
                    candidates.push(new_pos);
                }
            }
            result.insert(pos);
        }
    }

    return result;
}

fn get_exposed_sides(cube: &[i32; 3], steam: &HashSet<[i32; 3]>) -> usize {
    (0..=2)
        .flat_map(|axis| {
            [(axis, 1), (axis, -1)]
                .into_iter()
                .filter(|(axis, direction)| {
                    let mut pos = cube.clone();
                    pos[*axis] += direction;
                    steam.contains(&pos)
                })
        })
        .count()
}

fn solve(input: &str) -> usize {
    let cubes = input.lines().map(parse_coordinate).collect::<HashSet<_>>();
    let steam = generate_steam(&cubes);
    let result: usize = cubes
        .iter()
        .map(|cube| get_exposed_sides(cube, &steam))
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
        assert_eq!(58, result);
    }
}

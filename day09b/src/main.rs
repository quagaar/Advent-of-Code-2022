use std::collections::HashSet;

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    match (head.0 - tail.0, head.1 - tail.1) {
        (0, 2) => (tail.0, tail.1 + 1),
        (0, -2) => (tail.0, tail.1 - 1),
        (2, 0) => (tail.0 + 1, tail.1),
        (-2, 0) => (tail.0 - 1, tail.1),
        (2, 1) | (2, 2) | (1, 2) => (tail.0 + 1, tail.1 + 1),
        (2, -1) | (2, -2) | (1, -2) => (tail.0 + 1, tail.1 - 1),
        (-2, 1) | (-2, 2) | (-1, 2) => (tail.0 - 1, tail.1 + 1),
        (-2, -1) | (-2, -2) | (-1, -2) => (tail.0 - 1, tail.1 - 1),
        _ => tail,
    }
}

fn print_trail(visited: &HashSet<(i32, i32)>) {
    let mut iter = visited.iter();
    let (x, y) = iter.next().unwrap();
    let init = (*x, *x, *y, *y);

    let (x_min, x_max, y_min, y_max) = iter.fold(init, |acc, (x, y)| {
        (acc.0.min(*x), acc.1.max(*x), acc.2.min(*y), acc.3.max(*y))
    });

    for y in (y_min..=y_max).rev() {
        let mut row = String::new();
        for x in x_min..=x_max {
            row.push(if visited.contains(&(x, y)) { '#' } else { '.' });
        }
        println!("{}", row);
    }
}

fn solve(input: &str) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut rope = vec![(0, 0); 10];

    for line in input.lines() {
        let (direction, steps) = line.split_once(' ').unwrap();
        let steps: i32 = steps.parse().unwrap();

        for _ in 0..steps {
            match direction {
                "U" => rope[0].1 += 1,
                "D" => rope[0].1 -= 1,
                "L" => rope[0].0 -= 1,
                "R" => rope[0].0 += 1,
                _ => panic!("Unknown direction: {}", line),
            }
            for i in 1..rope.len() {
                rope[i] = move_tail(rope[i - 1], rope[i]);
            }
            visited.insert(rope[9]);
        }
    }

    if cfg!(debug_assertions) {
        print_trail(&visited);
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
    fn example1_result() {
        let result = solve(include_str!("example1.txt"));
        assert_eq!(1, result);
    }

    #[test]
    fn example2_result() {
        let result = solve(include_str!("example2.txt"));
        assert_eq!(36, result);
    }

    #[test]
    fn puzzle_result() {
        let result = solve(include_str!("input.txt"));
        assert_eq!(2562, result);
    }
}

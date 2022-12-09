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
    let x_min = visited.iter().map(|x| x.0).min().unwrap();
    let x_max = visited.iter().map(|x| x.0).max().unwrap();
    let y_min = visited.iter().map(|x| x.1).min().unwrap();
    let y_max = visited.iter().map(|x| x.1).max().unwrap();

    for y in (y_min..=y_max).rev() {
        let mut row = String::new();
        for x in x_min..=x_max {
            row.push(if visited.contains(&(x, y)) { '#' } else { '.' });
        }
        println!("{}", row);
    }
}

fn main() {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut rope = vec![(0, 0); 10];

    for line in include_str!("input.txt").lines() {
        let (direction, steps) = line.split_once(" ").unwrap();
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

    if false {
        print_trail(&visited);
    }

    let result = visited.len();

    println!("{}", result);
}

use std::collections::HashSet;

fn move_tail(head: &(i32, i32), tail: &mut (i32, i32)) {
    match head.0 - tail.0 {
        2 => {
            tail.0 += 1;
            tail.1 = head.1;
        }
        -2 => {
            tail.0 -= 1;
            tail.1 = head.1;
        }
        _ => (),
    }
    match head.1 - tail.1 {
        2 => {
            tail.1 += 1;
            tail.0 = head.0;
        }
        -2 => {
            tail.1 -= 1;
            tail.0 = head.0;
        }
        _ => (),
    }
}

fn solve(input: &str) -> usize {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut head = (0, 0);
    let mut tail = (0, 0);

    for line in input.lines() {
        let (direction, steps) = line.split_once(" ").unwrap();
        let steps: i32 = steps.parse().unwrap();

        match direction {
            "U" => {
                for _ in 0..steps {
                    head.1 += 1;
                    move_tail(&head, &mut tail);
                    visited.insert(tail);
                }
            }
            "D" => {
                for _ in 0..steps {
                    head.1 -= 1;
                    move_tail(&head, &mut tail);
                    visited.insert(tail);
                }
            }
            "L" => {
                for _ in 0..steps {
                    head.0 += 1;
                    move_tail(&head, &mut tail);
                    visited.insert(tail);
                }
            }
            "R" => {
                for _ in 0..steps {
                    head.0 -= 1;
                    move_tail(&head, &mut tail);
                    visited.insert(tail);
                }
            }
            _ => panic!("Unknown direction: {}", line),
        }
    }

    let result = visited.len();

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
        assert_eq!(13, result);
    }
}

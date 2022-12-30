use std::collections::VecDeque;

struct Location {
    height: i32,
    visited: bool,
}

struct Map {
    start: (usize, usize),
    target: (usize, usize),
    locations: Vec<Vec<Location>>,
}

impl Map {
    fn get_location(&mut self, position: (usize, usize)) -> Option<&mut Location> {
        self.locations
            .get_mut(position.1)
            .and_then(|v| v.get_mut(position.0))
    }
}

fn parse_map(input: &str) -> Map {
    let mut start = (0, 0);
    let mut target = (0, 0);
    let locations = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| match ch {
                    'S' => {
                        start = (x, y);
                        Location {
                            height: 0,
                            visited: true,
                        }
                    }
                    'E' => {
                        target = (x, y);
                        Location {
                            height: 25,
                            visited: false,
                        }
                    }
                    _ => Location {
                        height: ch as i32 - 'a' as i32,
                        visited: false,
                    },
                })
                .collect()
        })
        .collect();
    Map {
        start,
        target,
        locations,
    }
}

#[derive(Copy, Clone)]
struct State {
    steps: i32,
    height: i32,
    position: (usize, usize),
}

fn get_neighbors(position: (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = vec![(position.0 + 1, position.1), (position.0, position.1 + 1)];

    if position.0 > 0 {
        result.push((position.0 - 1, position.1))
    }

    if position.1 > 0 {
        result.push((position.0, position.1 - 1))
    }

    result
}

fn shortest_path(mut map: Map) -> Option<i32> {
    let mut queue = VecDeque::new();
    queue.push_back(State {
        steps: 0,
        height: 0,
        position: map.start,
    });

    while let Some(State {
        steps,
        height,
        position,
    }) = queue.pop_front()
    {
        if position == map.target {
            return Some(steps);
        }

        for next in get_neighbors(position) {
            if let Some(location) = map.get_location(next) {
                if !location.visited && location.height - height <= 1 {
                    location.visited = true;
                    queue.push_back(State {
                        steps: steps + 1,
                        height: location.height,
                        position: next,
                    });
                }
            }
        }
    }

    None
}

fn solve(input: &str) -> Option<i32> {
    let map = parse_map(input);
    shortest_path(map)
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
        assert_eq!(Some(31), result);
    }

    #[test]
    fn puzzle_result() {
        let result = solve(include_str!("input.txt"));
        assert_eq!(Some(330), result);
    }
}

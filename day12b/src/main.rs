use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone)]
struct Location {
    height: i32,
    visited: bool,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    steps: i32,
    height: i32,
    position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then_with(|| self.height.cmp(&other.height))
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn get_neighbours(position: (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = vec![(position.0 + 1, position.1), (position.0, position.1 + 1)];

    if position.0 > 0 {
        result.push((position.0 - 1, position.1))
    }

    if position.1 > 0 {
        result.push((position.0, position.1 - 1))
    }

    return result;
}

fn shortest_path(
    mut map: Vec<Vec<Location>>,
    start: (usize, usize),
    target: (usize, usize),
) -> Option<i32> {
    let mut heap = BinaryHeap::new();
    heap.push(State {
        steps: 0,
        height: 0,
        position: start,
    });

    while let Some(State {
        steps,
        height,
        position,
    }) = heap.pop()
    {
        if position == target {
            return Some(steps);
        }

        for next in get_neighbours(position) {
            if let Some(location) = map.get_mut(next.1).map(|v| v.get_mut(next.0)).flatten() {
                if !location.visited && location.height - height <= 1 {
                    location.visited = true;
                    heap.push(State {
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

fn main() {
    let mut target = (0, 0);
    let map: Vec<Vec<Location>> = include_str!("input.txt")
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, ch)| match ch {
                    'S' => Location {
                        height: 0,
                        visited: false,
                    },
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

    let result = (0..map.len())
        .map(|y| {
            (0..map[y].len())
                .filter(|&x| map[y][x].height == 0)
                .map(|x| (x, y))
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .filter_map(|start| shortest_path(map.clone(), start, target))
        .min();

    println!("{:?}", result);
}

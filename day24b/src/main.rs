use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy)]
struct Blizzard {
    direction: Direction,
    position: (usize, usize),
}

impl Blizzard {
    fn next(&self, width: usize, height: usize) -> Blizzard {
        Blizzard {
            direction: self.direction,
            position: match self.direction {
                Direction::Left => {
                    if self.position.0 > 1 {
                        (self.position.0 - 1, self.position.1)
                    } else {
                        (width, self.position.1)
                    }
                }
                Direction::Right => {
                    if self.position.0 < width {
                        (self.position.0 + 1, self.position.1)
                    } else {
                        (1, self.position.1)
                    }
                }
                Direction::Up => {
                    if self.position.1 > 1 {
                        (self.position.0, self.position.1 - 1)
                    } else {
                        (self.position.0, height)
                    }
                }
                Direction::Down => {
                    if self.position.1 < height {
                        (self.position.0, self.position.1 + 1)
                    } else {
                        (self.position.0, 1)
                    }
                }
            },
        }
    }
}

struct Valley {
    height: usize,
    width: usize,
    entrance: (usize, usize),
    exit: (usize, usize),
    initial_blizzards: Vec<Blizzard>,
}

impl Valley {
    fn from(input: &str) -> Valley {
        let height = input.lines().count() - 2;
        let width = input.lines().next().unwrap().chars().count() - 2;
        let entrance_x = input.lines().next().unwrap().find('.').unwrap();
        let exit_x = input.lines().rev().next().unwrap().find('.').unwrap();
        return Valley {
            height,
            width,
            entrance: (entrance_x, 0),
            exit: (exit_x, height + 1),
            initial_blizzards: input
                .lines()
                .enumerate()
                .skip(1)
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter_map(move |(x, ch)| match ch {
                            '<' => Some(Blizzard {
                                direction: Direction::Left,
                                position: (x, y),
                            }),
                            '>' => Some(Blizzard {
                                direction: Direction::Right,
                                position: (x, y),
                            }),
                            '^' => Some(Blizzard {
                                direction: Direction::Up,
                                position: (x, y),
                            }),
                            'v' => Some(Blizzard {
                                direction: Direction::Down,
                                position: (x, y),
                            }),
                            _ => None,
                        })
                })
                .collect(),
        };
    }

    fn update_blizzards(&self, blizzards: &[Blizzard]) -> Vec<Blizzard> {
        blizzards
            .iter()
            .map(|blizzard| blizzard.next(self.width, self.height))
            .collect()
    }

    fn build_occupation_grid(&self, blizzards: &Vec<Blizzard>) -> Vec<Vec<usize>> {
        let mut grid = vec![];
        grid.reserve(self.height + 2);
        grid.push(vec![1_usize; self.width + 2]);
        for y in 1..=self.height {
            grid.push(vec![0; self.width + 2]);
            grid[y][0] = 1;
            grid[y][self.width + 1] = 1;
        }
        grid.push(vec![1_usize; self.width + 2]);
        grid[self.entrance.1][self.entrance.0] = 0;
        grid[self.exit.1][self.exit.0] = 0;
        for b in blizzards {
            grid[b.position.1][b.position.0] += 1;
        }
        grid
    }
}

fn get_next_steps(pos: &(usize, usize), grid: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    let mut next_steps = vec![];

    if grid[pos.1][pos.0] == 0 {
        next_steps.push(*pos);
    }

    if pos.1 > 0 && grid[pos.1 - 1][pos.0] == 0 {
        next_steps.push((pos.0, pos.1 - 1));
    }
    if pos.1 + 1 < grid.len() && grid[pos.1 + 1][pos.0] == 0 {
        next_steps.push((pos.0, pos.1 + 1));
    }
    if grid[pos.1][pos.0 - 1] == 0 {
        next_steps.push((pos.0 - 1, pos.1));
    }
    if grid[pos.1][pos.0 + 1] == 0 {
        next_steps.push((pos.0 + 1, pos.1));
    }

    next_steps
}

fn get_steps(
    start: (usize, usize),
    end: (usize, usize),
    valley: &Valley,
    blizzards: &mut Vec<Blizzard>,
) -> usize {
    let mut possible_positions = HashSet::new();
    possible_positions.insert(start);
    for steps in 1.. {
        *blizzards = valley.update_blizzards(blizzards);
        let grid = valley.build_occupation_grid(blizzards);
        possible_positions = possible_positions
            .iter()
            .flat_map(|pos| get_next_steps(pos, &grid))
            .collect();
        if possible_positions.contains(&end) {
            return steps;
        }
    }
    panic!("Loop ended with no path found!!!");
}

fn solve(input: &str) -> usize {
    let valley = Valley::from(input);
    let mut blizzards = valley.initial_blizzards.clone();
    get_steps(valley.entrance, valley.exit, &valley, &mut blizzards)
        + get_steps(valley.exit, valley.entrance, &valley, &mut blizzards)
        + get_steps(valley.entrance, valley.exit, &valley, &mut blizzards)
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
        assert_eq!(54, result);
    }

    #[test]
    fn puzzle_result() {
        let result = solve(include_str!("input.txt"));
        assert_eq!(974, result);
    }
}

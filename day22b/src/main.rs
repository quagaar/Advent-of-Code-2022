use std::collections::HashMap;

#[derive(Copy, Clone)]
enum Direction {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction {
    fn right(&self) -> Direction {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn left(&self) -> Direction {
        match self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        }
    }
}

enum PathStep {
    Forward(usize),
    TurnLeft,
    TurnRight,
}

fn parse_path(path: &str) -> Vec<PathStep> {
    let mut result = vec![];
    let mut digits = String::new();
    for ch in path.trim().chars() {
        match ch {
            'L' => {
                let number = digits.parse().unwrap();
                result.push(PathStep::Forward(number));
                result.push(PathStep::TurnLeft);
                digits.clear();
            }
            'R' => {
                let number = digits.parse().unwrap();
                result.push(PathStep::Forward(number));
                result.push(PathStep::TurnRight);
                digits.clear();
            }
            _ => digits.push(ch),
        }
    }
    if !digits.is_empty() {
        let number = digits.parse().unwrap();
        result.push(PathStep::Forward(number));
    }
    return result;
}

#[derive(PartialEq)]
enum MapSquare {
    Wall,
    Open,
    Void,
}

fn parse_map(map: &str) -> Vec<Vec<MapSquare>> {
    map.lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '#' => MapSquare::Wall,
                    '.' => MapSquare::Open,
                    _ => MapSquare::Void,
                })
                .collect()
        })
        .collect()
}

#[derive(Clone, Copy, PartialEq)]
enum CubeEdge {
    Straight(usize, usize),
    TurnLeft(usize, usize),
    TurnRight(usize, usize),
    Reverse(usize, usize),
}

struct Cube {
    size: usize,
    net: Vec<Vec<Option<[CubeEdge; 4]>>>,
}

impl Cube {
    fn net_pos(&self, pos: Location) -> (usize, usize) {
        (pos.row / self.size, pos.column / self.size)
    }

    fn right(&self, pos: Location) -> Location {
        let (row, col) = self.net_pos(pos);
        let edge = self.net[row][col].unwrap()[0];
        return match edge {
            CubeEdge::Straight(r, c) => Location {
                row: r * self.size + pos.row % self.size,
                column: c * self.size,
                direction: Direction::Right,
            },
            CubeEdge::TurnLeft(r, c) => Location {
                row: (r + 1) * self.size - 1,
                column: pos.row % self.size + c * self.size,
                direction: Direction::Up,
            },
            CubeEdge::TurnRight(r, c) => Location {
                row: r * self.size,
                column: (c + 1) * self.size - 1 - pos.row % self.size,
                direction: Direction::Down,
            },
            CubeEdge::Reverse(r, c) => Location {
                row: (r + 1) * self.size - 1 - pos.row % self.size,
                column: (c + 1) * self.size - 1,
                direction: Direction::Left,
            },
        };
    }

    fn down(&self, pos: Location) -> Location {
        let (row, col) = self.net_pos(pos);
        let edge = self.net[row][col].unwrap()[1];
        return match edge {
            CubeEdge::Straight(r, c) => Location {
                row: r * self.size,
                column: c * self.size + pos.column % self.size,
                direction: Direction::Down,
            },
            CubeEdge::TurnLeft(r, c) => Location {
                row: (r + 1) * self.size - 1 - pos.column % self.size,
                column: c * self.size,
                direction: Direction::Right,
            },
            CubeEdge::TurnRight(r, c) => Location {
                row: r * self.size + pos.column % self.size,
                column: (c + 1) * self.size - 1,
                direction: Direction::Left,
            },
            CubeEdge::Reverse(r, c) => Location {
                row: (r + 1) * self.size - 1,
                column: (c + 1) * self.size - 1 - pos.column % self.size,
                direction: Direction::Up,
            },
        };
    }

    fn left(&self, pos: Location) -> Location {
        let (row, col) = self.net_pos(pos);
        let edge = self.net[row][col].unwrap()[2];
        return match edge {
            CubeEdge::Straight(r, c) => Location {
                row: r * self.size + pos.row % self.size,
                column: (c + 1) * self.size - 1,
                direction: Direction::Left,
            },
            CubeEdge::TurnLeft(r, c) => Location {
                row: r * self.size,
                column: pos.row % self.size + c * self.size,
                direction: Direction::Down,
            },
            CubeEdge::TurnRight(r, c) => Location {
                row: (r + 1) * self.size - 1,
                column: (c + 1) * self.size - 1 - pos.row % self.size,
                direction: Direction::Up,
            },
            CubeEdge::Reverse(r, c) => Location {
                row: (r + 1) * self.size - 1 - pos.row % self.size,
                column: c * self.size,
                direction: Direction::Right,
            },
        };
    }

    fn up(&self, pos: Location) -> Location {
        let (row, col) = self.net_pos(pos);
        let edge = self.net[row][col].unwrap()[3];
        return match edge {
            CubeEdge::Straight(r, c) => Location {
                row: (r + 1) * self.size - 1,
                column: c * self.size + pos.column % self.size,
                direction: Direction::Up,
            },
            CubeEdge::TurnLeft(r, c) => Location {
                row: (r + 1) * self.size - 1 - pos.column % self.size,
                column: (c + 1) * self.size - 1,
                direction: Direction::Left,
            },
            CubeEdge::TurnRight(r, c) => Location {
                row: r * self.size + pos.column % self.size,
                column: c * self.size,
                direction: Direction::Right,
            },
            CubeEdge::Reverse(r, c) => Location {
                row: r * self.size,
                column: (c + 1) * self.size - 1 - pos.column % self.size,
                direction: Direction::Down,
            },
        };
    }
}

#[derive(Clone, Copy)]
struct Location {
    row: usize,
    column: usize,
    direction: Direction,
}

fn find_start(map: &Vec<Vec<MapSquare>>) -> Location {
    Location {
        row: 0,
        column: map[0]
            .iter()
            .enumerate()
            .find(|(_, x)| **x == MapSquare::Open)
            .unwrap()
            .0,
        direction: Direction::Right,
    }
}

fn find_right(pos: &Location, map: &Vec<Vec<MapSquare>>, cube: &Cube) -> Location {
    let row = &map[pos.row];
    return match row.get(pos.column + 1) {
        Some(MapSquare::Void) | None => cube.right(*pos),
        Some(_) => Location {
            row: pos.row,
            column: pos.column + 1,
            direction: pos.direction,
        },
    };
}

fn find_down(pos: &Location, map: &Vec<Vec<MapSquare>>, cube: &Cube) -> Location {
    if let Some(row) = map.get(pos.row + 1) {
        if let Some(square) = row.get(pos.column) {
            if *square != MapSquare::Void {
                return Location {
                    row: pos.row + 1,
                    column: pos.column,
                    direction: pos.direction,
                };
            }
        }
    }
    return cube.down(*pos);
}

fn find_left(pos: &Location, map: &Vec<Vec<MapSquare>>, cube: &Cube) -> Location {
    let row = &map[pos.row];
    if pos.column > 0 {
        if let Some(square) = row.get(pos.column - 1) {
            if *square != MapSquare::Void {
                return Location {
                    row: pos.row,
                    column: pos.column - 1,
                    direction: pos.direction,
                };
            }
        }
    }
    return cube.left(*pos);
}

fn find_up(pos: &Location, map: &Vec<Vec<MapSquare>>, cube: &Cube) -> Location {
    if pos.row > 0 {
        if let Some(square) = map[pos.row - 1].get(pos.column) {
            if *square != MapSquare::Void {
                return Location {
                    row: pos.row - 1,
                    column: pos.column,
                    direction: pos.direction,
                };
            }
        }
    }
    return cube.up(*pos);
}

fn follow_path(path: Vec<PathStep>, map: Vec<Vec<MapSquare>>, cube: Cube) -> Location {
    let mut pos = find_start(&map);
    let mut cache = HashMap::new();
    cache.insert((pos.row, pos.column), pos.direction);
    for step in path {
        match step {
            PathStep::TurnRight => {
                pos.direction = pos.direction.right();
                cache.insert((pos.row, pos.column), pos.direction);
            }
            PathStep::TurnLeft => {
                pos.direction = pos.direction.left();
                cache.insert((pos.row, pos.column), pos.direction);
            }
            PathStep::Forward(num) => {
                for _ in 0..num {
                    let next = match pos.direction {
                        Direction::Right => find_right(&pos, &map, &cube),
                        Direction::Down => find_down(&pos, &map, &cube),
                        Direction::Left => find_left(&pos, &map, &cube),
                        Direction::Up => find_up(&pos, &map, &cube),
                    };
                    if map[next.row][next.column] == MapSquare::Open {
                        pos = next;
                        cache.insert((pos.row, pos.column), pos.direction);
                    } else {
                        break;
                    }
                }
            }
        }
    }
    if cfg!(debug_assertions) {
        for (r, row) in map.iter().enumerate() {
            for (c, sq) in row.iter().enumerate() {
                match cache.get(&(r, c)) {
                    Some(Direction::Right) => print!(">"),
                    Some(Direction::Down) => print!("V"),
                    Some(Direction::Left) => print!("<"),
                    Some(Direction::Up) => print!("^"),
                    None => match sq {
                        MapSquare::Void => print!(" "),
                        MapSquare::Wall => print!("#"),
                        MapSquare::Open => print!("."),
                    },
                }
            }
            print!("\n");
        }
    }
    return pos;
}

fn solve(input: &str, cube: Cube) -> usize {
    let (map, path) = input.split_once("\n\n").unwrap();
    let map = parse_map(map);
    let path = parse_path(path);
    let end = follow_path(path, map, cube);
    return 1000 * (end.row + 1) + 4 * (end.column + 1) + end.direction as usize;
}

fn main() {
    let cube = Cube {
        size: 50,
        net: vec![
            vec![
                None,
                Some([
                    CubeEdge::Straight(0, 2),
                    CubeEdge::Straight(1, 1),
                    CubeEdge::Reverse(2, 0),
                    CubeEdge::TurnRight(3, 0),
                ]),
                Some([
                    CubeEdge::Reverse(2, 1),
                    CubeEdge::TurnRight(1, 1),
                    CubeEdge::Straight(0, 1),
                    CubeEdge::Straight(3, 0),
                ]),
            ],
            vec![
                None,
                Some([
                    CubeEdge::TurnLeft(0, 2),
                    CubeEdge::Straight(2, 1),
                    CubeEdge::TurnLeft(2, 0),
                    CubeEdge::Straight(0, 1),
                ]),
                None,
            ],
            vec![
                Some([
                    CubeEdge::Straight(2, 1),
                    CubeEdge::Straight(3, 0),
                    CubeEdge::Reverse(0, 1),
                    CubeEdge::TurnRight(1, 1),
                ]),
                Some([
                    CubeEdge::Reverse(0, 2),
                    CubeEdge::TurnRight(3, 0),
                    CubeEdge::Straight(2, 0),
                    CubeEdge::Straight(1, 1),
                ]),
                None,
            ],
            vec![
                Some([
                    CubeEdge::TurnLeft(2, 1),
                    CubeEdge::Straight(0, 2),
                    CubeEdge::TurnLeft(0, 1),
                    CubeEdge::Straight(2, 0),
                ]),
                None,
                None,
            ],
        ],
    };
    let result = solve(include_str!("input.txt"), cube);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_result() {
        let cube = Cube {
            size: 4,
            net: vec![
                vec![
                    None,
                    None,
                    Some([
                        CubeEdge::Reverse(2, 3),
                        CubeEdge::Straight(1, 2),
                        CubeEdge::TurnLeft(1, 1),
                        CubeEdge::Straight(2, 2),
                    ]),
                    None,
                ],
                vec![
                    Some([
                        CubeEdge::Straight(1, 1),
                        CubeEdge::Reverse(2, 2),
                        CubeEdge::TurnRight(2, 3),
                        CubeEdge::Reverse(0, 2),
                    ]),
                    Some([
                        CubeEdge::Straight(1, 3),
                        CubeEdge::TurnLeft(2, 2),
                        CubeEdge::Straight(1, 0),
                        CubeEdge::TurnRight(0, 2),
                    ]),
                    Some([
                        CubeEdge::TurnRight(2, 3),
                        CubeEdge::Straight(2, 2),
                        CubeEdge::Straight(1, 1),
                        CubeEdge::Straight(0, 2),
                    ]),
                    None,
                ],
                vec![
                    None,
                    None,
                    Some([
                        CubeEdge::Straight(2, 3),
                        CubeEdge::Reverse(1, 0),
                        CubeEdge::TurnRight(1, 1),
                        CubeEdge::Straight(1, 2),
                    ]),
                    Some([
                        CubeEdge::Reverse(0, 2),
                        CubeEdge::TurnRight(1, 0),
                        CubeEdge::Straight(2, 2),
                        CubeEdge::TurnLeft(1, 2),
                    ]),
                ],
            ],
        };
        let result = solve(include_str!("example.txt"), cube);
        assert_eq!(5031, result);
    }
}

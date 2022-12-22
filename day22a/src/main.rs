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

fn find_right(pos: &Location, map: &Vec<Vec<MapSquare>>) -> Location {
    let row = &map[pos.row];
    let new_column = match row.get(pos.column + 1) {
        Some(MapSquare::Void) | None => {
            row.iter()
                .enumerate()
                .find(|(_, x)| **x != MapSquare::Void)
                .unwrap()
                .0
        }
        Some(_) => pos.column + 1,
    };
    return Location {
        row: pos.row,
        column: new_column,
        direction: pos.direction,
    };
}

fn find_down(pos: &Location, map: &Vec<Vec<MapSquare>>) -> Location {
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
    let new_row = map
        .iter()
        .enumerate()
        .find(|(_, row)| match row.get(pos.column) {
            Some(MapSquare::Void) | None => false,
            _ => true,
        })
        .unwrap()
        .0;
    return Location {
        row: new_row,
        column: pos.column,
        direction: pos.direction,
    };
}

fn find_left(pos: &Location, map: &Vec<Vec<MapSquare>>) -> Location {
    let row = &map[pos.row];
    let new_column = if pos.column > 0 {
        match row.get(pos.column - 1) {
            Some(MapSquare::Void) => row.len() - 1,
            _ => pos.column - 1,
        }
    } else {
        row.len() - 1
    };
    return Location {
        row: pos.row,
        column: new_column,
        direction: pos.direction,
    };
}

fn find_up(pos: &Location, map: &Vec<Vec<MapSquare>>) -> Location {
    let new_row = if pos.row > 0 {
        pos.row - 1
    } else {
        map.len() - 1
    };
    if let Some(square) = map[new_row].get(pos.column) {
        if *square != MapSquare::Void {
            return Location {
                row: new_row,
                column: pos.column,
                direction: pos.direction,
            };
        }
    }
    let new_row = map
        .iter()
        .enumerate()
        .rev()
        .find(|(_, row)| match row.get(pos.column) {
            Some(MapSquare::Void) | None => false,
            _ => true,
        })
        .unwrap()
        .0;
    return Location {
        row: new_row,
        column: pos.column,
        direction: pos.direction,
    };
}

fn follow_path(path: Vec<PathStep>, map: Vec<Vec<MapSquare>>) -> Location {
    let mut pos = find_start(&map);
    for step in path {
        match step {
            PathStep::TurnRight => pos.direction = pos.direction.right(),
            PathStep::TurnLeft => pos.direction = pos.direction.left(),
            PathStep::Forward(num) => {
                for _ in 0..num {
                    let next = match pos.direction {
                        Direction::Right => find_right(&pos, &map),
                        Direction::Down => find_down(&pos, &map),
                        Direction::Left => find_left(&pos, &map),
                        Direction::Up => find_up(&pos, &map),
                    };
                    if map[next.row][next.column] == MapSquare::Open {
                        pos = next;
                    } else {
                        break;
                    }
                }
            }
        }
    }
    return pos;
}

fn solve(input: &str) -> usize {
    let (map, path) = input.split_once("\n\n").unwrap();
    let map = parse_map(map);
    let path = parse_path(path);
    let end = follow_path(path, map);
    return 1000 * (end.row + 1) + 4 * (end.column + 1) + end.direction as usize;
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
        assert_eq!(6032, result);
    }
}

use std::collections::{HashMap, HashSet, VecDeque};

enum Direction {
    North,
    South,
    West,
    East,
}

fn parse_input(input: &str) -> HashSet<(isize, isize)> {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, ch)| {
                if ch == '#' {
                    Some((x as isize, y as isize))
                } else {
                    None
                }
            })
        })
        .collect::<HashSet<_>>()
}

fn print_map(round: usize, elves: &HashSet<(isize, isize)>) {
    if cfg!(debug_assertions) {
        println!("== End of round {} ==", round);
        let (first_x, first_y) = elves.iter().next().unwrap();
        let (min_x, min_y, max_x, max_y) =
            elves
                .iter()
                .fold((*first_x, *first_y, *first_x, *first_y), |acc, elf| {
                    (
                        acc.0.min(elf.0),
                        acc.1.min(elf.1),
                        acc.2.max(elf.0),
                        acc.3.max(elf.1),
                    )
                });
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if elves.contains(&(x, y)) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            print!("\n");
        }
        print!("\n");
    }
}

fn count_empty_spaces(elves: &HashSet<(isize, isize)>) -> usize {
    let (first_x, first_y) = elves.iter().next().unwrap();
    let (min_x, min_y, max_x, max_y) =
        elves
            .iter()
            .fold((*first_x, *first_y, *first_x, *first_y), |acc, elf| {
                (
                    acc.0.min(elf.0),
                    acc.1.min(elf.1),
                    acc.2.max(elf.0),
                    acc.3.max(elf.1),
                )
            });
    (min_x..=max_x)
        .flat_map(|x| (min_y..=max_y).filter(move |y| elves.get(&(x, *y)).is_none()))
        .count()
}

fn get_proposal(
    elf: &(isize, isize),
    elves: &HashSet<(isize, isize)>,
    directions: &VecDeque<Direction>,
) -> Option<(isize, isize)> {
    let nw = elves.contains(&(elf.0 - 1, elf.1 - 1));
    let n = elves.contains(&(elf.0, elf.1 - 1));
    let ne = elves.contains(&(elf.0 + 1, elf.1 - 1));
    let w = elves.contains(&(elf.0 - 1, elf.1));
    let e = elves.contains(&(elf.0 + 1, elf.1));
    let sw = elves.contains(&(elf.0 - 1, elf.1 + 1));
    let s = elves.contains(&(elf.0, elf.1 + 1));
    let se = elves.contains(&(elf.0 + 1, elf.1 + 1));
    if nw || n || ne || w || e || sw || s || se {
        for dir in directions {
            match *dir {
                Direction::North => {
                    if !nw && !n && !ne {
                        return Some((elf.0, elf.1 - 1));
                    }
                }
                Direction::South => {
                    if !sw && !s && !se {
                        return Some((elf.0, elf.1 + 1));
                    }
                }
                Direction::West => {
                    if !nw && !w && !sw {
                        return Some((elf.0 - 1, elf.1));
                    }
                }
                Direction::East => {
                    if !ne && !e && !se {
                        return Some((elf.0 + 1, elf.1));
                    }
                }
            }
        }
    }
    None
}

fn perform_round(
    elves: &HashSet<(isize, isize)>,
    directions: &VecDeque<Direction>,
) -> HashSet<(isize, isize)> {
    let proposals = elves
        .iter()
        .map(|elf| (*elf, get_proposal(elf, elves, directions)))
        .collect::<Vec<_>>();
    let mut counts: HashMap<&(isize, isize), usize> = HashMap::new();
    for (_, proposal) in proposals.iter() {
        if let Some(pos) = proposal {
            *counts.entry(&pos).or_default() += 1;
        }
    }
    return proposals
        .iter()
        .map(|(elf, proposal)| match proposal {
            Some(pos) => {
                if counts.get(pos) == Some(&1) {
                    *pos
                } else {
                    *elf
                }
            }
            None => *elf,
        })
        .collect();
}

fn solve(input: &str) -> usize {
    let mut elves = parse_input(input);
    let mut directions = VecDeque::from([
        Direction::North,
        Direction::South,
        Direction::West,
        Direction::East,
    ]);
    for round in 1..=10 {
        elves = perform_round(&elves, &directions);
        let dir = directions.pop_front().unwrap();
        directions.push_back(dir);
        print_map(round, &elves);
    }
    return count_empty_spaces(&elves);
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
        assert_eq!(110, result);
    }
}

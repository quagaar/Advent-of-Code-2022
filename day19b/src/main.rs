use std::collections::hash_map::Entry::Occupied;
use std::collections::hash_map::Entry::Vacant;
use std::thread::JoinHandle;
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    num::ParseIntError,
    str::FromStr,
    thread,
};

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

const TIME_LIMIT: i32 = 32;

#[derive(Clone, Copy, Default)]
struct RobotRecipe {
    consumes: [i32; 4],
    collects: [i32; 4],
}

struct Blueprint {
    _id: i32,
    robots: [RobotRecipe; 4],
}

fn resource_name_to_id(res_name: &str) -> usize {
    match res_name {
        "ore" => ORE,
        "clay" => CLAY,
        "obsidian" => OBSIDIAN,
        "geode" => GEODE,
        _ => panic!("Unknown resource name: {}", res_name),
    }
}

impl FromStr for Blueprint {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (bp, rest) = s.split_once(": Each ").unwrap();
        let id = bp.trim_start_matches("Blueprint ").parse()?;
        let robots = rest
            .split(" Each ")
            .map(|robot| {
                let (res_name, rest) = robot.split_once(' ').unwrap();
                let resources = rest
                    .trim_start_matches("robot costs ")
                    .trim_end_matches('.')
                    .split(" and ")
                    .map(|resource| {
                        let (number, res_type) = resource.split_once(' ').unwrap();
                        (number.parse::<i32>().unwrap(), res_type)
                    })
                    .fold([0; 4], |mut acc, (number, res_name)| {
                        acc[resource_name_to_id(res_name)] = number;
                        acc
                    });
                (res_name, resources)
            })
            .fold(
                [RobotRecipe::default(); 4],
                |mut acc, (res_name, resources)| {
                    let res_id = resource_name_to_id(res_name);
                    acc[res_id].consumes = resources;
                    acc[res_id].collects[res_id] = 1;
                    acc
                },
            );
        Ok(Blueprint { _id: id, robots })
    }
}

fn max_geodes(bp: Blueprint) -> i32 {
    #[derive(Clone, Eq, PartialEq)]
    struct State {
        minute: i32,
        robots: [i32; 4],
        resources: [i32; 4],
        building: Option<usize>,
        result: i32,
        potential: i32,
    }

    impl State {
        fn set_potential(&mut self) {
            // Result if we build a new geode robot every remaining turn
            // i.e. upper limit on what this nodes eventual result could be
            self.potential =
                self.result + ((TIME_LIMIT - self.minute) * (TIME_LIMIT - 1 - self.minute)) / 2;
        }

        fn robot_score(&self) -> i32 {
            self.robots[ORE]
                + (2 * self.robots[CLAY])
                + (4 * self.robots[OBSIDIAN])
                + (8 * self.robots[GEODE])
        }

        fn cache_key(&self) -> (i32, [i32; 3], [i32; 4]) {
            (
                self.minute,
                [
                    self.resources[ORE],
                    self.resources[CLAY],
                    self.resources[OBSIDIAN],
                ],
                self.robots,
            )
        }
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            self.result
                .cmp(&other.result)
                .then_with(|| self.robot_score().cmp(&other.robot_score()))
                .then_with(|| match (self.building, other.building) {
                    (Some(l), Some(r)) => l.cmp(&r),
                    (Some(_), None) => Ordering::Greater,
                    (None, Some(_)) => Ordering::Less,
                    _ => Ordering::Equal,
                })
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let robot_limits = [
        [CLAY, OBSIDIAN, GEODE]
            .into_iter()
            .map(|r| bp.robots[r].consumes[ORE])
            .max()
            .unwrap(),
        [ORE, OBSIDIAN, GEODE]
            .into_iter()
            .map(|r| bp.robots[r].consumes[CLAY])
            .max()
            .unwrap(),
        [ORE, CLAY, GEODE]
            .into_iter()
            .map(|r| bp.robots[r].consumes[OBSIDIAN])
            .max()
            .unwrap(),
        i32::MAX,
    ];

    let mut heap = BinaryHeap::new();
    heap.push(State {
        minute: 0,
        robots: [1, 0, 0, 0],
        resources: [0; 4],
        building: None,
        result: 0,
        potential: i32::MAX,
    });

    let mut state_cache: HashMap<(i32, [i32; 3], [i32; 4]), i32> = HashMap::new();

    let mut result = 0;

    while let Some(mut state) = heap.pop() {
        if result >= state.potential {
            continue;
        }

        state.minute += 1;

        for resource in ORE..=GEODE {
            state.resources[resource] += state.robots[resource];
        }

        if state.minute < TIME_LIMIT {
            if let Some(robot) = state.building {
                state.robots[robot] += 1;
                state.building = None;
                if robot == GEODE {
                    state.result += TIME_LIMIT - state.minute;
                    result = result.max(state.result);
                }
            }

            match state_cache.entry(state.cache_key()) {
                Occupied(mut entry) => {
                    if state.result <= *entry.get() {
                        continue;
                    } else {
                        entry.insert(state.result);
                    }
                }
                Vacant(entry) => {
                    entry.insert(state.result);
                }
            }

            if state.minute < TIME_LIMIT - 1 {
                (ORE..=GEODE)
                    .filter(|resource| robot_limits[*resource] > state.robots[*resource])
                    .map(|resource| (resource, &bp.robots[resource].consumes))
                    .filter(|(_, requires)| {
                        requires.iter().zip(state.resources).all(|(&l, r)| l <= r)
                    })
                    .for_each(|(resource, requires)| {
                        let mut new_state = state.clone();
                        new_state.building = Some(resource);
                        new_state
                            .resources
                            .iter_mut()
                            .enumerate()
                            .for_each(|(i, res)| *res -= requires[i]);
                        new_state.set_potential();
                        heap.push(new_state);
                    });
            }

            state.set_potential();
            heap.push(state);
        }
    }

    result
}

fn spawn_solver_threads(input: &str) -> Vec<JoinHandle<i32>> {
    input
        .lines()
        .map(|line| line.parse().unwrap())
        .take(3)
        .map(|bp| thread::spawn(move || max_geodes(bp)))
        .collect()
}

fn solve(input: &str) -> i32 {
    match thread::available_parallelism() {
        Ok(threads) if threads.get() >= 3 => spawn_solver_threads(input)
            .into_iter()
            .map(|handle| handle.join().unwrap())
            .product(),
        _ => input
            .lines()
            .map(|line| line.parse().unwrap())
            .map(max_geodes)
            .product(),
    }
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
        assert_eq!(56 * 62, result);
    }

    #[test]
    fn puzzle_result() {
        let result = solve(include_str!("input.txt"));
        assert_eq!(6000, result);
    }
}

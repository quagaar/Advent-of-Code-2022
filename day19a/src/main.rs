use std::{
    cmp::Ordering,
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        BinaryHeap, HashMap,
    },
    num::ParseIntError,
    str::FromStr,
    thread,
};

const ORE: usize = 0;
const CLAY: usize = 1;
const OBSIDIAN: usize = 2;
const GEODE: usize = 3;

const TIME_LIMIT: i32 = 24;

#[derive(Clone, Copy, Default)]
struct RobotRecipe {
    consumes: [i32; 4],
    collects: [i32; 4],
}

#[derive(Clone, Copy)]
struct Blueprint {
    id: i32,
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
                let (res_name, rest) = robot.split_once(" ").unwrap();
                let resources = rest
                    .trim_start_matches("robot costs ")
                    .trim_end_matches(".")
                    .split(" and ")
                    .map(|resource| {
                        let (number, res_type) = resource.split_once(" ").unwrap();
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
        Ok(Blueprint { id, robots })
    }
}

fn max_geodes(bp: &Blueprint) -> i32 {
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
                for resource in ORE..=GEODE {
                    if robot_limits[resource] > state.robots[resource] {
                        let requires = &bp.robots[resource].consumes;
                        if requires.iter().zip(state.resources).all(|(&l, r)| l <= r) {
                            let mut new_state = state.clone();
                            new_state.building = Some(resource);
                            new_state
                                .resources
                                .iter_mut()
                                .enumerate()
                                .for_each(|(i, res)| *res -= requires[i]);
                            new_state.set_potential();
                            heap.push(new_state);
                        }
                    }
                }
            }

            state.set_potential();
            heap.push(state);
        }
    }

    return result;
}

fn get_quality_level(bp: &Blueprint) -> i32 {
    return bp.id * max_geodes(bp);
}

fn thread_chunk_size(bp_number: usize) -> usize {
    let threads = thread::available_parallelism().unwrap();
    let chunk_size = bp_number / threads;
    if bp_number % threads == 0 {
        chunk_size
    } else {
        chunk_size + 1
    }
}

fn solve(input: &str) -> i32 {
    let blueprints = input
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<Blueprint>>();

    let chunks = blueprints
        .chunks(thread_chunk_size(blueprints.len()))
        .map(|chunk| chunk.into())
        .collect::<Vec<Vec<_>>>();

    let handles = chunks
        .into_iter()
        .map(|chunk| thread::spawn(move || chunk.iter().map(get_quality_level).sum::<i32>()))
        .collect::<Vec<_>>();

    return handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .sum();
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
        assert_eq!(33, result);
    }
}

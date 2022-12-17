use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
};

struct Valve {
    label: String,
    flow_rate: i32,
    links: Vec<String>,
}

impl Valve {
    fn from(line: &str) -> Valve {
        let (valve, links) = line.split_once("; ").unwrap();
        let (label, flow_rate) = valve
            .trim_start_matches("Valve ")
            .split_once(" has flow rate=")
            .unwrap();
        Valve {
            label: label.to_string(),
            flow_rate: flow_rate.parse().unwrap(),
            links: links
                .trim_start_matches("tunnel leads to valve ")
                .trim_start_matches("tunnels lead to valves ")
                .split(", ")
                .map(String::from)
                .collect(),
        }
    }
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    pressure: i32,
    potential: i32,
    minute: i32,
    p_valve: String,
    p_distance: i32,
    e_valve: String,
    e_distance: i32,
    inactive: Vec<String>,
}

impl State {
    fn activate(&mut self, valve: &String) {
        let index = self
            .inactive
            .iter()
            .enumerate()
            .find(|(_, v)| valve.eq(*v))
            .map(|(i, _)| i)
            .unwrap();
        self.inactive.remove(index);
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // (self.pressure + self.potential)
        //     .cmp(&(other.pressure + other.potential))
        //     .then_with(|| self.minute.cmp(&other.minute))

        self.pressure
            .cmp(&other.pressure)
            .then_with(|| self.potential.cmp(&other.potential))
            .then_with(|| self.minute.cmp(&other.minute))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Solver {
    valves: HashMap<String, Valve>,
    useable_valves: Vec<String>,
    paths: HashMap<String, HashMap<String, i32>>,
    min_distance: i32,
}

const TIME_AVAILABLE: i32 = 26;

fn shortest_path(v1: &String, v2: &String, valves: &HashMap<String, Valve>) -> Option<i32> {
    #[derive(Clone, Eq, PartialEq)]
    struct State {
        distance: i32,
        node: String,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other.distance.cmp(&self.distance)
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let mut heap = BinaryHeap::new();
    heap.push(State {
        distance: 0,
        node: v1.clone(),
    });

    let mut visited: HashMap<String, bool> = HashMap::new();

    while let Some(State { distance, node }) = heap.pop() {
        if node == *v2 {
            return Some(distance);
        }

        if let Some(valve) = valves.get(&node) {
            for next in valve.links.iter() {
                if let None = visited.get(next) {
                    visited.insert(next.clone(), true);
                    heap.push(State {
                        distance: distance + 1,
                        node: next.clone(),
                    });
                }
            }
        }
    }

    None
}

impl Solver {
    fn from(input: &str) -> Solver {
        let valves: HashMap<String, Valve> = input
            .lines()
            .map(Valve::from)
            .map(|v| (v.label.clone(), v))
            .collect();

        let mut useable_valves = valves
            .iter()
            .filter(|v| v.1.flow_rate > 0)
            .map(|v| v.1)
            .collect::<Vec<_>>();
        useable_valves.sort_by_key(|v| v.flow_rate);
        let useable_valves = useable_valves
            .into_iter()
            .rev()
            .map(|v| v.label.clone())
            .collect::<Vec<_>>();

        let mut paths: HashMap<String, HashMap<String, i32>> = HashMap::new();
        for (i, v1) in useable_valves.iter().enumerate() {
            for v2 in useable_valves.iter().skip(i + 1) {
                let distance = shortest_path(v1, v2, &valves).unwrap();
                paths
                    .entry(v1.clone())
                    .or_default()
                    .insert(v2.clone(), distance);
                paths
                    .entry(v2.clone())
                    .or_default()
                    .insert(v1.clone(), distance);
            }
        }

        let min_distance = *paths
            .iter()
            .flat_map(|(_, path)| path.iter().map(|(_, d)| d))
            .min()
            .unwrap();

        Solver {
            valves,
            useable_valves,
            paths,
            min_distance,
        }
    }

    fn get_potential(&self, minute: i32, inactive: &Vec<String>) -> i32 {
        inactive
            .iter()
            .enumerate()
            .flat_map(|(i, v)| {
                let steps = (i / 2) as i32;
                let distance = (self.min_distance + 1) * steps;
                let multiplier = TIME_AVAILABLE - minute - distance;
                if multiplier > 0 {
                    Some(self.valves.get(v).unwrap().flow_rate * multiplier)
                } else {
                    None
                }
            })
            .sum()
    }

    fn state_is_in_play(&self, state: &State, result: i32) -> bool {
        state.minute < TIME_AVAILABLE - 1 && result < (state.pressure + state.potential)
    }

    fn next_state_option(&self, result: i32, mut state: State) -> Option<State> {
        state.potential = self.get_potential(state.minute, &state.inactive);
        if self.state_is_in_play(&state, result) {
            Some(state)
        } else {
            None
        }
    }

    fn solve(&self) -> i32 {
        let mut result = 0;
        let mut heap = BinaryHeap::new();

        let start = String::from("AA");
        let mut init_valves = self
            .useable_valves
            .iter()
            .map(|valve| (shortest_path(&start, valve, &self.valves).unwrap(), valve))
            .collect::<Vec<_>>();
        init_valves.sort_unstable_by_key(|(distance, _)| *distance);
        for (i, (p_distance, p_valve)) in init_valves.iter().enumerate() {
            for (e_distance, e_valve) in init_valves.iter().skip(i + 1) {
                let &minute = p_distance.min(e_distance);
                heap.push(State {
                    pressure: 0,
                    potential: self.get_potential(minute, &self.useable_valves),
                    minute: minute,
                    p_valve: p_valve.to_string(),
                    p_distance: p_distance - minute,
                    e_valve: e_valve.to_string(),
                    e_distance: e_distance - minute,
                    inactive: self.useable_valves.clone(),
                });
            }
        }

        while let Some(mut state) = heap.pop() {
            if self.state_is_in_play(&state, result) {
                state.minute += 1;

                if state.p_distance == 0 {
                    let p_valve = self.valves.get(&state.p_valve).unwrap();
                    state.pressure += p_valve.flow_rate * (TIME_AVAILABLE - state.minute);
                    if state.pressure > result {
                        result = state.pressure;
                    }
                    state.activate(&p_valve.label);
                }
                state.p_distance -= 1;

                if state.e_distance == 0 {
                    let e_valve = self.valves.get(&state.e_valve).unwrap();
                    state.pressure += e_valve.flow_rate * (TIME_AVAILABLE - state.minute);
                    if state.pressure > result {
                        result = state.pressure;
                    }
                    state.activate(&e_valve.label);
                }
                state.e_distance -= 1;

                if state.inactive.len() > 0 {
                    match (state.p_distance, state.e_distance) {
                        (-1, -1) => {
                            let p_paths = self.paths.get(&state.p_valve).unwrap();
                            let e_paths = self.paths.get(&state.e_valve).unwrap();
                            if state.inactive.len() == 1 {
                                let valve = state.inactive.get(0).unwrap();
                                let &p_distance = p_paths.get(valve).unwrap();
                                let &e_distance = e_paths.get(valve).unwrap();
                                let distance = p_distance.min(e_distance);
                                if let Some(new_state) = self.next_state_option(
                                    result,
                                    State {
                                        pressure: state.pressure,
                                        potential: 0,
                                        minute: state.minute + distance,
                                        p_valve: valve.clone(),
                                        p_distance: p_distance - distance,
                                        e_valve: valve.clone(),
                                        e_distance: e_distance - distance,
                                        inactive: state.inactive.clone(),
                                    },
                                ) {
                                    heap.push(new_state);
                                }
                            } else {
                                heap.extend(state.inactive.iter().flat_map(|p_valve| {
                                    state
                                        .inactive
                                        .iter()
                                        .filter(|e_valve| p_valve.ne(*e_valve))
                                        .filter_map(|e_valve| {
                                            let &p_distance = p_paths.get(p_valve).unwrap();
                                            let &e_distance = e_paths.get(e_valve).unwrap();
                                            let distance = p_distance.min(e_distance);
                                            self.next_state_option(
                                                result,
                                                State {
                                                    pressure: state.pressure,
                                                    potential: 0,
                                                    minute: state.minute + distance,
                                                    p_valve: p_valve.clone(),
                                                    p_distance: p_distance - distance,
                                                    e_valve: e_valve.clone(),
                                                    e_distance: e_distance - distance,
                                                    inactive: state.inactive.clone(),
                                                },
                                            )
                                        })
                                }));
                            }
                        }
                        (-1, _) => {
                            if state.inactive.len() == 1 {
                                state.minute += state.e_distance;
                                state.potential = self.get_potential(state.minute, &state.inactive);
                                state.p_distance = i32::MAX;
                                state.e_distance = 0;
                                heap.push(state);
                            } else {
                                let p_paths = self.paths.get(&state.p_valve).unwrap();
                                heap.extend(
                                    state
                                        .inactive
                                        .iter()
                                        .filter(|p_valve| state.e_valve.ne(*p_valve))
                                        .filter_map(|p_valve| {
                                            let &p_distance = p_paths.get(p_valve).unwrap();
                                            let distance = p_distance.min(state.e_distance);
                                            self.next_state_option(
                                                result,
                                                State {
                                                    pressure: state.pressure,
                                                    potential: 0,
                                                    minute: state.minute + distance,
                                                    p_valve: p_valve.clone(),
                                                    p_distance: p_distance - distance,
                                                    e_valve: state.e_valve.clone(),
                                                    e_distance: state.e_distance - distance,
                                                    inactive: state.inactive.clone(),
                                                },
                                            )
                                        }),
                                );
                            }
                        }
                        (_, -1) => {
                            if state.inactive.len() == 1 {
                                state.minute += state.p_distance;
                                state.potential = self.get_potential(state.minute, &state.inactive);
                                state.p_distance = 0;
                                state.e_distance = i32::MAX;
                                heap.push(state);
                            } else {
                                let e_paths = self.paths.get(&state.e_valve).unwrap();
                                heap.extend(
                                    state
                                        .inactive
                                        .iter()
                                        .filter(|e_valve| state.p_valve.ne(*e_valve))
                                        .filter_map(|e_valve| {
                                            let &e_distance = e_paths.get(e_valve).unwrap();
                                            let distance = e_distance.min(state.p_distance);
                                            self.next_state_option(
                                                result,
                                                State {
                                                    pressure: state.pressure,
                                                    potential: 0,
                                                    minute: state.minute + distance,
                                                    p_valve: state.p_valve.clone(),
                                                    p_distance: state.p_distance - distance,
                                                    e_valve: e_valve.clone(),
                                                    e_distance: e_distance - distance,
                                                    inactive: state.inactive.clone(),
                                                },
                                            )
                                        }),
                                );
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        result
    }
}

fn main() {
    let solver = Solver::from(include_str!("input.txt"));
    println!("{:?}", solver.solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_result() {
        let solver = Solver::from(include_str!("example.txt"));
        assert_eq!(1707, solver.solve());
    }
}

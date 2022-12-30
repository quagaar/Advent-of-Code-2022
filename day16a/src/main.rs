use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, VecDeque},
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
    valve: String,
    inactive: Vec<String>,
}

impl State {
    fn activate(&mut self, valve: &String) {
        self.inactive.retain(|v| v != valve);
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

const TIME_AVAILABLE: i32 = 30;

fn shortest_path(v1: &str, v2: &str, valves: &HashMap<String, Valve>) -> Option<i32> {
    struct State<'a> {
        distance: i32,
        node: &'a str,
    }

    let mut queue = VecDeque::new();
    queue.push_back(State {
        distance: 0,
        node: v1,
    });

    let mut visited: HashMap<&str, bool> = HashMap::new();

    while let Some(State { distance, node }) = queue.pop_front() {
        if node == v2 {
            return Some(distance);
        }

        if let Some(valve) = valves.get(node) {
            for next in valve.links.iter() {
                if visited.get(next.as_str()).is_none() {
                    visited.insert(next, true);
                    queue.push_back(State {
                        distance: distance + 1,
                        node: next,
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

    fn get_potential(&self, minute: i32, inactive: &[String]) -> i32 {
        inactive
            .iter()
            .enumerate()
            .flat_map(|(i, v)| {
                let distance = (self.min_distance + 1) * i as i32;
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

    fn potential_state_option(
        &self,
        result: i32,
        pressure: i32,
        minute: i32,
        inactive: &[String],
        valve: &str,
        distance: i32,
    ) -> Option<State> {
        let minute = minute + distance;
        if minute < TIME_AVAILABLE - 1 {
            let potential = self.get_potential(minute, inactive);
            if result < (pressure + potential) {
                return Some(State {
                    pressure,
                    potential,
                    minute,
                    valve: valve.to_owned(),
                    inactive: inactive.to_owned(),
                });
            }
        }
        None
    }

    fn create_initial_heap(&self) -> BinaryHeap<State> {
        let mut heap = BinaryHeap::new();
        let start = String::from("AA");
        for valve in self.useable_valves.iter() {
            let distance = shortest_path(&start, valve, &self.valves).unwrap();
            heap.push(State {
                pressure: 0,
                potential: self.get_potential(distance, &self.useable_valves),
                minute: distance,
                valve: valve.clone(),
                inactive: self.useable_valves.clone(),
            });
        }

        heap
    }

    fn solve(&self) -> i32 {
        let mut result = 0;
        let mut heap = self.create_initial_heap();

        while let Some(mut state) = heap.pop() {
            if self.state_is_in_play(&state, result) {
                state.minute += 1;

                let valve = self.valves.get(&state.valve).unwrap();
                state.pressure += valve.flow_rate * (TIME_AVAILABLE - state.minute);
                if state.pressure > result {
                    result = state.pressure;
                }
                state.activate(&valve.label);

                let next_path = self.paths.get(&state.valve).unwrap();
                heap.extend(state.inactive.iter().filter_map(|valve| {
                    let distance = next_path.get(valve).unwrap();
                    self.potential_state_option(
                        result,
                        state.pressure,
                        state.minute,
                        &state.inactive,
                        valve,
                        *distance,
                    )
                }));
            }
        }

        result
    }
}

fn main() {
    let solver = Solver::from(include_str!("input.txt"));
    println!("{}", solver.solve());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_result() {
        let solver = Solver::from(include_str!("example.txt"));
        assert_eq!(1651, solver.solve());
    }

    #[test]
    fn puzzle_result() {
        let solver = Solver::from(include_str!("input.txt"));
        assert_eq!(2183, solver.solve());
    }
}

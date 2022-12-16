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
    valve: String,
    inactive: Vec<String>,
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
}

const TIME_AVAILABLE: i32 = 30;

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

        Solver {
            valves,
            useable_valves,
            paths,
        }
    }

    fn get_potential(&self, minute: i32, inactive: &Vec<String>) -> i32 {
        inactive
            .iter()
            .map(|v| self.valves.get(v).unwrap().flow_rate * (TIME_AVAILABLE - minute))
            .sum::<i32>()
    }

    fn solve(&self) -> i32 {
        let mut result = 0;
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

        while let Some(mut state) = heap.pop() {
            state.minute += 1;
            if state.minute < TIME_AVAILABLE && result < state.pressure + state.potential {
                let valve = self.valves.get(&state.valve).unwrap();
                state.pressure += valve.flow_rate * (TIME_AVAILABLE - state.minute);
                if state.pressure > result {
                    result = state.pressure;
                }

                let index = state
                    .inactive
                    .iter()
                    .enumerate()
                    .find(|(_, v)| **v == state.valve)
                    .map(|(i, _)| i)
                    .unwrap();
                state.inactive.remove(index);

                let next_path = self.paths.get(&state.valve).unwrap();
                heap.extend(state.inactive.iter().map(|valve| {
                    let distance = next_path.get(valve).unwrap();
                    State {
                        pressure: state.pressure,
                        potential: self.get_potential(state.minute + distance, &state.inactive),
                        minute: state.minute + distance,
                        valve: valve.clone(),
                        inactive: state.inactive.clone(),
                    }
                }));
            }
        }

        result
    }
}

fn solve(input: &str) -> i32 {
    let solver = Solver::from(input);
    return solver.solve();
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
        assert_eq!(1651, result);
    }
}

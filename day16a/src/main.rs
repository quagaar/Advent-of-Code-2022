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
        (self.pressure + self.potential)
            .cmp(&(other.pressure + other.potential))
            .then_with(|| self.minute.cmp(&other.minute))

        // self.pressure
        //     .cmp(&other.pressure)
        //     .then_with(|| self.potential.cmp(&other.potential))
        //     .then_with(|| self.minute.cmp(&other.minute))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Solver {
    valves: HashMap<String, Valve>,
}

impl Solver {
    fn from(input: &str) -> Solver {
        Solver {
            valves: input
                .lines()
                .map(Valve::from)
                .map(|v| (v.label.clone(), v))
                .collect(),
        }
    }

    fn get_usable_valves(&self) -> Vec<String> {
        let mut usable = self
            .valves
            .iter()
            .filter(|v| v.1.flow_rate > 0)
            .map(|v| v.1)
            .collect::<Vec<_>>();
        usable.sort_by_key(|v| v.flow_rate);
        usable.into_iter().rev().map(|v| v.label.clone()).collect()
    }

    fn get_potential(&self, minute: i32, inactive: &Vec<String>) -> i32 {
        inactive
            .iter()
            .enumerate()
            .map(|(i, v)| self.valves.get(v).unwrap().flow_rate * (30 - minute - (2 * i) as i32))
            .sum::<i32>()
    }

    fn solve(&self) -> i32 {
        let useable_valves: Vec<String> = self.get_usable_valves();

        let mut result = 0;
        let mut heap = BinaryHeap::new();
        heap.push(State {
            pressure: 0,
            potential: self.get_potential(0, &useable_valves),
            minute: 0,
            valve: String::from("AA"),
            inactive: useable_valves,
        });

        while let Some(mut state) = heap.pop() {
            if state.minute == 29 {
                if result < state.pressure {
                    result = state.pressure;
                }
            }
            if state.minute < 29 && result < state.pressure + state.potential {
                state.minute += 1;
                let valve = self.valves.get(&state.valve).unwrap();

                let potential = self.get_potential(state.minute + 1, &state.inactive);
                if result < state.pressure + potential
                    && (state.inactive.len() > 1 || !state.inactive.contains(&state.valve))
                {
                    heap.extend(valve.links.iter().map(|v| State {
                        pressure: state.pressure,
                        potential: potential,
                        minute: state.minute,
                        valve: v.clone(),
                        inactive: state.inactive.clone(),
                    }));
                }

                if let Some(index) = state
                    .inactive
                    .iter()
                    .enumerate()
                    .find(|(_, v)| **v == state.valve)
                    .map(|(i, _)| i)
                {
                    state.pressure += valve.flow_rate * (30 - state.minute);
                    state.inactive.remove(index);
                    if state.inactive.is_empty() {
                        if state.pressure > result {
                            result = state.pressure;
                        }
                    } else {
                        state.potential = self.get_potential(state.minute + 2, &state.inactive);
                        if result < state.pressure + state.potential {
                            heap.push(state);
                        }
                    }
                }
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

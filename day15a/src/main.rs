use std::{collections::HashSet, ops::RangeInclusive};

struct Sensor {
    position: (i32, i32),
    beacon: (i32, i32),
}

impl Sensor {
    fn from(line: &str) -> Sensor {
        let (position, beacon) = line.split_once(": ").unwrap();
        let position: (i32, i32) = position
            .trim_start_matches("Sensor at ")
            .split_once(", ")
            .map(|(x, y)| {
                (
                    x.trim_start_matches("x=").parse().unwrap(),
                    y.trim_start_matches("y=").parse().unwrap(),
                )
            })
            .unwrap();
        let beacon: (i32, i32) = beacon
            .trim_start_matches("closest beacon is at ")
            .split_once(", ")
            .map(|(x, y)| {
                (
                    x.trim_start_matches("x=").parse().unwrap(),
                    y.trim_start_matches("y=").parse().unwrap(),
                )
            })
            .unwrap();
        return Sensor { position, beacon };
    }

    fn beacon_distance(&self) -> u32 {
        self.position.0.abs_diff(self.beacon.0) + self.position.1.abs_diff(self.beacon.1)
    }

    fn detection_range(&self, target_row: i32) -> Option<RangeInclusive<i32>> {
        let row_distance = self.position.1.abs_diff(target_row);
        let beacon_distance = self.beacon_distance();
        if row_distance <= beacon_distance {
            let spread = (beacon_distance - row_distance) as i32;
            let start = self.position.0 - spread;
            let end = self.position.0 + spread;
            return Some(start..=end);
        } else {
            return None;
        }
    }
}

fn solve(input: &str, target_row: i32) -> usize {
    let sensors: Vec<Sensor> = input.lines().map(|line| Sensor::from(line)).collect();

    let positions = sensors
        .iter()
        .filter_map(|sensor| sensor.detection_range(target_row))
        .flatten()
        .collect::<HashSet<_>>();

    let beacons = sensors
        .iter()
        .filter(|sensor| sensor.beacon.1 == target_row)
        .map(|sensor| sensor.beacon.0)
        .collect::<HashSet<_>>();

    return positions.len() - beacons.len();
}

fn main() {
    let result = solve(include_str!("input.txt"), 2000000);
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_result() {
        let result = solve(include_str!("example.txt"), 10);
        assert_eq!(26, result);
    }
}

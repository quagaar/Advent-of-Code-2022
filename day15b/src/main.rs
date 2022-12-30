use std::ops::RangeInclusive;

struct Sensor {
    position: (i64, i64),
    beacon_distance: i64,
}

fn distance(start: &(i64, i64), end: &(i64, i64)) -> i64 {
    (start.0.abs_diff(end.0) + start.1.abs_diff(end.1)) as i64
}

impl Sensor {
    fn from(line: &str) -> Sensor {
        let (position, beacon) = line.split_once(": ").unwrap();
        let position: (i64, i64) = position
            .trim_start_matches("Sensor at ")
            .split_once(", ")
            .map(|(x, y)| {
                (
                    x.trim_start_matches("x=").parse().unwrap(),
                    y.trim_start_matches("y=").parse().unwrap(),
                )
            })
            .unwrap();
        let beacon: (i64, i64) = beacon
            .trim_start_matches("closest beacon is at ")
            .split_once(", ")
            .map(|(x, y)| {
                (
                    x.trim_start_matches("x=").parse().unwrap(),
                    y.trim_start_matches("y=").parse().unwrap(),
                )
            })
            .unwrap();
        let beacon_distance = distance(&position, &beacon);
        Sensor {
            position,
            beacon_distance,
        }
    }

    fn in_range(&self, target: &(i64, i64)) -> bool {
        distance(&self.position, target) <= self.beacon_distance
    }
}

fn solve(input: &str, target_range: RangeInclusive<i64>) -> i64 {
    let sensors: Vec<Sensor> = input.lines().map(Sensor::from).collect();

    let pos = sensors
        .iter()
        .flat_map(|sensor| {
            (0..=sensor.beacon_distance)
                .flat_map(|i| {
                    let offset = sensor.beacon_distance + 1 - i;
                    [
                        (sensor.position.0 + i, sensor.position.1 + offset),
                        (sensor.position.0 + offset, sensor.position.1 - i),
                        (sensor.position.0 - i, sensor.position.1 - offset),
                        (sensor.position.0 - offset, sensor.position.1 + i),
                    ]
                })
                .filter(|position| {
                    target_range.contains(&position.0)
                        && target_range.contains(&position.1)
                        && !sensors.iter().any(|sensor| sensor.in_range(position))
                })
        })
        .next()
        .unwrap();

    (pos.0 * 4000000) + pos.1
}

fn main() {
    let result = solve(include_str!("input.txt"), 0..=4000000);
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_result() {
        let result = solve(include_str!("example.txt"), 0..=20);
        assert_eq!(56000011, result);
    }

    #[test]
    fn puzzle_result() {
        let result = solve(include_str!("input.txt"), 0..=4000000);
        assert_eq!(10852583132904, result);
    }
}

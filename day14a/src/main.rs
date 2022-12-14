fn solve(input: &str) -> usize {
    let scans: Vec<Vec<(usize, usize)>> = input
        .lines()
        .map(|line| {
            line.split(" -> ")
                .map(|pair| {
                    pair.split_once(",")
                        .map(|(x, y)| (x.parse().unwrap(), y.parse().unwrap()))
                        .unwrap()
                })
                .collect()
        })
        .collect();

    let min_x = *scans.iter().flatten().map(|(x, _)| x).min().unwrap();
    let max_x = *scans.iter().flatten().map(|(x, _)| x).max().unwrap();
    let max_y = *scans.iter().flatten().map(|(_, y)| y).max().unwrap();

    let mut map = vec![vec![false; max_x - min_x + 1]; max_y + 1];

    for path in scans {
        let mut windows = path.windows(2);
        while let Some([start, end]) = windows.next() {
            if start.0 == end.0 {
                let from = start.1.min(end.1);
                let to = start.1.max(end.1);
                for y in from..=to {
                    map[y][start.0 - min_x] = true;
                }
            } else {
                let from = start.0.min(end.0);
                let to = start.0.max(end.0);
                for x in from..=to {
                    map[start.1][x - min_x] = true;
                }
            }
        }
    }

    let start_x = 500 - min_x;
    let mut result = 0;

    loop {
        let mut x = start_x;

        for y in 0..max_y {
            if map[y + 1][x] {
                if x == 0 {
                    return result;
                } else if map[y + 1][x - 1] {
                    if x + 1 > max_x - min_x {
                        return result;
                    } else if map[y + 1][x + 1] {
                        map[y][x] = true;
                        result += 1;
                        break;
                    } else {
                        x = x + 1;
                    }
                } else {
                    x = x - 1;
                }
            } else if y + 1 == max_y {
                return result;
            }
        }
    }
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
        assert_eq!(24, result);
    }
}

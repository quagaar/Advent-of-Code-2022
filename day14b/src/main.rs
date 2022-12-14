fn get_scans(input: &str) -> Vec<Vec<(usize, usize)>> {
    input
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
        .collect()
}

fn build_map(scans: &Vec<Vec<(usize, usize)>>) -> (Vec<Vec<bool>>, usize) {
    let min_x = *scans.iter().flatten().map(|(x, _)| x).min().unwrap();
    let max_x = *scans.iter().flatten().map(|(x, _)| x).max().unwrap();
    let max_y = *scans.iter().flatten().map(|(_, y)| y).max().unwrap() + 2;
    let min_x = min_x.min(500 - max_y);
    let max_x = max_x.max(500 + max_y);

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

    for x in 0..map[max_y].len() {
        map[max_y][x] = true;
    }

    return (map, min_x);
}

fn drop_sand((x, y): (usize, usize), map: &Vec<Vec<bool>>) -> Option<(usize, usize)> {
    if !map[y + 1][x] {
        return Some((x, y + 1));
    }
    if !map[y + 1][x - 1] {
        return Some((x - 1, y + 1));
    }
    if !map[y + 1][x + 1] {
        return Some((x + 1, y + 1));
    }
    return None;
}

fn solve(input: &str) -> usize {
    let scans = get_scans(input);
    let (mut map, x_offset) = build_map(&scans);

    let start_x = 500 - x_offset;
    let mut result = 0;

    loop {
        let mut pos = (start_x, 0);

        while let Some(next) = drop_sand(pos, &map) {
            pos = next;
        }

        map[pos.1][pos.0] = true;
        result += 1;

        if pos.1 == 0 {
            return result;
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
        assert_eq!(93, result);
    }
}

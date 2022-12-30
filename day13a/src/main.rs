use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq)]
enum PacketData {
    Number(i32),
    List(Vec<PacketData>),
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PacketData::Number(left), PacketData::Number(right)) => left.cmp(right),
            (PacketData::Number(_), PacketData::List(right)) => vec![self.clone()].cmp(right),
            (PacketData::List(left), PacketData::List(right)) => left.cmp(right),
            (PacketData::List(left), PacketData::Number(_)) => left.cmp(&vec![other.clone()]),
        }
    }
}

impl PartialOrd for PacketData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_data(src: &str) -> (PacketData, &str) {
    match src.chars().next() {
        Some('[') => parse_list(&src[1..]),
        Some(ch) if ('0'..='9').contains(&ch) => parse_number(src),
        _ => panic!("Unparsable string: {}", src),
    }
}

fn parse_list(src: &str) -> (PacketData, &str) {
    let mut list: Vec<PacketData> = vec![];
    let mut remain = src;
    while let Some(ch) = remain.chars().next() {
        if ch == ']' {
            return (PacketData::List(list), &remain[1..]);
        }
        let (value, rest) = parse_data(remain);
        list.push(value);
        remain = rest.trim_start_matches(',').trim_start();
    }
    (PacketData::List(list), &remain[0..0])
}

fn parse_number(src: &str) -> (PacketData, &str) {
    match src.char_indices().find(|(_, ch)| *ch < '0' || *ch > '9') {
        Some((i, _)) => (PacketData::Number(src[..i].parse().unwrap()), &src[i..]),
        None => (PacketData::Number(src.parse().unwrap()), &src[0..0]),
    }
}

fn solve(input: &str) -> usize {
    let packets: Vec<(PacketData, PacketData)> = input
        .split("\n\n")
        .map(|s| {
            let mut packets = s.lines().map(parse_data);
            (packets.next().unwrap().0, packets.next().unwrap().0)
        })
        .collect();

    let order: Vec<_> = packets
        .iter()
        .enumerate()
        .map(|(i, (left, right))| (i + 1, left.cmp(right)))
        .collect();

    return order
        .iter()
        .filter(|(_, ord)| *ord == Ordering::Less)
        .map(|(i, _)| i)
        .sum();
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
        assert_eq!(13, result);
    }

    #[test]
    fn puzzle_result() {
        let result = solve(include_str!("input.txt"));
        assert_eq!(5003, result);
    }
}

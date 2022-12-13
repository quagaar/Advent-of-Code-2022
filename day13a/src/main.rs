use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, PartialEq)]
enum PacketData {
    Number(i32),
    List(Vec<PacketData>),
}

impl Ord for PacketData {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (PacketData::Number(left), PacketData::Number(right)) => left.cmp(&right),
            (PacketData::Number(_), PacketData::List(right)) => vec![self.clone()].cmp(right),
            (PacketData::List(left), PacketData::List(right)) => left.cmp(&right),
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
    match src.chars().nth(0) {
        Some('[') => parse_list(&src[1..]),
        Some(ch) if ch >= '0' && ch <= '9' => parse_number(src),
        _ => panic!("Unparsable string: {}", src),
    }
}

fn parse_list(src: &str) -> (PacketData, &str) {
    let mut list: Vec<PacketData> = vec![];
    let mut remain = src;
    while let Some(ch) = remain.chars().nth(0) {
        if ch == ']' {
            return (PacketData::List(list), &remain[1..]);
        }
        let (value, rest) = parse_data(remain);
        list.push(value);
        remain = rest.trim_start_matches(",").trim_start();
    }
    return (PacketData::List(list), &remain[0..0]);
}

fn parse_number(src: &str) -> (PacketData, &str) {
    match src.char_indices().find(|(_, ch)| *ch < '0' || *ch > '9') {
        Some((i, _)) => (PacketData::Number(src[..i].parse().unwrap()), &src[i..]),
        None => (PacketData::Number(src.parse().unwrap()), &src[0..0]),
    }
}

fn main() {
    let packets: Vec<(PacketData, PacketData)> = include_str!("input.txt")
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

    let result: usize = order
        .iter()
        .filter(|(_, ord)| *ord == Ordering::Less)
        .map(|(i, _)| i)
        .sum();

    println!("{:?}", result);
}

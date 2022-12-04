use std::ops::RangeInclusive;

fn get_range(str: &str) -> RangeInclusive<i32> {
    let (start, end) = str.split_once("-").expect("missing range delimiter");
    let start: i32 = start.parse().expect("range start is not an integer");
    let end: i32 = end.parse().expect("range end is not an integer");
    start..=end
}

fn get_ranges(line: &str) -> (RangeInclusive<i32>, RangeInclusive<i32>) {
    let (a, b) = line.split_once(",").expect("no comma on line");
    (get_range(a), get_range(b))
}

fn any_overlap((a, b): &(RangeInclusive<i32>, RangeInclusive<i32>)) -> bool {
    a.end() >= b.start() && b.end() >= a.start()
}

fn main() {
    let result = include_str!("input.txt")
        .lines()
        .map(get_ranges)
        .filter(any_overlap)
        .count();

    println!("{}", result);
}

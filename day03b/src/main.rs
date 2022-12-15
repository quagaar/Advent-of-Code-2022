struct ElfGroupIterator<'a> {
    iter: std::str::Lines<'a>,
}

impl<'a> Iterator for ElfGroupIterator<'a> {
    type Item = [&'a str; 3];

    fn next(&mut self) -> Option<[&'a str; 3]> {
        let a = self.iter.next()?;
        let b = self.iter.next().expect("last group is incomplete");
        let c = self.iter.next().expect("last group is incomplete");
        Some([a, b, c])
    }
}

trait GroupElves<'a> {
    fn group_elves(self) -> ElfGroupIterator<'a>;
}

impl<'a> GroupElves<'a> for std::str::Lines<'a> {
    fn group_elves(self) -> ElfGroupIterator<'a> {
        ElfGroupIterator { iter: self }
    }
}

fn get_group_badge(elves: [&str; 3]) -> char {
    for a in elves[0].chars() {
        for b in elves[1].chars() {
            if a == b {
                for c in elves[2].chars() {
                    if a == c {
                        return c;
                    }
                }
            }
        }
    }
    panic!("Group badge not found: {:?}", elves)
}

fn get_priority(item_type: char) -> i32 {
    match item_type {
        'a'..='z' => 1 + (item_type as i32 - 'a' as i32),
        'A'..='Z' => 27 + (item_type as i32 - 'A' as i32),
        _ => panic!("Unexpected item type: {}", item_type),
    }
}

fn solve(input: &str) -> i32 {
    input
        .lines()
        .group_elves()
        .map(get_group_badge)
        .map(get_priority)
        .sum()
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
        assert_eq!(70, result);
    }
}

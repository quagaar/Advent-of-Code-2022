use std::str::Lines;

struct Monkey {
    items: Vec<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    test: Box<dyn Fn(i64) -> usize>,
}

fn parse_starting_items(line: &str) -> Vec<i64> {
    line.trim_start_matches("  Starting items: ")
        .split(", ")
        .map(|s| s.parse().unwrap())
        .collect()
}

fn parse_operation(line: &str) -> Box<dyn Fn(i64) -> i64> {
    line.trim_start_matches("  Operation: new = old ")
        .split_once(' ')
        .map(|(op, num)| -> Box<dyn Fn(i64) -> i64> {
            if num == "old" {
                match op {
                    "+" => Box::new(|x| x + x),
                    "*" => Box::new(|x| x * x),
                    _ => panic!("Unknown operation: {}", op),
                }
            } else {
                let num: i64 = num.parse().unwrap();
                match op {
                    "+" => Box::new(move |x| x + num),
                    "*" => Box::new(move |x| x * num),
                    _ => panic!("Unknown operation: {}", op),
                }
            }
        })
        .unwrap()
}

fn parse_test(mut lines: Lines, modulus: &mut i64) -> Box<dyn Fn(i64) -> usize> {
    lines
        .next()
        .unwrap()
        .trim_start_matches("  Test: divisible by ")
        .parse::<i64>()
        .map(|div| {
            *modulus *= div;
            let if_true: usize = lines
                .next()
                .unwrap()
                .trim_start_matches("    If true: throw to monkey ")
                .parse()
                .unwrap();
            let if_false: usize = lines
                .next()
                .unwrap()
                .trim_start_matches("    If false: throw to monkey ")
                .parse()
                .unwrap();
            Box::new(move |x| if x % div == 0 { if_true } else { if_false })
        })
        .unwrap()
}

fn parse_monkey(s: &str, modulus: &mut i64) -> Monkey {
    let mut lines = s.lines();
    assert!(lines.next().unwrap().starts_with("Monkey "));
    let items = parse_starting_items(lines.next().unwrap());
    let operation = parse_operation(lines.next().unwrap());
    let test = parse_test(lines, modulus);
    Monkey {
        items,
        operation,
        test,
    }
}

fn solve(input: &str) -> usize {
    let mut modulus = 1;

    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|s| parse_monkey(s, &mut modulus))
        .collect();

    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            inspections[i] += monkeys[i].items.len();
            let items: Vec<i64> = monkeys[i].items.drain(..).collect();
            for worry in items {
                let worry = (monkeys[i].operation)(worry) % modulus;
                let next = (monkeys[i].test)(worry);
                monkeys[next].items.push(worry);
            }
        }
    }

    if cfg!(debug_assertions) {
        println!("modulus => {}", modulus);
        println!();
        for (i, monkey) in monkeys.iter().enumerate() {
            println!("Monkey {}: {:?}", i, monkey.items);
        }
        println!();
        println!("inspections => {:?}", inspections);
        println!();
    }

    inspections.sort();

    inspections.iter().rev().take(2).product()
}

fn main() {
    let result = solve(include_str!("input.txt"));
    println!("monkey business => {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_result() {
        let result = solve(include_str!("example.txt"));
        assert_eq!(2713310158, result);
    }

    #[test]
    fn puzzle_result() {
        let result = solve(include_str!("input.txt"));
        assert_eq!(51382025916, result);
    }
}

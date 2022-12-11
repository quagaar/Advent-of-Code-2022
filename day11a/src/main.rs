struct Monkey {
    items: Vec<i32>,
    operation: Box<dyn Fn(i32) -> i32>,
    test: Box<dyn Fn(i32) -> usize>,
}

fn main() {
    let mut monkeys: Vec<Monkey> = include_str!("input.txt")
        .split("\n\n")
        .map(|s| {
            let mut lines = s.lines();
            assert!(lines.next().unwrap().starts_with("Monkey "));
            let items = lines
                .next()
                .unwrap()
                .trim_start_matches("  Starting items: ")
                .split(", ")
                .map(|s| s.parse().unwrap())
                .collect();
            let operation = lines
                .next()
                .unwrap()
                .trim_start_matches("  Operation: new = old ")
                .split_once(" ")
                .map(|(op, num)| -> Box<dyn Fn(i32) -> i32> {
                    if num == "old" {
                        match op {
                            "+" => Box::new(|x| x + x),
                            "*" => Box::new(|x| x * x),
                            _ => panic!("Unknown operation: {}", op),
                        }
                    } else {
                        let num: i32 = num.parse().unwrap();
                        match op {
                            "+" => Box::new(move |x| x + num),
                            "*" => Box::new(move |x| x * num),
                            _ => panic!("Unknown operation: {}", op),
                        }
                    }
                })
                .unwrap();
            let test = lines
                .next()
                .unwrap()
                .trim_start_matches("  Test: divisible by ")
                .parse::<i32>()
                .map(|div| -> Box<dyn Fn(i32) -> usize> {
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
                .unwrap();
            Monkey {
                items,
                operation,
                test,
            }
        })
        .collect();

    let mut inspections = vec![0; monkeys.len()];

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            inspections[i] += monkeys[i].items.len();
            let items: Vec<i32> = monkeys[i].items.drain(..).collect();
            for worry in items {
                let worry = (monkeys[i].operation)(worry) / 3;
                let next = (monkeys[i].test)(worry);
                monkeys[next].items.push(worry);
            }
        }
    }

    for (i, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {}: {:?}", i, monkey.items);
    }

    println!("\ninspections => {:?}\n", inspections);

    inspections.sort();
    let result: usize = inspections.iter().rev().take(2).product();

    println!("monkey business => {}", result);
}

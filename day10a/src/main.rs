fn do_cycle(cycles: &mut i32, x: i32, signal_strengths: &mut [i32; 6]) {
    *cycles += 1;
    match cycles {
        20 => signal_strengths[0] = *cycles * x,
        60 => signal_strengths[1] = *cycles * x,
        100 => signal_strengths[2] = *cycles * x,
        140 => signal_strengths[3] = *cycles * x,
        180 => signal_strengths[4] = *cycles * x,
        220 => signal_strengths[5] = *cycles * x,
        _ => (),
    }
}

fn main() {
    let mut cycles: i32 = 0;
    let mut x: i32 = 1;
    let mut signal_strengths = [0; 6];

    for line in include_str!("input.txt").lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "noop" => {
                do_cycle(&mut cycles, x, &mut signal_strengths);
            }
            "addx" => {
                do_cycle(&mut cycles, x, &mut signal_strengths);
                do_cycle(&mut cycles, x, &mut signal_strengths);
                x += parts[1].parse::<i32>().unwrap();
            }
            _ => panic!("unexpected op: {}", line),
        }
    }

    let result: i32 = signal_strengths.iter().sum();

    println!("{}", result);
}

fn do_cycle(cycles: &mut usize, x: i32, screen: &mut [[char; 40]; 6]) {
    let screen_cycle = *cycles % 240;
    let row = screen_cycle / 40;
    let column = screen_cycle % 40;

    match x - column as i32 {
        -1 | 0 | 1 => {
            screen[row][column] = '#';
        }
        _ => {
            screen[row][column] = ' ';
        }
    }

    if column == 39 {
        println!("{}", String::from_iter(screen[row].iter()));
    }
    if screen_cycle == 239 {
        println!("");
    }

    *cycles += 1;
}

fn main() {
    let mut cycles: usize = 0;
    let mut x: i32 = 1;
    let mut screen = [['.'; 40]; 6];

    for line in include_str!("input.txt").lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        match parts[0] {
            "noop" => {
                do_cycle(&mut cycles, x, &mut screen);
            }
            "addx" => {
                do_cycle(&mut cycles, x, &mut screen);
                do_cycle(&mut cycles, x, &mut screen);
                x += parts[1].parse::<i32>().unwrap();
            }
            _ => panic!("unexpected op: {}", line),
        }
    }
}

const CHAMBER_WIDTH: usize = 7;
const NUMBER_OF_ROCKS: usize = 2022;
const EMPTY_ROW: [char; CHAMBER_WIDTH] = [' '; CHAMBER_WIDTH];

fn get_rocks() -> [Vec<&'static str>; 5] {
    [
        vec!["####"],
        vec![".#.", "###", ".#."],
        vec!["..#", "..#", "###"],
        vec!["#", "#", "#", "#"],
        vec!["##", "##"],
    ]
}

fn empty_space(chamber: &Vec<[char; CHAMBER_WIDTH]>) -> usize {
    chamber
        .iter()
        .rev()
        .take_while(|&row| row.eq(&EMPTY_ROW))
        .count()
}

fn update_chamber_height(chamber: &mut Vec<[char; CHAMBER_WIDTH]>, rock_height: usize) {
    let space_available = empty_space(&chamber);
    let space_required = rock_height + 3;
    if space_required > space_available {
        (space_available..space_required).for_each(|_| chamber.push(EMPTY_ROW));
    } else if space_required < space_available {
        (space_required..space_available).for_each(|_| {
            chamber.pop();
        });
    }
}

fn check_overlap(
    pos: &(usize, usize),
    rock: &Vec<&'static str>,
    chamber: &Vec<[char; CHAMBER_WIDTH]>,
) -> bool {
    rock.iter().enumerate().any(|(i, &row)| {
        let y = pos.1 - i;
        row.chars()
            .enumerate()
            .any(|(j, ch)| ch == '#' && chamber[y][pos.0 + j] == '#')
    })
}

fn add_rock(
    pos: &(usize, usize),
    rock: &Vec<&'static str>,
    chamber: &mut Vec<[char; CHAMBER_WIDTH]>,
) {
    rock.iter().enumerate().for_each(|(i, &row)| {
        let y = pos.1 - i;
        row.chars().enumerate().for_each(|(j, ch)| {
            if ch == '#' {
                chamber[y][pos.0 + j] = '#'
            }
        })
    })
}

fn solve(input: &str) -> usize {
    let mut jets_iter = input.trim_end().chars().cycle();
    let mut chamber: Vec<[char; CHAMBER_WIDTH]> = vec![];

    for rock in get_rocks().into_iter().cycle().take(NUMBER_OF_ROCKS) {
        update_chamber_height(&mut chamber, rock.len());

        // drop rock
        let mut pos = (2_usize, chamber.len() - 1);
        loop {
            let mut next_pos = pos;

            // Update next pos based on jet, limited by walls
            let jet = jets_iter.next().unwrap();
            match jet {
                '>' => next_pos.0 = (CHAMBER_WIDTH - rock[0].len()).min(next_pos.0 + 1),
                '<' => next_pos.0 = if pos.0 > 0 { pos.0 - 1 } else { pos.0 },
                _ => panic!("Unknown jet type: {}", jet),
            }

            // If jet movement causes overlap then reset next pos, otherwise set pos to next pos
            if check_overlap(&next_pos, &rock, &chamber) {
                next_pos = pos;
            } else {
                pos = next_pos;
            }

            // If at floor then stop here
            if next_pos.1 < rock.len() {
                break;
            }

            // Drop down one place
            next_pos.1 -= 1;
            if check_overlap(&next_pos, &rock, &chamber) {
                break;
            } else {
                pos = next_pos;
            }
        }

        // add rock to chamber
        add_rock(&pos, &rock, &mut chamber);
    }

    if cfg!(debug_assertions) {
        for line in chamber.iter().rev() {
            println!("|{}|", String::from_iter(line.iter()));
        }
        println!("+{}+\n", String::from_iter(['-'; CHAMBER_WIDTH].iter()));
    }

    return chamber.len() - empty_space(&chamber);
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
        assert_eq!(3068, result);
    }
}

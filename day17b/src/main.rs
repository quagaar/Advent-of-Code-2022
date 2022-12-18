use std::cell::Cell;

const CHAMBER_WIDTH: usize = 7;
const NUMBER_OF_ROCKS: usize = 1_000_000_000_000;
const SAMPLE_SIZE: usize = 1_000_000;
const REPEAT_TEST_WINDOW: usize = 100;
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

fn drop_rock(
    rock: &Vec<&'static str>,
    get_jet: &mut impl FnMut() -> char,
    chamber: &Vec<[char; CHAMBER_WIDTH]>,
) -> (usize, usize) {
    let mut pos = (2_usize, chamber.len() - 1);
    loop {
        let mut next_pos = pos;

        // Update next pos based on jet, limited by walls
        let jet = get_jet();
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
            return pos;
        }

        // Drop down one place
        next_pos.1 -= 1;
        if check_overlap(&next_pos, &rock, &chamber) {
            return pos;
        } else {
            pos = next_pos;
        }
    }
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

fn calculate_final_height(end_states: Vec<(usize, usize, usize)>) -> usize {
    let target = end_states
        .iter()
        .rev()
        .take(REPEAT_TEST_WINDOW)
        .rev()
        .collect::<Vec<_>>();
    let samples = end_states
        .windows(REPEAT_TEST_WINDOW)
        .rev()
        .enumerate()
        .skip(1)
        .filter(|(_, window)| {
            window
                .iter()
                .zip(target.iter())
                .all(|(a, b)| a.0 == b.0 && a.1 == b.1)
        })
        .take(2)
        .collect::<Vec<_>>();

    let repeat_rock_count = samples[1].0 - samples[0].0;
    let remaining_repeats = ((NUMBER_OF_ROCKS - SAMPLE_SIZE) / repeat_rock_count) + 1;
    let index = NUMBER_OF_ROCKS - (remaining_repeats * repeat_rock_count) - 1;
    let height_before_repeats = end_states[index].2;
    let repeat_height = height_before_repeats - end_states[index - repeat_rock_count].2;

    if cfg!(debug_assertions) {
        println!("repeat rock count => {}", repeat_rock_count);
        println!("remaining repeats => {}", remaining_repeats);
        println!("height before repeats => {}", height_before_repeats);
        println!("repeat height => {}\n", repeat_height);
    }

    return height_before_repeats + (remaining_repeats * repeat_height);
}

fn solve(input: &str) -> usize {
    let jets = input.trim_end();
    let mut jets_iter = jets.chars().cycle();
    let mut chamber: Vec<[char; CHAMBER_WIDTH]> = vec![];

    let mut rock_index: usize = 0;
    let jet_index: Cell<usize> = Cell::new(0);
    let mut get_jet = || {
        jet_index.set((jet_index.get() + 1) % jets.len());
        jets_iter.next().unwrap()
    };

    let mut end_states: Vec<(usize, usize, usize)> = vec![];

    for rock in get_rocks().into_iter().cycle().take(SAMPLE_SIZE) {
        rock_index = (rock_index + 1) % 5;

        update_chamber_height(&mut chamber, rock.len());
        let pos = drop_rock(&rock, &mut get_jet, &chamber);
        add_rock(&pos, &rock, &mut chamber);

        end_states.push((
            rock_index,
            jet_index.get(),
            chamber.len() - empty_space(&chamber),
        ));
    }

    return calculate_final_height(end_states);
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
        assert_eq!(1514285714288, result);
    }
}

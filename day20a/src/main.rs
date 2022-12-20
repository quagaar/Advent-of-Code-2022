fn solve(input: &str) -> i32 {
    let mut numbers: Vec<(usize, i32)> = input
        .lines()
        .map(|l| l.parse().unwrap())
        .enumerate()
        .collect();

    for n in 0..numbers.len() {
        let (old_pos, num) = numbers
            .iter()
            .enumerate()
            .find(|(_, (i, _))| *i == n)
            .map(|(pos, (_, num))| (pos, *num))
            .unwrap();

        let new_pos = (old_pos as i32 + num).rem_euclid(numbers.len() as i32 - 1) as usize;

        let element = numbers.remove(old_pos);
        numbers.insert(new_pos, element);
    }

    let zero_pos = numbers
        .iter()
        .enumerate()
        .find(|(_, (_, n))| *n == 0)
        .map(|(i, (_, _))| i)
        .unwrap();

    let result = [1000, 2000, 3000]
        .into_iter()
        .map(|offset| numbers.get((zero_pos + offset) % numbers.len()).unwrap().1)
        .sum();

    return result;
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
        assert_eq!(3, result);
    }
}

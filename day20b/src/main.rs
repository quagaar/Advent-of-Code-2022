const DECRYPTION_KEY: i64 = 811589153;

fn solve(input: &str) -> i64 {
    let mut numbers: Vec<(usize, i64)> = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap() * DECRYPTION_KEY)
        .enumerate()
        .collect();

    for _ in 0..10 {
        for n in 0..numbers.len() {
            let (old_pos, num) = numbers
                .iter()
                .enumerate()
                .find(|(_, (i, _))| *i == n)
                .map(|(pos, (_, num))| (pos, *num))
                .unwrap();

            let new_pos = (old_pos as i64 + num).rem_euclid(numbers.len() as i64 - 1) as usize;

            let element = numbers.remove(old_pos);
            numbers.insert(new_pos, element);
        }
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
        assert_eq!(1623178306, result);
    }
}

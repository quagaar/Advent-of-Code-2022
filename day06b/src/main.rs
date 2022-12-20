fn has_duplicates(chars: &&[char]) -> bool {
    chars
        .iter()
        .enumerate()
        .any(|(i, ch)| chars.iter().skip(i + 1).any(|x| x == ch))
}

fn solve(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<char>>();
    return chars.windows(14).take_while(has_duplicates).count() + 14;
}

fn main() {
    let result = solve(include_str!("input.txt"));
    println!("{:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1_result() {
        let result = solve(include_str!("example1.txt"));
        assert_eq!(19, result);
    }

    #[test]
    fn example2_result() {
        let result = solve(include_str!("example2.txt"));
        assert_eq!(23, result);
    }

    #[test]
    fn example3_result() {
        let result = solve(include_str!("example3.txt"));
        assert_eq!(23, result);
    }

    #[test]
    fn example4_result() {
        let result = solve(include_str!("example4.txt"));
        assert_eq!(29, result);
    }

    #[test]
    fn example5_result() {
        let result = solve(include_str!("example5.txt"));
        assert_eq!(26, result);
    }
}

fn has_duplicates(chars: &&[char]) -> bool {
    chars
        .iter()
        .enumerate()
        .any(|(i, ch)| chars.iter().skip(i + 1).any(|x| x == ch))
}

fn solve(input: &str) -> usize {
    let chars = input.chars().collect::<Vec<char>>();
    return chars.windows(4).take_while(has_duplicates).count() + 4;
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
        assert_eq!(7, result);
    }

    #[test]
    fn example2_result() {
        let result = solve(include_str!("example2.txt"));
        assert_eq!(5, result);
    }

    #[test]
    fn example3_result() {
        let result = solve(include_str!("example3.txt"));
        assert_eq!(6, result);
    }

    #[test]
    fn example4_result() {
        let result = solve(include_str!("example4.txt"));
        assert_eq!(10, result);
    }

    #[test]
    fn example5_result() {
        let result = solve(include_str!("example5.txt"));
        assert_eq!(11, result);
    }
}

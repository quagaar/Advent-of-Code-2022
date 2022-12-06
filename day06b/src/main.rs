fn has_duplicates(chars: &&[char]) -> bool {
    chars
        .iter()
        .enumerate()
        .any(|(i, ch)| chars.iter().skip(i + 1).any(|x| x == ch))
}

fn main() {
    let input = include_str!("input.txt");
    let chars = input.chars().collect::<Vec<char>>();
    let result = chars.windows(14).take_while(has_duplicates).count() + 14;
    println!("{}", result);
}

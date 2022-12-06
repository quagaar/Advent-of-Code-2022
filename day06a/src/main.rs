fn has_duplicates(chars: &&[char]) -> bool {
    for (i, ch) in chars.iter().enumerate() {
        if chars.iter().skip(i + 1).any(|x| x == ch) {
            return true;
        }
    }
    return false;
}

fn main() {
    let input = include_str!("input.txt");
    let chars = input.chars().collect::<Vec<char>>();
    let result = chars.windows(4).take_while(has_duplicates).count() + 4;
    println!("{}", result);
}

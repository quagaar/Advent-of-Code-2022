fn main() {
    let result = include_str!("input.txt")
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<i32>().unwrap()).sum::<i32>())
        .max()
        .unwrap();

    println!("{}", result);
}

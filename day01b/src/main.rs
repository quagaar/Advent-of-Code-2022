fn main() {
    let mut elf_calories = include_str!("input.txt")
        .split("\n\n")
        .map(|x| x.lines().map(|x| x.parse::<i32>().unwrap()).sum::<i32>())
        .collect::<Vec<i32>>();

    elf_calories.sort();

    let result: i32 = elf_calories.iter().rev().take(3).sum();

    println!("{}", result);
}

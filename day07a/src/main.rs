use std::str::Lines;

enum DirectoryEntry {
    Directory {
        name: String,
        contents: Vec<DirectoryEntry>,
    },
    File {
        name: String,
        size: usize,
    },
}

impl DirectoryEntry {
    fn name(&self) -> &str {
        match self {
            DirectoryEntry::Directory { name, contents: _ } => name,
            DirectoryEntry::File { name, size: _ } => name,
        }
    }

    fn size(&self) -> usize {
        match self {
            DirectoryEntry::Directory { name: _, contents } => {
                contents.iter().map(|x| x.size()).sum()
            }
            DirectoryEntry::File { name: _, size } => *size,
        }
    }
}

fn read_input(cwd: &mut DirectoryEntry, lines: &mut Lines) {
    if let DirectoryEntry::Directory { name: _, contents } = cwd {
        while let Some(line) = lines.next() {
            match line {
                "$ ls" => (),
                "$ cd .." => return,
                str if str.starts_with("$ cd ") => {
                    let dir = &str[5..];
                    let dir = contents.iter_mut().find(|x| match x {
                        DirectoryEntry::Directory { name, contents: _ } => name == dir,
                        _ => false,
                    });
                    read_input(dir.unwrap(), lines);
                }
                str if str.starts_with("dir ") => {
                    let name = &str[4..];
                    debug_assert!(!contents.iter().any(|x| x.name() == name));
                    contents.push(DirectoryEntry::Directory {
                        name: name.to_string(),
                        contents: vec![],
                    });
                }
                str => {
                    let (size, name) = str.split_once(' ').unwrap();
                    debug_assert!(!contents.iter().any(|x| x.name() == name));
                    contents.push(DirectoryEntry::File {
                        name: name.to_string(),
                        size: size.parse().unwrap(),
                    });
                }
            }
        }
    }
}

fn count_small_directories(cwd: &DirectoryEntry) -> usize {
    let mut result = 0;

    if let DirectoryEntry::Directory { name: _, contents } = cwd {
        let size = cwd.size();
        if size <= 100000 {
            result += size;
        }

        result += contents.iter().map(count_small_directories).sum::<usize>();
    }

    result
}

fn solve(input: &str) -> usize {
    let mut root = DirectoryEntry::Directory {
        name: String::from("/"),
        contents: vec![],
    };
    let mut lines = input.lines();
    assert_eq!("$ cd /", lines.next().unwrap());

    read_input(&mut root, &mut lines);

    count_small_directories(&root)
}

fn main() {
    let result = solve(include_str!("input.txt"));
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_result() {
        let result = solve(include_str!("example.txt"));
        assert_eq!(95437, result);
    }

    #[test]
    fn puzzle_result() {
        let result = solve(include_str!("input.txt"));
        assert_eq!(1886043, result);
    }
}

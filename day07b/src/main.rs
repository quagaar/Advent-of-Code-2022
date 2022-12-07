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
                    assert!(!contents.iter().any(|x| x.name() == name));
                    contents.push(DirectoryEntry::Directory {
                        name: name.to_string(),
                        contents: vec![],
                    });
                }
                str => {
                    let (size, name) = str.split_once(" ").unwrap();
                    assert!(!contents.iter().any(|x| x.name() == name));
                    contents.push(DirectoryEntry::File {
                        name: name.to_string(),
                        size: size.parse().unwrap(),
                    });
                }
            }
        }
    }
}

fn find_directory_to_delete(cwd: &DirectoryEntry, target: usize) -> Option<usize> {
    if let DirectoryEntry::Directory { name: _, contents } = cwd {
        let size = cwd.size();
        if size < target {
            return None;
        }

        let result = contents
            .iter()
            .map(|x| find_directory_to_delete(x, target))
            .filter(|x| x.is_some())
            .map(|x| x.unwrap())
            .min()
            .unwrap_or(size);

        return Some(result);
    }

    return None;
}

const TOTAL_SPACE_AVAILABLE: usize = 70000000;
const REQUIRED_SPACE: usize = 30000000;
const TARGET_SIZE: usize = TOTAL_SPACE_AVAILABLE - REQUIRED_SPACE;

fn main() {
    let mut root = DirectoryEntry::Directory {
        name: String::from("/"),
        contents: vec![],
    };
    let mut lines = include_str!("input.txt").lines();
    assert_eq!("$ cd /", lines.next().unwrap());

    read_input(&mut root, &mut lines);

    let current_usage = root.size();
    let need_to_delete = current_usage - TARGET_SIZE;

    let result = find_directory_to_delete(&root, need_to_delete).unwrap();

    println!("{}", result);
}

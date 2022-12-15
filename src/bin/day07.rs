use std::collections::HashMap;
use std::str::FromStr;

fn main() {
    let data = include_str!("../../inputs/day07.txt");
    let lines: Vec<&str> = data.lines().collect();

    let part1 = part1(&lines);
    println!("result(part1) = {part1}");

    let part2 = part2(&lines);
    println!("result(part2) = {part2}");
}

enum LineType {
    CdCommand { target_directory: String },
    LsCommand,
    DirLine { directory_name: String },
    FileSizeLine { size: usize },
}

impl FromStr for LineType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            _ if s.starts_with("$ cd") => {
                let dir = s.rsplitn(2, ' ').next().unwrap();
                Ok(LineType::CdCommand {
                    target_directory: String::from(dir),
                })
            }
            "$ ls" => Ok(LineType::LsCommand),
            _ if s.starts_with("dir") => {
                let dir = s.rsplitn(2, ' ').next().unwrap();
                Ok(LineType::DirLine {
                    directory_name: String::from(dir),
                })
            }
            _ => {
                let size = s.split(' ').next().unwrap();
                Ok(LineType::FileSizeLine {
                    size: size.parse::<usize>().unwrap(),
                })
            }
        }
    }
}

fn parse_input(lines: &Vec<&str>) -> HashMap<Vec<String>, usize> {
    let slash = String::from("/");
    let back = String::from("..");
    let mut sizes: HashMap<Vec<String>, usize> = HashMap::new();
    let slash_key = vec![slash.clone()];
    sizes.insert(slash_key.clone(), 0);
    let mut current_path: Vec<String> = vec![];

    for &line in lines {
        match line.parse::<LineType>().unwrap() {
            LineType::CdCommand { target_directory } => {
                if target_directory.eq(&back) {
                    current_path.pop();
                } else if target_directory.eq(&slash) {
                    current_path = slash_key.clone();
                } else {
                    current_path.push(target_directory);
                }
            }
            LineType::LsCommand => {}
            LineType::DirLine { directory_name } => {
                let mut new_dir_path = current_path.clone();
                new_dir_path.push(directory_name);
                sizes.insert(new_dir_path, 0);
            }
            LineType::FileSizeLine { size } => {
                for i in 0..(current_path.len()) {
                    // update each path
                    *sizes.get_mut(&current_path[0..i + 1]).unwrap() += size;
                }
            }
        }
    }
    sizes
}

fn part1(lines: &Vec<&str>) -> usize {
    let size_map = parse_input(lines);

    size_map.values().filter(|v| **v < 100_000).sum()
}

fn part2(lines: &Vec<&str>) -> usize {
    let size_map = parse_input(lines);

    let fs_total_size: usize = 70_000_000;
    let update_space: usize = 30_000_000;

    let used_space = size_map[&vec![String::from("/")]];
    let unused_space = fs_total_size - used_space;
    let needed_space = update_space - unused_space;

    *size_map
        .values()
        .filter(|v| **v > needed_space)
        .min()
        .unwrap()
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    fn part_1_sample() {
        let lines = include_str!("../../inputs/day07.sample.txt")
            .lines()
            .collect();
        assert_eq!(part1(&lines), 95437);
    }

    #[test]
    fn part_2_sample() {
        let lines = include_str!("../../inputs/day07.sample.txt")
            .lines()
            .collect();
        assert_eq!(part2(&lines), 24933642);
    }
}

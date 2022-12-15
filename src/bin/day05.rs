use std::collections::VecDeque;
use std::fmt::{Display, Formatter};

fn main() {
    let data = include_str!("../../inputs/day05.txt");
    let lines = data.lines().collect();

    let part1 = part1(&mut Puzzle::create(&lines));
    println!("result(part1) = {part1}");

    let part2 = part2(&mut Puzzle::create(&lines));
    println!("result(part2) = {part2}");
}

#[derive(Debug, Copy, Clone)]
struct Command {
    nb: usize,
    from: usize,
    to: usize,
}

struct Puzzle {
    ship: Vec<VecDeque<char>>,
    commands: Vec<Command>,
}

impl Puzzle {
    pub fn create(input: &Vec<&str>) -> Self {
        let mut ship: Vec<VecDeque<char>> = vec![];
        let mut commands: Vec<Command> = vec![];

        for line in input {
            if line.is_empty() {
                continue;
            }
            let words = line
                .split_ascii_whitespace()
                .filter(|l| !l.is_empty())
                .collect::<Vec<&str>>();
            if words[0] == "move" {
                // it's an instruction
                let nb = words[1].parse::<usize>().unwrap();
                let from = words[3].parse::<usize>().unwrap() - 1;
                let to = words[5].parse::<usize>().unwrap() - 1;
                commands.push(Command { nb, from, to });
            } else if line.contains("[") {
                // it's a line containing a package
                for (char_idx, ch) in line
                    .chars()
                    .enumerate()
                    .skip(1)
                    .step_by(4)
                    .filter(|(_, ch)| *ch != ' ')
                {
                    let stack_idx = (char_idx - 1) / 4;
                    while ship.len() < stack_idx + 1 {
                        ship.push(VecDeque::new());
                    }
                    ship[stack_idx].push_front(ch);
                }
            }
        }
        Self { ship, commands }
    }

    pub fn top_letters(&self) -> String {
        self.ship.iter().filter_map(|stack| stack.back()).collect()
    }
}

impl Display for Puzzle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let max_len = self.ship.iter().map(VecDeque::len).max().unwrap();
        for y in (0..max_len).rev() {
            for stack in self.ship.iter() {
                if let Some(c) = stack.get(y) {
                    write!(f, "[{c}] ")?;
                } else {
                    write!(f, "    ")?;
                }
            }
            writeln!(f)?;
        }
        for x in 1..=self.ship.len() {
            write!(f, " {x}  ")?;
        }
        Ok(())
    }
}

fn part1(p: &mut Puzzle) -> String {
    println!("\nPuzzle (part 1): \n{p}");
    for command in &p.commands {
        for _ in 0..command.nb {
            let package = p.ship[command.from].pop_back().unwrap();
            p.ship[command.to].push_back(package);
        }
    }

    p.top_letters()
}

fn part2(p: &mut Puzzle) -> String {
    println!("\nPuzzle (part 2): \n{p}");

    for command in &p.commands {
        let start = p.ship[command.from].len() - command.nb;
        let mut moved_pile = p.ship[command.from].split_off(start);
        p.ship[command.to].append(&mut moved_pile);
    }

    p.top_letters()
}

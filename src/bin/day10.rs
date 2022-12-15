use std::str::FromStr;

fn main() {
    let data = include_str!("../../inputs/day10.txt");

    let mut x = 1;
    let cycles = data
        .lines()
        .filter_map(|l| l.parse::<Operation>().ok())
        .flat_map(|op: Operation| match op {
            Operation::AddX(num) => {
                let old_x = x;
                x += num;
                vec![old_x, old_x]
            }
            Operation::Noop => {
                vec![x]
            }
        })
        .collect::<Vec<i32>>();

    let part1: i32 = cycles
        .iter()
        .enumerate()
        .skip(19)
        .step_by(40)
        .map(|en| (en.0 as i32 + 1) * en.1)
        .sum();

    println!("Result part 1: {}", part1);

    println!("Result part 2 (below)");
    cycles.chunks(40).for_each(|chunk| {
        let line =
            chunk
                .iter()
                .enumerate()
                .fold(String::new(), |mut acc, (crt_index, sprite_pos)| {
                    let c = if crt_index as i32 == *sprite_pos - 1
                        || crt_index as i32 == *sprite_pos
                        || crt_index as i32 == *sprite_pos + 1
                    {
                        '#'
                    } else {
                        ' '
                    };
                    acc.push(c);
                    acc
                });

        println!("{}", line);
    });
}

#[derive(Debug, Copy, Clone)]
enum Operation {
    AddX(i32),
    Noop,
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "noop" => Ok(Operation::Noop),
            _ if s.starts_with("addx ") => {
                let number = s.split_ascii_whitespace().nth(1);
                Ok(Operation::AddX(
                    number.map(|n| n.parse::<i32>().unwrap()).unwrap(),
                ))
            }
            _ => Err(format!("[{}] is not valid operation", s)),
        }
    }
}

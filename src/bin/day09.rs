use num::signum;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

fn main() {
    let data: Vec<&str> = include_str!("../../inputs/day09.txt").lines().collect();
    println!("part 1 result: {}", part1(&data));
    println!("part 2 result: {}", part2(&data));
}

fn part1(lines: &Vec<&str>) -> usize {
    let mut grid = Grid::new(2);
    let valid_commands_iter = lines.iter().filter_map(|&l| l.parse::<Command>().ok());
    for command in valid_commands_iter {
        grid.move_head(command);
    }

    grid.tail_tracks.len()
}

fn part2(lines: &Vec<&str>) -> usize {
    let mut grid = Grid::new(10);
    let valid_commands_iter = lines.iter().filter_map(|&l| l.parse::<Command>().ok());
    for command in valid_commands_iter {
        grid.move_head(command);
    }

    grid.tail_tracks.len()
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl FromStr for Direction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "R" => Ok(Direction::Right),
            "L" => Ok(Direction::Left),
            "U" => Ok(Direction::Up),
            "D" => Ok(Direction::Down),
            _ => Err(format!("not a valid letter for a direction: [{}]", s)),
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct Command {
    times: usize,
    direction: Direction,
}

impl Display for Command {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.direction {
            Direction::Up => write!(f, "U {}", self.times)?,
            Direction::Down => write!(f, "D {}", self.times)?,
            Direction::Right => write!(f, "R {}", self.times)?,
            Direction::Left => write!(f, "L {}", self.times)?,
        }
        Ok(())
    }
}

impl FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (letter, quantity) = s
            .split_once(" ")
            .expect("command doesn't contain any space");

        // TODO:  I don't know yet how to propagate the eventual parsing error here as an Err(...)
        let times = quantity.parse::<usize>().unwrap();
        let direction = letter.parse::<Direction>().unwrap();
        Ok(Command { times, direction })
    }
}

#[derive(Debug)]
struct Grid {
    head_tracks: HashSet<(i32, i32)>,
    tail_tracks: HashSet<(i32, i32)>,
    rope: Vec<(i32, i32)>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (min_x, max_x, min_y, max_y) = self.head_tracks.iter().fold(
            (i32::MAX, i32::MIN, i32::MAX, i32::MIN),
            |acc, &point| {
                (
                    acc.0.min(point.0),
                    acc.1.max(point.0),
                    acc.2.min(point.1),
                    acc.3.max(point.1),
                )
            },
        );
        for y in (min_y..=max_y).rev() {
            for x in min_x..=max_x {
                if let Some(&head) = self.rope.first() {
                    if head == (x, y) {
                        write!(f, "H")?;
                    } else if let Some((idx, _)) = self
                        .rope
                        .iter()
                        .enumerate()
                        .find(|(_, &point)| point == (x, y))
                    {
                        write!(f, "{}", idx)?;
                    } else if x == 0 && y == 0 {
                        write!(f, "s")?;
                    } else {
                        write!(f, ".")?;
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(rope_size: usize) -> Self {
        let rope = vec![(0, 0); rope_size];
        let mut tail_tracks = HashSet::new();
        tail_tracks.insert(rope[rope.len() - 1]);
        let head_tracks = tail_tracks.clone();
        Self {
            tail_tracks,
            rope,
            head_tracks,
        }
    }

    fn move_head(&mut self, c: Command) {
        // println!("\n==== {} ====", c);
        for _ in 0..c.times {
            self.move_head_once(c.direction);
            // println!("{self}");
        }
    }

    fn move_head_once(&mut self, d: Direction) {
        let mut iter = self.rope.iter_mut();
        if let Some(head) = iter.next() {
            *head = new_point(*head, d);
            self.head_tracks.insert(*head);
            // adapting the rest of the rope
            let mut point_to_follow = *head;
            for rope_body_part in iter {
                *rope_body_part = follow(*rope_body_part, point_to_follow);
                point_to_follow = *rope_body_part;
            }
            self.tail_tracks.insert(point_to_follow);
        }
    }
}

fn new_point(point: (i32, i32), direction: Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (point.0, point.1 + 1),
        Direction::Down => (point.0, point.1 - 1),
        Direction::Left => (point.0 - 1, point.1),
        Direction::Right => (point.0 + 1, point.1),
    }
}

fn follow(point: (i32, i32), target: (i32, i32)) -> (i32, i32) {
    let (tx, ty) = point;
    let diff = (target.0 - tx, target.1 - ty);

    if diff.0 != 2 && diff.0 != -2 && diff.1 != 2 && diff.1 != -2 {
        // don't move
        return point;
    }
    (tx + signum(diff.0), ty + signum(diff.1))
}

#[cfg(test)]
mod test {
    use crate::{follow, part1, part2, Command, Direction, Grid};

    #[test]
    fn part_1_sample() {
        let lines = include_str!("../../inputs/day09.sample1.txt")
            .lines()
            .collect();
        assert_eq!(part1(&lines), 13);
    }

    #[test]
    fn tail_should_move_north_west() {
        let mut grid = Grid::new(2);
        grid.move_head(Command {
            times: 1,
            direction: Direction::Right,
        });
        grid.move_head(Command {
            times: 2,
            direction: Direction::Up,
        });

        assert_eq!(grid.rope, vec![(1, 2), (1, 1)]);
        assert_eq!(grid.tail_tracks.len(), 2);
        assert_eq!(grid.tail_tracks.contains(&(0, 0)), true);
        assert_eq!(grid.tail_tracks.contains(&(1, 1)), true);
    }

    #[test]
    fn part_2_sample1() {
        let lines = include_str!("../../inputs/day09.sample1.txt")
            .lines()
            .collect();
        assert_eq!(part2(&lines), 1);
    }

    #[test]
    fn part_2_sample2() {
        let lines = include_str!("../../inputs/day09.sample2.txt")
            .lines()
            .collect();
        assert_eq!(part2(&lines), 36);
    }

    #[test]
    fn follow_north() {
        let point = (0, 0);
        let target = (0, 2);
        assert_eq!(follow(point, target), (0, 1));
    }
    #[test]
    fn follow_south() {
        let point = (0, 0);
        let target = (0, -2);
        assert_eq!(follow(point, target), (0, -1));
    }

    #[test]
    fn follow_east() {
        let point = (0, 0);
        let target = (2, 0);
        assert_eq!(follow(point, target), (1, 0));
    }

    #[test]
    fn follow_west() {
        let point = (0, 0);
        let target = (-2, 0);
        assert_eq!(follow(point, target), (-1, 0));
    }

    #[test]
    fn follow_north_west_1() {
        let point = (0, 0);
        let target = (-2, 1);
        assert_eq!(follow(point, target), (-1, 1));
    }

    #[test]
    fn follow_north_west_2() {
        let point = (0, 0);
        let target = (-1, 2);
        assert_eq!(follow(point, target), (-1, 1));
    }

    #[test]
    fn follow_north_west_3() {
        let point = (0, 0);
        let target = (-2, 2);
        assert_eq!(follow(point, target), (-1, 1));
    }

    #[test]
    fn follow_north_east_1() {
        let point = (0, 0);
        let target = (2, 1);
        assert_eq!(follow(point, target), (1, 1));
    }

    #[test]
    fn follow_north_east_2() {
        let point = (0, 0);
        let target = (1, 2);
        assert_eq!(follow(point, target), (1, 1));
    }

    #[test]
    fn follow_north_east_3() {
        let point = (0, 0);
        let target = (2, 2);
        assert_eq!(follow(point, target), (1, 1));
    }

    #[test]
    fn follow_south_west_1() {
        let point = (0, 0);
        let target = (-2, -1);
        assert_eq!(follow(point, target), (-1, -1));
    }

    #[test]
    fn follow_south_west_2() {
        let point = (0, 0);
        let target = (-1, -2);
        assert_eq!(follow(point, target), (-1, -1));
    }

    #[test]
    fn follow_south_west_3() {
        let point = (0, 0);
        let target = (-2, -2);
        assert_eq!(follow(point, target), (-1, -1));
    }

    #[test]
    fn follow_south_east_1() {
        let point = (0, 0);
        let target = (2, -1);
        assert_eq!(follow(point, target), (1, -1));
    }

    #[test]
    fn follow_south_east_2() {
        let point = (0, 0);
        let target = (1, -2);
        assert_eq!(follow(point, target), (1, -1));
    }

    #[test]
    fn follow_south_east_3() {
        let point = (0, 0);
        let target = (2, -2);
        assert_eq!(follow(point, target), (1, -1));
    }
}

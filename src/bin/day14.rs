use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::str::FromStr;

fn main() {
    let data = include_str!("../../inputs/day14.txt");
    let game = parse_input(data);
    game.print();

    println!("Launching part 1:");
    let part1 = part1(game.clone());
    println!("result part 1: {part1}");

    println!("Launching part 2:");
    let part2 = part2(game.clone());
    println!("result part 2: {part2}");
}

fn part1(mut game: Game) -> usize {
    let mut counter = 0;
    while game.drop_sand() {
        counter = counter + 1;
    }
    game.print();
    counter
}

fn part2(mut game: Game) -> usize {
    let mut counter = 0;
    while game.drop_sand_part2() {
        counter = counter + 1;
    }
    game.print();
    counter
}

#[derive(Debug, Clone)]
struct Game {
    map: HashMap<Point, Content>,
}

impl Game {
    fn floor(&self) -> isize {
        self.map
            .iter()
            .filter(|e| e.1 == &Content::Rock)
            .map(|e| e.0)
            .map(|point| point.1)
            .max()
            .expect("map has no element")
    }

    fn drop_sand(&mut self) -> bool {
        let max_y = self.floor();
        let mut source = Point(500, 0);
        while let Some(p) = source
            .potential_next()
            .iter()
            .find(|&point| !self.map.contains_key(point))
        {
            if source.1 > max_y {
                return false;
            }
            source = *p;
        }
        // Mark the source as a sand
        self.map.insert(source, Content::Sand);
        true
    }

    fn drop_sand_part2(&mut self) -> bool {
        let max_y = self.floor() + 2;
        let mut source = Point(500, 0);
        while let Some(p) = source.potential_next().iter().find(|&point| {
            let Point(_, y) = *point;
            y != max_y && !self.map.contains_key(point)
        }) {
            if source.1 > max_y {
                return false;
            }
            source = *p;
        }
        if let Some(Content::Sand) = self.map.get(&Point(500, 0)) {
            return false;
        }
        // Mark the source as a sand
        self.map.insert(source, Content::Sand);
        true
    }

    fn print(&self) {
        let mut min_x = isize::MAX;
        let mut max_x = isize::MIN;
        let mut min_y = isize::MAX;
        let mut max_y = isize::MIN;
        for (p, content) in self.map.iter() {
            let Point(x, y) = *p;
            min_x = min_x.min(x);
            max_x = max_x.max(x);
            min_y = min_y.min(y);
            if content == &Content::Rock {
                max_y = max_y.max(y);
            }
        }

        for y in min_y..=max_y + 2 {
            let mut line = String::new();
            for x in min_x - 2..=max_x + 2 {
                if let Some(content) = self.map.get(&Point(x, y)) {
                    line.push(match *content {
                        Content::Rock => '#',
                        Content::Source => '+',
                        Content::Sand => 'o',
                    });
                } else if y == max_y + 2 {
                    line.push('#');
                } else {
                    line.push('.');
                }
            }
            println!("{line}");
        }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Content {
    Sand,
    Rock,
    Source,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Point(isize, isize);

impl Point {
    fn potential_next(&self) -> [Point; 3] {
        let Point(x, y) = self;
        [
            Point(*x, *y + 1),
            Point(*x - 1, *y + 1),
            Point(*x + 1, *y + 1),
        ]
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((xs, ys)) = s.split_once(',') {
            let x = xs.parse::<isize>().expect("not a number");
            let y = ys.parse::<isize>().expect("not a number");
            return Ok(Point(x, y));
        }
        Err(format!("not a point {s}"))
    }
}

fn parse_input(input: &str) -> Game {
    let mut grid = HashMap::new();
    for line in input.lines() {
        let parts = line
            .split(" -> ")
            .filter_map(|part| part.parse::<Point>().ok())
            .collect::<Vec<Point>>();
        for i in 0..parts.len() - 1 {
            let Point(p1x, p1y) = &parts[i];
            let Point(p2x, p2y) = &parts[i + 1];

            let r: RangeInclusive<isize>;
            if p1x == p2x {
                // same colum
                r = if p1y < p2y { *p1y..=*p2y } else { *p2y..=*p1y };
                for y in r {
                    grid.insert(Point(*p1x, y), Content::Rock);
                }
            } else {
                // same row
                r = if p1x < p2x { *p1x..=*p2x } else { *p2x..=*p1x };
                for x in r {
                    grid.insert(Point(x, *p1y), Content::Rock);
                }
            }
        }
    }
    grid.insert(Point(500, 0), Content::Source);
    Game { map: grid }
}

#[cfg(test)]
mod test {
    use crate::{parse_input, Content, Game, Point};

    #[test]
    fn loading_data() {
        let input = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9\
";
        let Game { map } = parse_input(input);
        assert_eq!(map.get(&Point(498, 4)), Some(&Content::Rock));
        assert_eq!(map.get(&Point(498, 5)), Some(&Content::Rock));
        assert_eq!(map.get(&Point(498, 6)), Some(&Content::Rock));
        assert_eq!(map.get(&Point(497, 6)), Some(&Content::Rock));
        assert_eq!(map.get(&Point(496, 6)), Some(&Content::Rock));

        assert_eq!(map.get(&Point(503, 4)), Some(&Content::Rock));
        assert_eq!(map.get(&Point(502, 4)), Some(&Content::Rock));
        assert_eq!(map.get(&Point(502, 5)), Some(&Content::Rock));
        assert_eq!(map.get(&Point(502, 6)), Some(&Content::Rock));
        assert_eq!(map.get(&Point(502, 7)), Some(&Content::Rock));
        assert_eq!(map.get(&Point(502, 8)), Some(&Content::Rock));
        assert_eq!(map.get(&Point(502, 9)), Some(&Content::Rock));
        assert_eq!(map.get(&Point(501, 9)), Some(&Content::Rock));
        assert_eq!(map.get(&Point(500, 9)), Some(&Content::Rock));
        assert_eq!(map.get(&Point(499, 9)), Some(&Content::Rock));
        assert_eq!(map.get(&Point(498, 9)), Some(&Content::Rock));
        assert_eq!(map.get(&Point(497, 9)), Some(&Content::Rock));
        assert_eq!(map.get(&Point(496, 9)), Some(&Content::Rock));
        assert_eq!(map.get(&Point(495, 9)), Some(&Content::Rock));
        assert_eq!(map.get(&Point(494, 9)), Some(&Content::Rock));
    }
}

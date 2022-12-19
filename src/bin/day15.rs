use std::collections::HashMap;

use regex::Regex;

fn main() {
    let part1 = part1();
    println!("part1 result: {part1}");
}

fn part1() -> usize {
    let data = include_str!("../../inputs/day15.txt");
    let lines: Vec<&str> = data.lines().collect();
    let y_that_matters = 2_000_000;
    Game::parse_part1(&lines, y_that_matters)
        .map
        .iter()
        .filter(|(&p, &content)| p.1 == y_that_matters && content == Content::NotABeaconForSure)
        .count()
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum Content {
    Sensor,
    Beacon,
    NotABeaconForSure,
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Hash, Debug, Copy, Clone)]
struct Point(isize, isize);

impl Point {
    fn distance_to(&self, other: Point) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

#[derive(Debug, Clone)]
struct Game {
    map: HashMap<Point, Content>,
}

impl Game {
    fn parse_part1(lines: &Vec<&str>, limit: isize) -> Self {
        let mut map = HashMap::new();
        let re = Regex::new(
            r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)",
        )
        .unwrap();
        for &line in lines.iter() {
            let captures = re.captures(line).unwrap();
            let sensor_x = captures
                .get(1)
                .map_or(0, |m| m.as_str().parse::<isize>().unwrap());
            let sensor_y = captures
                .get(2)
                .map_or(0, |m| m.as_str().parse::<isize>().unwrap());
            let beacon_x = captures
                .get(3)
                .map_or(0, |m| m.as_str().parse::<isize>().unwrap());
            let beacon_y = captures
                .get(4)
                .map_or(0, |m| m.as_str().parse::<isize>().unwrap());

            let sensor = Point(sensor_x, sensor_y);
            let beacon = Point(beacon_x, beacon_y);

            map.insert(sensor, Content::Sensor);
            map.insert(beacon, Content::Beacon);

            let distance_covered_by_sensor = sensor.distance_to(beacon);
            if sensor_y - distance_covered_by_sensor as isize <= limit
                && limit <= sensor_y + distance_covered_by_sensor as isize
            {
                let dist_that_matter = limit.abs_diff(sensor_y);
                let remaining: isize = (distance_covered_by_sensor - dist_that_matter) as isize;
                let x_range = sensor_x - remaining..=sensor_x + remaining;
                for x in x_range {
                    let p = Point(x, limit);
                    if !map.contains_key(&p) {
                        map.insert(p, Content::NotABeaconForSure);
                    }
                }
            }
        }

        Game { map }
    }
}

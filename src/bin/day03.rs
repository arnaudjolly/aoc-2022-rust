use std::collections::HashSet;

fn main() {
    let data = include_str!("../../inputs/day03.txt");
    let part1 = part1(data);
    println!("result(part1) = {part1}");
    let part2 = part2(data);
    println!("result(part2) = {part2}");
}

fn part1(data: &str) -> usize {
    data.lines()
        .map(|rucksack| {
            let len = rucksack.len();
            let (comp1, comp2) = rucksack.split_at(len / 2);
            let set: HashSet<char> = comp1.chars().collect();

            comp2
                .chars()
                .find(|c| set.contains(c))
                .expect("should at least have one in common")
        })
        .map(|it| to_points(it))
        .sum()
}

const POINTS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
fn to_points(item_type: char) -> usize {
    match POINTS.find(item_type) {
        Some(p) => p + 1,
        None => 0,
    }
}

fn part2(data: &str) -> usize {
    data.lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            // https://stackoverflow.com/a/67571893
            let [first, second, third] = <[&str; 3]>::try_from(chunk).ok().unwrap();
            let first_set: HashSet<char> = first.chars().collect();
            let second_set: HashSet<char> = second.chars().collect();

            third
                .chars()
                .find(|c| first_set.contains(c) && second_set.contains(c))
                .expect("should have at least a common char in the group")
        })
        .map(|it| to_points(it))
        .sum()
}

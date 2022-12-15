fn main() {
    let data = include_str!("../../inputs/day08.txt");
    let lines: Vec<&str> = data.lines().collect();

    let part1 = part1(&lines);
    println!("result(part1) = {part1}");

    let part2 = part2(&lines);
    println!("result(part2) = {part2}");
}

struct Puzzle {
    tree_heights: Vec<String>,
}

impl Puzzle {
    fn visible(&self, x: usize, y: usize) -> bool {
        self.visible_from_right(x, y)
            || self.visible_from_left(x, y)
            || self.visible_from_top(x, y)
            || self.visible_from_bottom(x, y)
    }

    fn visible_from_top(&self, x: usize, y: usize) -> bool {
        if self.is_on_the_edge(x, y) {
            // on the edge
            return true;
        }
        let row = self.tree_heights.iter().nth(y).unwrap();
        let ch = row.chars().nth(x);
        let max_from_top = self
            .tree_heights
            .iter()
            .take(y)
            .filter_map(|row| row.chars().nth(x))
            .max();
        ch > max_from_top
    }

    fn is_on_the_edge(&self, x: usize, y: usize) -> bool {
        x == 0 || y == 0 || x == self.tree_heights[0].len() - 1 || y == self.tree_heights.len() - 1
    }

    fn visible_from_bottom(&self, x: usize, y: usize) -> bool {
        if self.is_on_the_edge(x, y) {
            // on the edge
            return true;
        }
        let row = self.tree_heights.iter().nth(y).unwrap();
        let ch = row.chars().nth(x);
        let max_from_bottom = self
            .tree_heights
            .iter()
            .rev()
            .take(self.tree_heights.len() - y - 1)
            .filter_map(|row| row.chars().nth(x))
            .max();
        ch > max_from_bottom
    }

    fn visible_from_right(&self, x: usize, y: usize) -> bool {
        if self.is_on_the_edge(x, y) {
            // on the edge
            return true;
        }
        let row = self.tree_heights.iter().nth(y).unwrap();
        let ch = row.chars().nth(x);
        let max_from_right = row[x + 1..].chars().max();
        ch > max_from_right
    }

    fn visible_from_left(&self, x: usize, y: usize) -> bool {
        if self.is_on_the_edge(x, y) {
            // on the edge
            return true;
        }
        let row = self.tree_heights.iter().nth(y).unwrap();
        let ch = row.chars().nth(x);
        let max_from_left = row[..x].chars().max();
        ch > max_from_left
    }

    fn nb_trees_visible_on_left(&self, x: usize, y: usize) -> usize {
        let row = self.tree_heights.iter().nth(y).unwrap();
        let ch = row.chars().nth(x);
        let path_to_tree = row.chars().take(x).collect::<Vec<char>>();
        let trees_visible_on_left = path_to_tree
            .iter()
            .rev()
            .take_while(|&c| Some(*c) < ch)
            .count();

        if trees_visible_on_left < x {
            // we hit a tree on the road, count that tree
            trees_visible_on_left + 1
        } else {
            trees_visible_on_left
        }
    }

    fn nb_trees_visible_on_right(&self, x: usize, y: usize) -> usize {
        let row = self.tree_heights.iter().nth(y).unwrap();
        let ch = row.chars().nth(x);
        let trees_visible_on_right = row
            .chars()
            .skip(x + 1)
            .take_while(|c| Some(*c) < ch)
            .count();
        if trees_visible_on_right < row.len() - x - 1 {
            // we hit a tree on the road, count that tree
            trees_visible_on_right + 1
        } else {
            trees_visible_on_right
        }
    }

    fn nb_trees_visible_on_top(&self, x: usize, y: usize) -> usize {
        let row = self.tree_heights.iter().nth(y).unwrap();
        let ch = row.chars().nth(x);
        let impacted_rows: Vec<&String> = self.tree_heights.iter().take(y).collect();
        let trees_visible_on_top = impacted_rows
            .iter()
            .rev()
            .map(|r| r.chars().nth(x))
            .take_while(|c| *c < ch)
            .count();
        if trees_visible_on_top < y {
            // we hit a tree on the road, count that tree
            trees_visible_on_top + 1
        } else {
            trees_visible_on_top
        }
    }

    fn nb_trees_visible_on_bottom(&self, x: usize, y: usize) -> usize {
        let row = self.tree_heights.iter().nth(y).unwrap();
        let ch = row.chars().nth(x);
        let trees_visible_on_bottom = self
            .tree_heights
            .iter()
            .skip(y + 1)
            .map(|r| r.chars().nth(x))
            .take_while(|c| *c < ch)
            .count();

        if trees_visible_on_bottom < self.tree_heights.len() - y - 1 {
            // we hit a tree on the road, count that tree
            trees_visible_on_bottom + 1
        } else {
            trees_visible_on_bottom
        }
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        let trees_visible_on_left = self.nb_trees_visible_on_left(x, y);
        let trees_visible_on_right = self.nb_trees_visible_on_right(x, y);
        let trees_visible_on_top = self.nb_trees_visible_on_top(x, y);
        let trees_visible_on_bottom = self.nb_trees_visible_on_bottom(x, y);

        trees_visible_on_bottom
            * trees_visible_on_top
            * trees_visible_on_left
            * trees_visible_on_right
    }
}

fn parse_input(lines: &Vec<&str>) -> Puzzle {
    let heights = lines.iter().map(|&s| String::from(s)).collect();
    Puzzle {
        tree_heights: heights,
    }
}

fn part1(lines: &Vec<&str>) -> usize {
    let p = parse_input(lines);

    let mut count = 2 * p.tree_heights[0].len() + 2 * p.tree_heights.len() - 4;
    for x in 1..p.tree_heights[0].len() - 1 {
        for y in 1..p.tree_heights.len() - 1 {
            if p.visible(x, y) {
                count += 1;
            }
        }
    }
    count
}

fn part2(lines: &Vec<&str>) -> usize {
    let p = parse_input(lines);

    let mut max = 0;
    for x in 0..p.tree_heights[0].len() {
        for y in 0..p.tree_heights.len() {
            max = max.max(p.scenic_score(x, y));
        }
    }

    max
}

#[cfg(test)]
mod test {
    use crate::{parse_input, part1, part2};

    #[test]
    fn part_1_sample() {
        let lines = include_str!("../../inputs/day08.sample.txt")
            .lines()
            .collect();
        assert_eq!(part1(&lines), 21);
    }

    #[test]
    fn part_2_sample() {
        let lines = include_str!("../../inputs/day08.sample.txt")
            .lines()
            .collect();
        assert_eq!(part2(&lines), 8);
    }

    #[test]
    fn scenic_score_1_1() {
        let lines = include_str!("../../inputs/day08.sample.txt")
            .lines()
            .collect();
        let puzzle = parse_input(&lines);
        assert_eq!(puzzle.scenic_score(1, 1), 1);
    }

    #[test]
    fn scenic_score_1_2() {
        let lines = include_str!("../../inputs/day08.sample.txt")
            .lines()
            .collect();
        let puzzle = parse_input(&lines);
        assert_eq!(puzzle.scenic_score(1, 2), 6);
    }

    #[test]
    fn scenic_score_1_3() {
        let lines = include_str!("../../inputs/day08.sample.txt")
            .lines()
            .collect();
        let puzzle = parse_input(&lines);
        assert_eq!(puzzle.scenic_score(1, 3), 1);
    }

    #[test]
    fn scenic_score_2_1() {
        let lines = include_str!("../../inputs/day08.sample.txt")
            .lines()
            .collect();
        let puzzle = parse_input(&lines);
        assert_eq!(puzzle.scenic_score(2, 1), 4);
    }

    #[test]
    fn scenic_score_2_3() {
        let lines = include_str!("../../inputs/day08.sample.txt")
            .lines()
            .collect();
        let puzzle = parse_input(&lines);
        assert_eq!(puzzle.scenic_score(2, 3), 8);
    }

    #[test]
    fn scenic_score_4_3() {
        let lines = include_str!("../../inputs/day08.sample.txt")
            .lines()
            .collect();
        let puzzle = parse_input(&lines);
        assert_eq!(puzzle.scenic_score(4, 3), 0);
    }
}

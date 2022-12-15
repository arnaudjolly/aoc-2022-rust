use pathfinding::prelude::dijkstra;
use std::cmp::Ordering;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Copy)]
struct Node(usize, usize);

impl Node {
    fn successors(&self, grid: &Vec<Vec<usize>>) -> Vec<(Node, usize)> {
        let &Node(x, y) = self;
        let our_value = grid[y][x];
        let x_max = grid[0].len() - 1;
        let y_max = grid.len() - 1;
        let mut potential_successors: Vec<Node> = vec![];
        if x != 0 {
            potential_successors.push(Node(x - 1, y));
        }
        if x != x_max {
            potential_successors.push(Node(x + 1, y));
        }
        if y != 0 {
            potential_successors.push(Node(x, y - 1));
        }
        if y != y_max {
            potential_successors.push(Node(x, y + 1));
        }

        potential_successors
            .iter()
            .filter_map(|&node| test(node, grid, &our_value))
            .collect()
    }
}

fn test(node: Node, grid: &Vec<Vec<usize>>, actual: &usize) -> Option<(Node, usize)> {
    let Node(x, y) = node;
    let target = grid[y][x];
    match target.cmp(actual) {
        Ordering::Equal => Some((node, 1)),
        Ordering::Less => Some((node, 1)),
        Ordering::Greater => {
            if target - actual <= 1 {
                Some((node, 1))
            } else {
                None
            }
        }
    }
}

fn main() {
    let data = include_str!("../../inputs/day12.txt");

    let mut grid: Vec<Vec<usize>> = vec![];
    let mut start = Node(0, 0);
    let mut goal = Node(0, 0);
    for (y, line) in data.lines().enumerate() {
        grid.push(
            line.chars()
                .enumerate()
                .map(|(x, ch)| match ch {
                    'S' => {
                        start = Node(x, y);
                        0
                    }
                    'E' => {
                        goal = Node(x, y);
                        'z' as usize - 'a' as usize
                    }
                    _ => ch as usize - 'a' as usize,
                })
                .collect(),
        );
    }

    println!("result part 1: {}", part1(&start, &goal, &grid));
    println!("result part 2: {}", part2(&goal, &grid));
}

fn part1(start: &Node, goal: &Node, grid: &Vec<Vec<usize>>) -> usize {
    let (_, weight) =
        dijkstra(start, |p| p.successors(&grid), |p| *p == *goal).expect("no path found");
    weight
}

fn part2(goal: &Node, grid: &Vec<Vec<usize>>) -> usize {
    // for part 2, we will try each start node that is at elevation "a" and take the min from them
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            // create the Nodes for elevation "a" (e.g. value is 0)
            row.iter()
                .enumerate()
                .filter(|(_, v)| **v == 0)
                // compiler told me to consider using "move" here for capturing "y", i don't master that part yet but it works
                .map(move |(x, _)| Node(x, y))
        })
        // some clusters of "a" can't join the "z" so just ignore them if we start in one of these clusters
        .filter_map(|start| dijkstra(&start, |p| p.successors(&grid), |p| *p == *goal))
        .map(|(_, weight)| weight)
        .min()
        .unwrap()
}

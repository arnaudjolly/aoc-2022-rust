fn main() {
    let data = include_str!("../../inputs/day11.txt");
    let input = data
        .lines()
        .filter(|l| !l.is_empty())
        .collect::<Vec<&str>>();

    println!("Part 1 result: {}", part1(&input));
    println!("Part 2 result: {}", part2(&input));
}

fn part1(input: &Vec<&str>) -> usize {
    handle_puzzle(&input, 20, true)
}

fn part2(input: &Vec<&str>) -> usize {
    handle_puzzle(&input, 10_000, false)
}

fn handle_puzzle(input: &Vec<&str>, nb_rounds: usize, is_part_1: bool) -> usize {
    let mut monkeys = parse_monkeys(&input);
    // for part2, you multiple super big numbers so you have to find a number that will not change
    // the results of all the tests for monkeys.
    // staying in Z/nZ with n=product(divisors) will keep enough "range" to cover each divisibility tests for those divisors
    let keep_cool_modulo = monkeys
        .iter()
        .map(|m| m.test.divisible_by)
        .product::<usize>();
    for _ in 0..nb_rounds {
        for i in 0..monkeys.len() {
            monkeys = handle_turn_of_monkey_x(monkeys, i, is_part_1, keep_cool_modulo);
        }
    }
    let mut inspected_items = monkeys
        .iter()
        .map(|monkey| monkey.nb_inspected_items)
        .collect::<Vec<usize>>();
    inspected_items.sort_by(|a, b| b.cmp(a));
    inspected_items.iter().take(2).product()
}

fn handle_turn_of_monkey_x(
    monkeys: Vec<Monkey>,
    x: usize,
    divide_by_3: bool,
    stay_cool: usize,
) -> Vec<Monkey> {
    let mut new_monkeys = monkeys;
    let mut monkey = new_monkeys[x].clone();
    for item in &monkey.items {
        let mut worry_level = match monkey.operation {
            Operation::Times(o) => *item * o,
            Operation::Add(o) => *item + o,
            Operation::Square => *item * *item,
        };
        if divide_by_3 {
            worry_level = worry_level / 3;
        } else {
            worry_level = worry_level % stay_cool;
        }
        let target_monkey_idx = if worry_level % monkey.test.divisible_by == 0 {
            monkey.test.is_true
        } else {
            monkey.test.is_false
        };
        new_monkeys[target_monkey_idx].items.push(worry_level);
        monkey.nb_inspected_items += 1;
    }
    monkey.items.clear();
    new_monkeys[x] = monkey;
    new_monkeys
}

#[derive(Copy, Clone, PartialEq)]
enum Operation {
    Times(usize),
    Add(usize),
    Square,
}

#[derive(Copy, Clone, PartialEq)]
struct Test {
    divisible_by: usize,
    is_true: usize,
    is_false: usize,
}

#[derive(Clone)]
struct Monkey {
    items: Vec<usize>,
    operation: Operation,
    test: Test,
    nb_inspected_items: usize,
}

impl PartialEq for Monkey {
    fn eq(&self, other: &Self) -> bool {
        self.items.eq(&other.items)
            && self.operation == other.operation
            && self.test == other.test
            && self.nb_inspected_items == other.nb_inspected_items
    }
}

fn parse_monkeys(lines: &Vec<&str>) -> Vec<Monkey> {
    lines
        .chunks(6)
        .map(|monkey_lines| {
            let items = monkey_lines[1][18..]
                .split(", ")
                .map(|worry_level| worry_level.parse::<usize>().unwrap())
                .fold(vec![], |mut acc, it| {
                    acc.push(it);
                    acc
                });
            let op_line = &monkey_lines[2][23..];
            let operation = match op_line {
                op if op.starts_with("+") => {
                    Operation::Add(op[2..].parse::<usize>().expect("not a number"))
                }
                op if op.starts_with("* old") => Operation::Square,
                op if op.starts_with("*") => {
                    Operation::Times(op[2..].parse::<usize>().expect("not a number"))
                }
                _ => panic!("can't figure out the operation for a monkey"),
            };
            let test = monkey_lines[3][21..]
                .parse::<usize>()
                .expect("not a number");
            let if_true = monkey_lines[4][29..]
                .parse::<usize>()
                .expect("not a number");
            let if_false = monkey_lines[5][30..]
                .parse::<usize>()
                .expect("not a number");

            Monkey {
                items,
                operation,
                test: Test {
                    divisible_by: test,
                    is_true: if_true,
                    is_false: if_false,
                },
                nb_inspected_items: 0,
            }
        })
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{parse_monkeys, part1, part2, Monkey, Operation, Test};

    #[test]
    fn parse_one_monkey() {
        let input = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3
";
        let monkeys = parse_monkeys(&input.lines().collect());
        assert_eq!(
            true,
            monkeys.contains(&Monkey {
                items: vec![79, 98],
                operation: Operation::Times(19),
                test: Test {
                    divisible_by: 23,
                    is_true: 2,
                    is_false: 3
                },
                nb_inspected_items: 0
            })
        );
    }

    #[test]
    fn part1_sample() {
        let data = include_str!("../../inputs/day11.sample.txt");
        let input = data
            .lines()
            .filter(|l| !l.is_empty())
            .collect::<Vec<&str>>();

        assert_eq!(10_605, part1(&input));
    }

    #[test]
    fn part2_sample() {
        let data = include_str!("../../inputs/day11.sample.txt");
        let input = data
            .lines()
            .filter(|l| !l.is_empty())
            .collect::<Vec<&str>>();

        assert_eq!(2_713_310_158, part2(&input));
    }
}

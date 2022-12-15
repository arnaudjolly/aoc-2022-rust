use std::str::FromStr;

fn main() {
    let data = include_str!("../../inputs/day02.txt");
    let part1 = part1(data);
    println!("result(part1) = {part1}");

    let part2 = part2(data);
    println!("result(part2) = {part2}");
}

fn part1(data: &str) -> usize {
    data.lines()
        .map(|line| {
            let moves = line.split_once(" ").unwrap();
            let (opponent_move, our_move) = (
                moves.0.parse::<Move>().unwrap(),
                moves.1.parse::<Move>().unwrap(),
            );
            let outcome = outcome(opponent_move, our_move);
            our_move as usize + outcome as usize
        })
        .sum()
}

fn outcome(op_move: Move, our_move: Move) -> Outcome {
    match (op_move, our_move) {
        (Move::Rock, Move::Rock) => Outcome::Draw,
        (Move::Rock, Move::Paper) => Outcome::Win,
        (Move::Rock, Move::Scissors) => Outcome::Loss,
        (Move::Paper, Move::Rock) => Outcome::Loss,
        (Move::Paper, Move::Paper) => Outcome::Draw,
        (Move::Paper, Move::Scissors) => Outcome::Win,
        (Move::Scissors, Move::Rock) => Outcome::Win,
        (Move::Scissors, Move::Paper) => Outcome::Loss,
        (Move::Scissors, Move::Scissors) => Outcome::Draw,
    }
}

fn part2(data: &str) -> usize {
    data.lines()
        .map(|line| {
            let letters = line.split_once(" ").unwrap();
            let (opponent_move, expected_outcome) = (
                letters.0.parse::<Move>().unwrap(),
                letters.1.parse::<Outcome>().unwrap(),
            );
            let our_move = guess_move_to_play(expected_outcome, opponent_move);
            expected_outcome as usize + our_move as usize
        })
        .sum()
}

fn guess_move_to_play(expected_outcome: Outcome, opponent_move: Move) -> Move {
    match expected_outcome {
        Outcome::Loss => match opponent_move {
            Move::Rock => Move::Scissors,
            Move::Paper => Move::Rock,
            Move::Scissors => Move::Paper,
        },
        Outcome::Draw => opponent_move,
        Outcome::Win => match opponent_move {
            Move::Rock => Move::Paper,
            Move::Paper => Move::Scissors,
            Move::Scissors => Move::Rock,
        },
    }
}

#[derive(Copy, Clone)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Move {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err(String::from("unexpected move")),
        }
    }
}

#[derive(Copy, Clone)]
enum Outcome {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

impl FromStr for Outcome {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Loss),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err("unknown outcome".to_string()),
        }
    }
}

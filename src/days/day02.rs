use crate::util;

pub fn run() {
    let raw_input = util::read_input("inputs/day02.txt").unwrap();
    let input = process(raw_input.clone());
    println!("part 1: {}", part_1(&input));
    let p2_input = process_p2(raw_input);
    println!("part 2: {}", part_1(&p2_input));
}

fn part_1(input: &[Round]) -> u64 {
    input.iter()
        .fold(0, |total_score, round| total_score + calculate_score(round))
}

#[derive(Debug)]
enum Selection {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl From<&str> for Outcome {
    fn from(value: &str) -> Self {
        match value {
            "X" => Outcome::Lose,
            "Y" => Outcome::Draw,
            "Z" => Outcome::Win,
            _ => panic!("unreachable")
        }
    }
}

type Round = (Selection, Selection);

impl From<&str> for Selection {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => Selection::Rock,
            "B" | "Y" => Selection::Paper,
            "C" | "Z" => Selection::Scissors,
            _ => panic!("unreachable")
        }
    }
}

fn process(input: String) -> Vec<Round> {
    input.lines()
        .map(|r_str| {
             let (other, me) = r_str.split_once(' ').unwrap();
            (Selection::from(other), Selection::from(me))
        })
        .collect()
}

fn process_p2(input: String) -> Vec<Round> {
    input.lines()
        .map(|r_str| {
            let (play, desired_outcome) = r_str.split_once(' ').unwrap();
            let opponent_selection = Selection::from(play);
            let desired_outcome = Outcome::from(desired_outcome);
            calculate_your_play((opponent_selection, desired_outcome))
        })
        .collect()
}

fn calculate_score(round: &Round) -> u64 {
    match round {
        (Selection::Rock, Selection::Rock) => 4,
        (Selection::Rock, Selection::Paper) => 8,
        (Selection::Rock, Selection::Scissors) => 3,
        (Selection::Paper, Selection::Rock) => 1,
        (Selection::Paper, Selection::Paper) => 5,
        (Selection::Paper, Selection::Scissors) => 9,
        (Selection::Scissors, Selection::Rock) => 7,
        (Selection::Scissors, Selection::Paper) => 2,
        (Selection::Scissors, Selection::Scissors) => 6,
    }

}

fn calculate_your_play(desired: (Selection, Outcome)) -> Round {
    match desired {
        (Selection::Rock, Outcome::Lose) => (Selection::Rock, Selection::Scissors),
        (Selection::Rock, Outcome::Draw) => (Selection::Rock, Selection::Rock),
        (Selection::Rock, Outcome::Win) => (Selection::Rock, Selection::Paper),
        (Selection::Paper, Outcome::Lose) => (Selection::Paper, Selection::Rock),
        (Selection::Paper, Outcome::Draw) => (Selection::Paper, Selection::Paper),
        (Selection::Paper, Outcome::Win) => (Selection::Paper, Selection::Scissors),
        (Selection::Scissors, Outcome::Lose) => (Selection::Scissors, Selection::Paper),
        (Selection::Scissors, Outcome::Draw) => (Selection::Scissors, Selection::Scissors),
        (Selection::Scissors, Outcome::Win) => (Selection::Scissors, Selection::Rock),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = process(util::read_input("inputs/day02.txt").unwrap());
        b.iter(|| part_1(&input));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = process_p2(util::read_input("inputs/day02.txt").unwrap());
        b.iter(|| part_1(&input));
    }
}

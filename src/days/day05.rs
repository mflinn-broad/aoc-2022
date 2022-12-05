use std::collections::VecDeque;

use crate::util;

pub fn run() {
    let raw_input = util::read_input("inputs/day05.txt").unwrap();
    let mut cargo = init_cargo();
    let instructions = parse(raw_input);
    instructions
        .iter()
        .for_each(|instruction| cargo.execute_move(instruction));
    println!("part 1: {}", cargo.tops());
}

fn parse(input: String) -> Vec<MoveInstruction> {
    let (_, input) = input.split_once("\n\n").unwrap();
    input.lines().map(MoveInstruction::from).collect()
}

#[derive(Debug)]
struct Cargo {
    stacks: Vec<Vec<char>>,
}

impl Cargo {
    fn new(stacks: Vec<Vec<char>>) -> Self {
        Self { stacks }
    }

    fn execute_move(&mut self, instruction: &MoveInstruction) {
        let mut temp_storage = VecDeque::new();
        for _ in 0..instruction.amount {
            if let Some(item) = self.stacks[instruction.source].pop() {
                temp_storage.push_front(item);
            } else {
                break;
            }
        }
        self.stacks[instruction.destination].extend(temp_storage.iter().copied());
    }

    fn tops(&self) -> String {
        let mut message = String::new();
        for stack in self.stacks.iter() {
            message.push(stack.last().copied().unwrap())
        }
        message
    }
}

#[derive(Debug)]
struct MoveInstruction {
    source: usize,
    destination: usize,
    amount: u32,
}

impl From<&str> for MoveInstruction {
    fn from(value: &str) -> Self {
        let mut word_iter = value.split_ascii_whitespace();
        word_iter.next();
        let amount = word_iter.next().unwrap().parse().unwrap();
        word_iter.next();
        let source: usize = word_iter.next().unwrap().parse().unwrap();
        word_iter.next();
        let destination: usize = word_iter.next().unwrap().parse().unwrap();

        Self {
            source: source - 1,
            destination: destination - 1,
            amount,
        }
    }
}

fn init_cargo() -> Cargo {
    let stacks = "HRLBDZFLS
TBMZR
ZLCHNS
SCFJ
PGHWRZB
VJZGDNMT
GLNWFSPQ
MZR
MCLGVRT";

    let stacks: Vec<Vec<char>> = stacks
        .lines()
        .map(|stack| stack.chars().collect())
        .collect();

    Cargo::new(stacks)
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day05.txt").unwrap();
        let instructions = parse(raw_input);
        b.iter(|| {
            let mut cargo = init_cargo();
            instructions
                .iter()
                .for_each(|instruction| cargo.execute_move(instruction))
        })
    }
}

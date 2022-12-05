use std::collections::VecDeque;

use crate::util;

pub fn run() {
    let raw_input = util::read_input("inputs/day05.txt").unwrap();
    let (mut cargo, instructions) = parse(raw_input);
    instructions
        .iter()
        .for_each(|instruction| cargo.execute_move(instruction));
    println!("part 1: {}", cargo.tops());
}

fn parse(input: String) -> (Cargo, Vec<MoveInstruction>) {
    let (start, instructions) = input.split_once("\n\n").unwrap();
    let cargo = parse_starting_pos(start);
    let instructions = instructions.lines().map(MoveInstruction::from).collect();
    (cargo, instructions)
}

fn parse_starting_pos(input: &str) -> Cargo {
    let mut stack_data = input.lines().rev();
    let num_stacks = stack_data
        .next()
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .count();

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_stacks];
    for row in stack_data {
        for (col, c) in row.chars().skip(1).enumerate() {
            match c {
                'A'..='Z' => stacks[col / 4].push(c),
                _ => continue,
            }
        }
    }
    Cargo::new(stacks)
}

#[derive(Debug, Clone)]
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

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day05.txt").unwrap();
        let (cargo, instructions) = parse(raw_input);
        b.iter(|| {
            let mut cargo_ = cargo.clone();
            instructions
                .iter()
                .for_each(|instruction| cargo_.execute_move(instruction))
        })
    }
}

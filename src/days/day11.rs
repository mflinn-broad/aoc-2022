use crate::util;
use std::collections::VecDeque;

pub fn run() {
    let raw_input = util::read_input("inputs/day11.txt").unwrap();
    let mut monkeys = parse(raw_input);
    println!("part 1: {}", part_1(&mut monkeys.clone()));
    println!("part 2: {}", part_2(&mut monkeys));
}

fn part_2(monkeys: &mut Monkeys) -> usize {
    monkeys.process_rounds(10_000);
    monkeys.monkey_business()
}

fn part_1(monkeys: &mut Monkeys) -> usize {
    monkeys.process_rounds(20);
    monkeys.monkey_business()
}

fn parse(input: String) -> Monkeys {
    let monkeys = input.split("\n\n").map(Monkey::from).collect();

    Monkeys { monkeys }
}

#[derive(Clone)]
struct Monkeys {
    monkeys: Vec<Monkey>,
}

impl Monkeys {
    fn process_rounds(&mut self, num_rounds: usize) {
        let worry_reducer: usize = self.monkeys.iter().map(|monkey| monkey.test).product();
        for _ in 0..num_rounds {
            for monkey_idx in 0..self.monkeys.len() {
                let mut monkey = self.monkeys[monkey_idx].clone();
                for _ in 0..monkey.items.len() {
                    monkey.items_inspected += 1;
                    let item = monkey.op.apply(monkey.items.pop_front().unwrap()) % worry_reducer;
                    if item % monkey.test == 0 {
                        self.monkeys[monkey.op_true].items.push_back(item);
                    } else {
                        self.monkeys[monkey.op_false].items.push_back(item);
                    }
                }
                self.monkeys[monkey_idx] = monkey;
            }
        }
    }

    fn monkey_business(&self) -> usize {
        let mut monkeys = self.monkeys.clone();
        monkeys.sort_by(|x, y| y.items_inspected.cmp(&x.items_inspected));

        monkeys
            .iter()
            .take(2)
            .map(|monkey| monkey.items_inspected)
            .product()
    }
}

#[derive(Debug, Clone)]
struct Monkey {
    items_inspected: usize,
    test: usize,
    op: Op,
    op_true: usize,
    op_false: usize,
    items: VecDeque<usize>,
}

impl From<&str> for Monkey {
    fn from(value: &str) -> Self {
        let mut monkey_lines = value.lines().skip(1);
        let items_str = monkey_lines.next().unwrap();
        let (_, items) = items_str.split_once(':').unwrap();
        let items: VecDeque<usize> = items
            .trim()
            .split(", ")
            .map(|item| item.parse().unwrap())
            .collect();
        let op: Op = monkey_lines.next().unwrap().into();
        let test = monkey_lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let op_true = monkey_lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();
        let op_false = monkey_lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        Self {
            items_inspected: 0,
            test,
            op,
            op_true,
            op_false,
            items,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add(usize),
    Mul(usize),
    Sq,
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        let value = value.trim();
        let mut op_vals = value.split_ascii_whitespace().rev().take(2);
        match op_vals.next().unwrap().parse() {
            Err(_) => Op::Sq,
            Ok(num) => match op_vals.next().unwrap() {
                "+" => Op::Add(num),
                _ => Op::Mul(num),
            },
        }
    }
}

impl Op {
    fn apply(&self, item: usize) -> usize {
        match self {
            Op::Add(val) => item + val,
            Op::Mul(val) => item * val,
            Op::Sq => item * item,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = util::read_input("inputs/day11.txt").unwrap();
        b.iter(|| {
            let mut monkeys = parse(input.clone());
            part_1(&mut monkeys);
        })
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = util::read_input("inputs/day11.txt").unwrap();
        b.iter(|| {
            let mut monkeys = parse(input.clone());
            part_2(&mut monkeys);
        })
    }
}

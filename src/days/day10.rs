use std::collections::VecDeque;

use crate::util;

const TARGET_CYCLES: [usize; 6] = [20, 60, 100, 140, 180, 220];

pub fn run() {
    let raw_input = util::read_input("inputs/day10.txt").unwrap();
    let mut input = parse(raw_input);
    println!("part 1: {}", part_1(&mut input.clone()));
    println!("part: 2");
    for line in part_2(&mut input).iter() {
        println!("{}", String::from_iter(line.iter()));
    }
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl From<&str> for Instruction {
    fn from(value: &str) -> Self {
        let instruction_name = &value[0..4];
        match instruction_name {
            "noop" => Instruction::Noop,
            "addx" => Instruction::Addx(value[5..].parse().unwrap()),
            _ => panic!("invalid instruction"),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CPUState {
    Idle,
    Busy,
}

#[derive(Debug)]
struct Cpu {
    x_register: i64,
    cycle_count: usize,
    curr_state: CPUState,
    curr_instruction: Option<Instruction>,
}

impl Cpu {
    fn new() -> Self {
        Self {
            x_register: 1,
            cycle_count: 0,
            curr_state: CPUState::Idle,
            curr_instruction: None,
        }
    }

    fn is_busy(&self) -> bool {
        if self.curr_state == CPUState::Busy {
            return true;
        }
        false
    }

    fn load_instruction(&mut self, instruction: &Instruction) -> Option<()> {
        // load instruction begins a cycle
        self.cycle_count += 1;
        if self.curr_instruction.is_some() {
            return None;
        }
        self.curr_instruction = Some(instruction.to_owned());
        Some(())
    }

    fn tick(&mut self) {
        if self.is_busy() {
            if let Some(instruction) = self.curr_instruction {
                match instruction {
                    Instruction::Addx(val) => self.x_register += val,
                    Instruction::Noop => (), // this should never happen
                }
                self.curr_state = CPUState::Idle;
                self.curr_instruction = None;
            }
        } else {
            match self.curr_instruction.unwrap() {
                Instruction::Addx(_) => self.curr_state = CPUState::Busy,
                _ => self.curr_instruction = None,
            }
        }
    }
}

fn parse(input: String) -> VecDeque<Instruction> {
    input.lines().map(Instruction::from).collect()
}

fn part_1(instructions: &mut VecDeque<Instruction>) -> i64 {
    let mut signal_strength = 0;
    let mut cpu = Cpu::new();
    for _ in 0..220 {
        // try to load next instruction
        let next_instruction = instructions.pop_front().unwrap();
        match cpu.load_instruction(&next_instruction) {
            Some(_) => (),
            // instruction already loaded, put it back on the queue
            None => instructions.push_front(next_instruction),
        }

        if TARGET_CYCLES.contains(&cpu.cycle_count) {
            signal_strength += cpu.cycle_count as i64 * cpu.x_register;
        }

        cpu.tick();
    }
    signal_strength
}

fn part_2(instructions: &mut VecDeque<Instruction>) -> Vec<Vec<char>> {
    let mut crt = vec![vec![' '; 40]; 6];
    let mut cpu = Cpu::new();
    for _ in 0..240 {
        let next_instruction = instructions.pop_front().unwrap();
        match cpu.load_instruction(&next_instruction) {
            Some(_) => (),
            None => instructions.push_front(next_instruction),
        }
        let curr_row = cpu.cycle_count / 40;
        let curr_col = (cpu.cycle_count - 1) % 40;
        let curr_col_i64 = curr_col as i64;
        if (cpu.x_register - 1..=cpu.x_register + 1).contains(&curr_col_i64) {
            crt[curr_row][curr_col] = '#';
        }
        cpu.tick();
    }
    crt
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day10.txt").unwrap();
        b.iter(|| {
            let mut input = parse(raw_input.clone());
            part_1(&mut input);
        })
    }
}

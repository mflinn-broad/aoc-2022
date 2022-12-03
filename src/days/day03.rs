use crate::util;
use itertools::Itertools;
use std::collections::HashSet;

pub fn run() {
    let input = util::read_input("inputs/day03.txt").unwrap();
    println!("part 1: {}", part_1(input.clone()));
    println!("part 2: {}", part_2(input));
}

#[derive(Debug)]
struct Rucksack {
    first: HashSet<char>,
    second: HashSet<char>,
}

impl Rucksack {
    fn new(contents: &str) -> Self {
        let (first, second) = contents.split_at(contents.len() / 2);
        let first = HashSet::from_iter(first.chars());
        let second = HashSet::from_iter(second.chars());
        Rucksack { first, second }
    }

    fn common(&self) -> char {
        self.first
            .intersection(&self.second)
            .next()
            .copied()
            .unwrap()
    }
}

fn part_1(input: String) -> u32 {
    input
        .lines()
        .map(|ruck_str| Rucksack::new(ruck_str).common())
        .fold(0, |total_priority, shared| {
            total_priority + priority(shared)
        })
}

fn priority(item: char) -> u32 {
    match item {
        'a'..='z' => ((item as u32 - 71) % 26) + 1,
        'A'..='Z' => ((item as u32 - 65) % 26) + 27,
        _ => panic!("unreachable"),
    }
}

struct Group {
    a: HashSet<char>,
    b: HashSet<char>,
    c: HashSet<char>,
}

impl Group {
    fn new(a: &str, b: &str, c: &str) -> Self {
        let a = HashSet::from_iter(a.chars());
        let b = HashSet::from_iter(b.chars());
        let c = HashSet::from_iter(c.chars());
        Group { a, b, c }
    }

    fn common(&self) -> char {
        let a_b_shared: HashSet<char> = self.a.intersection(&self.b).copied().collect();
        a_b_shared.intersection(&self.c).next().copied().unwrap()
    }
}

fn part_2(input: String) -> u32 {
    let lines: Vec<&str> = input.lines().collect_vec();
    lines
        .chunks(3)
        .map(|group_lines| Group::new(group_lines[0], group_lines[1], group_lines[2]).common())
        .fold(0, |total_priority, shared| {
            total_priority + priority(shared)
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[test]
    fn test_part_1() {
        let test_input = String::from(
            "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
        );
        assert_eq!(part_1(test_input), 157);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = util::read_input("inputs/day03.txt").unwrap();
        b.iter(|| part_1(input.clone()));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = util::read_input("inputs/day03.txt").unwrap();
        b.iter(|| part_2(input.clone()));
    }
}

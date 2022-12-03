use crate::util;
use itertools::Itertools;
use std::collections::HashSet;

pub fn run() {
    let raw_input = util::read_input("inputs/day03.txt").unwrap();
    let input = process(raw_input.clone());
    println!("part 1: {}", part_1(&input));
    let p2_input = process_p2(raw_input);
    println!("part 2: {}", part_2(&p2_input));
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
}

fn process(input: String) -> Vec<Rucksack> {
    input
        .lines()
        .map(Rucksack::new)
        .collect()
}

fn priority(item: char) -> u32 {
    match item {
        'a'..='z' => ((item as u32 - 71) % 26) + 1,
        'A'..='Z' => ((item as u32 - 65) % 26) + 27,
        _ => panic!("unreachable"),
    }
}

fn part_1(rucksacks: &[Rucksack]) -> u32 {
    rucksacks.iter().fold(0, |total_priority, rucksack| {
        let shared = rucksack
            .first
            .intersection(&rucksack.second)
            .next()
            .unwrap();
        total_priority + priority(*shared)
    })
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
        *a_b_shared.intersection(&self.c).next().unwrap()
    }
}

fn process_p2(input: String) -> Vec<Group> {
    let lines: Vec<&str> = input.lines().collect_vec();
    lines
        .chunks(3)
        .map(|group_lines| Group::new(group_lines[0], group_lines[1], group_lines[2]))
        .collect()
}

fn part_2(groups: &[Group]) -> u32 {
    groups.iter().fold(0, |total_priority, group| {
        total_priority + priority(group.common())
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

        let input = process(test_input);
        assert_eq!(part_1(&input), 157);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = process(util::read_input("inputs/day03.txt").unwrap());
        b.iter(|| part_1(&input));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = process_p2(util::read_input("inputs/day03.txt").unwrap());
        b.iter(|| part_2(&input));
    }
}

use std::collections::HashSet;

use crate::util;

pub fn run() {
    let raw_input = util::read_input("inputs/day04.txt").unwrap();
    let input = process(raw_input);
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

type Pair = ((u32, u32), (u32, u32));

fn process(input: String) -> Vec<Pair> {
    input
        .lines()
        .map(|pair_str| {
            let (first, second) = pair_str.split_once(',').unwrap();
            let (first_start, first_end) = first.split_once('-').unwrap();
            let (second_start, second_end) = second.split_once('-').unwrap();
            (
                (first_start.parse().unwrap(), first_end.parse().unwrap()),
                (second_start.parse().unwrap(), second_end.parse().unwrap()),
            )
        })
        .collect()
}

fn part_1(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .filter(|(first, second)| {
            (first.0 >= second.0 && first.1 <= second.1)
                || (second.0 >= first.0 && second.1 <= first.1)
        })
        .count()
}

fn part_2(pairs: &[Pair]) -> usize {
    pairs
        .iter()
        .filter(|(first, second)| {
            let first_range: HashSet<u32> = HashSet::from_iter(first.0..=first.1);
            let second_range: HashSet<u32> = HashSet::from_iter(second.0..=second.1);

            first_range.intersection(&second_range).count() != 0
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = process(util::read_input("inputs/day04.txt").unwrap());
        b.iter(|| part_1(&input));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = process(util::read_input("inputs/day04.txt").unwrap());
        b.iter(|| part_2(&input));
    }
}

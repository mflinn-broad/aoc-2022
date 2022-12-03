use crate::util;

pub fn run() {
    let raw_input = util::read_input("inputs/day01.txt").unwrap();
    let input = process(raw_input);
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

fn process(input: String) -> Vec<u64> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.lines()
                .map(|calories| calories.parse::<u64>().unwrap())
                .sum()
        })
        .collect()
}

fn part_1(input: &[u64]) -> u64 {
    input.iter().max().unwrap().to_owned()
}

fn part_2(input: &[u64]) -> u64 {
    let mut sorted_elves = input.to_owned();
    sorted_elves.sort();

    sorted_elves.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = process(util::read_input("inputs/day01.txt").unwrap());
        b.iter(|| part_1(&input));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = process(util::read_input("inputs/day01.txt").unwrap());
        b.iter(|| part_2(&input));
    }
}

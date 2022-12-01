use crate::util;

pub fn run() {
    let raw_input = util::read_input("inputs/day01.txt").unwrap();
    let input = process(raw_input);
    println!("part 1: {}", part_1(input.clone()));
    println!("part 2: {}", part_2(input));
}

fn process(input: String) -> Vec<Vec<u64>> {
    input.split("\n\n")
        .map(|elf| elf.lines()
            .map(|calories| calories.parse().unwrap())
            .collect()
        )
        .collect()
}

fn part_1(input: Vec<Vec<u64>>) -> u64 {
    input.iter()
        .map(|elf| elf.iter().sum())
        .max().unwrap()
}

fn part_2 (input: Vec<Vec<u64>>) -> u64 {
    let mut elves: Vec<u64> = input.iter()
        .map(|elf| elf.iter().sum())
        .collect();
    
    elves.sort_by(|a, b| b.cmp(a));
    elves.iter()
        .take(3)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;

    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = process(util::read_input("inputs/day01.txt").unwrap());
        b.iter(|| part_1(input.clone()));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = process(util::read_input("inputs/day01.txt").unwrap());
        b.iter(|| part_2(input.clone()));
    }
}

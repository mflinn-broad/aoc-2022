use crate::util;
use std::collections::HashSet;

pub fn run() {
    let raw_input = util::read_input("inputs/day06.txt").unwrap();
    println!("part 1: {}", part_1(&raw_input, 4));
    println!("part 2: {}", part_1(&raw_input, 14));
}

fn part_1(input: &str, window_size: usize) -> usize {
    let input_chars: Vec<char> = input.chars()
        .collect();

    for (i, window) in input_chars.windows(window_size).enumerate(){
        let window_char_set: HashSet<char> = HashSet::from_iter(window.iter().copied());
        if window_char_set.len() == window_size {
            return i + window_size;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = util::read_input("inputs/day06.txt").unwrap();
        b.iter(|| part_1(&input, 4));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = util::read_input("inputs/day06.txt").unwrap();
        b.iter(|| part_1(&input, 14));
    } 
}

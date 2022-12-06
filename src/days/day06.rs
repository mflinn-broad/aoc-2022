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

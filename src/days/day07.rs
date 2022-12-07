use crate::util;
use std::collections::HashMap;

const DEV_SPACE: u64 = 70_000_000;
const MIN_REQUIRED_SPACE: u64 = 30_000_000;

pub fn run() {
    let raw_input = util::read_input("inputs/day07.txt").unwrap();
    println!("part 1: {}", part_1(&raw_input));
    println!("part 2: {}", part_2(&raw_input));
}

fn part_1(input: &str) -> u64 {
    let dir_size_map = construct_dir_size_map(input);
    dir_size_map
        .iter()
        .filter(|(_, &dir_size)| dir_size <= 100_000)
        .fold(0, |total_size, (_, &dir_size)| total_size + dir_size)
}

fn part_2(input: &str) -> u64 {
    let dir_size_map = construct_dir_size_map(input);
    dir_size_map
        .iter()
        .filter(|(_, &dir_size)| dir_size >= (MIN_REQUIRED_SPACE - (DEV_SPACE - dir_size_map["."])))
        .map(|(_, &size)| size)
        .min()
        .unwrap()
}

fn construct_dir_size_map(input: &str) -> HashMap<String, u64> {
    let mut cwd = String::new();
    let mut dir_size_map: HashMap<String, u64> = HashMap::new();
    let input_without_ls_or_dir = input
        .lines()
        .filter(|&line| line != "$ ls" && &line[0..3] != "dir");
    for line in input_without_ls_or_dir {
        match line {
            // use . as the root dir so that / can easily be used as a delimiter
            "$ cd /" => cwd.push('.'),
            "$ cd .." => cwd.truncate(cwd.rfind('/').unwrap()),
            _ if line.starts_with("$ cd") => {
                cwd.push('/');
                cwd.push_str(&line[5..]);
            }
            _ => {
                let file_size: u64 = line
                    .split_ascii_whitespace()
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
                update_size_of_cwd_and_parents(&mut dir_size_map, &cwd, file_size)
            }
        }
    }

    dir_size_map
}

fn update_size_of_cwd_and_parents(
    dir_size_map: &mut HashMap<String, u64>,
    path: &str,
    file_size: u64,
) {
    let dir_size = dir_size_map.entry(path.to_string()).or_insert(0);
    *dir_size += file_size;
    path.match_indices('/').for_each(|(i, _)| {
        let dir_size = dir_size_map.entry(path[0..i].to_string()).or_insert(0);
        *dir_size += file_size;
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = util::read_input("inputs/day07.txt").unwrap();
        b.iter(|| part_1(&input));
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = util::read_input("inputs/day07.txt").unwrap();
        b.iter(|| part_2(&input));
    }
}

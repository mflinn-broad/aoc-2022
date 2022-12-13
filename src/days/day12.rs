use crate::util;
use std::collections::HashSet;
use std::collections::VecDeque;

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

pub fn run() {
    let raw_input = util::read_input("inputs/day12.txt").unwrap();
    let grid = parse(raw_input);
    println!(
        "part 1: {}",
        grid.find_shortest_path_length_by(|pos| pos == grid.start)
    );
    println!(
        "part 2: {}",
        grid.find_shortest_path_length_by(|pos| grid.map[pos.0][pos.1] == 0)
    );
}

fn parse(input: String) -> Grid {
    let mut grid = Grid::new();
    grid.map = input
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.char_indices()
                .map(|(col, c)| match c {
                    'S' => {
                        grid.start = (row, col);
                        0
                    }
                    'E' => {
                        grid.end = (row, col);
                        25
                    }
                    h => ((h as u32 - 71) % 26) as i64,
                })
                .collect()
        })
        .collect();
    grid
}

#[derive(Debug, Clone)]
struct Grid {
    map: Vec<Vec<i64>>,
    start: (usize, usize),
    end: (usize, usize),
}

impl Grid {
    fn new() -> Self {
        Grid {
            map: Vec::new(),
            start: (0, 0),
            end: (0, 0),
        }
    }

    fn find_shortest_path_length_by<F: Fn((usize, usize)) -> bool>(&self, pred: F) -> usize {
        let mut visited: HashSet<(usize, usize)> = HashSet::new();
        let mut possible_paths = VecDeque::new();
        possible_paths.push_back(vec![self.end]);
        while !possible_paths.is_empty() {
            let current_path = possible_paths.pop_front().unwrap();
            let current_pos = *current_path.last().unwrap();
            if pred(current_pos) {
                return current_path.len() - 1;
            }
            let possible_moves = self.get_possible_moves(current_pos);
            for mov in possible_moves {
                if !current_path.contains(&mov) && !visited.contains(&mov) {
                    let mut new_path = current_path.clone();
                    new_path.push(mov);
                    possible_paths.push_back(new_path);
                    visited.insert(mov);
                }
            }
        }
        0
    }

    fn get_possible_moves(&self, current_pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut possible_moves = Vec::new();
        for direction in DIRECTIONS {
            let dest_row = (current_pos.0 as isize) + direction.0;
            let dest_col = (current_pos.1 as isize) + direction.1;
            if (dest_row < 0 || dest_row >= (self.map.len()) as isize)
                || (dest_col < 0 || dest_col >= (self.map[0].len() as isize))
            {
                continue;
            }
            if self.map[dest_row as usize][dest_col as usize]
                >= self.map[current_pos.0][current_pos.1] - 1
            {
                possible_moves.push((dest_row as usize, dest_col as usize));
            }
        }
        possible_moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = util::read_input("inputs/day12.txt").unwrap();
        b.iter(|| {
            let grid = parse(input.clone());
            grid.find_shortest_path_length_by(|pos| pos == grid.start);
        })
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = util::read_input("inputs/day12.txt").unwrap();
        b.iter(|| {
            let grid = parse(input.clone());
            grid.find_shortest_path_length_by(|pos| grid.map[pos.0][pos.1] == 0);
        })
    }

    #[test]
    fn test_part_1() {
        let input = String::from(
            "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi",
        );
        let grid = parse(input);

        assert_eq!(
            31,
            grid.find_shortest_path_length_by(|pos| pos == grid.start)
        );
    }
}

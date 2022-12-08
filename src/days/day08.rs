use crate::util;
use itertools::iproduct;

pub fn run() {
    let raw_input = util::read_input("inputs/day08.txt").unwrap();
    let trees = process(raw_input);
    println!("part 1: {}", part_1(trees.clone()));
    println!("part 2: {}", part_2(trees));
}

fn process(input: String) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|height| height.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn part_1(trees: Vec<Vec<i32>>) -> usize {
    let (height, width) = (trees.len(), trees[0].len());
    let mut visible_trees = vec![vec![false; width]; height];

    for (row, col) in iproduct!(0..height, 0..width) {
        let mut prior_tallest = -1;
        for curr_col in 0..=col {
            if trees[row][curr_col] > prior_tallest {
                visible_trees[row][curr_col] = true;
                prior_tallest = trees[row][curr_col];
            }
        }

        let mut prior_tallest = -1;
        for curr_col in (col..width).rev() {
            if trees[row][curr_col] > prior_tallest {
                visible_trees[row][curr_col] = true;
                prior_tallest = trees[row][curr_col];
            }
        }

        let mut prior_tallest = -1;
        for curr_row in 0..=row {
            if trees[curr_row][col] > prior_tallest {
                visible_trees[curr_row][col] = true;
                prior_tallest = trees[curr_row][col];
            }
        }

        let mut prior_tallest = -1;
        for curr_row in (row..height).rev() {
            if trees[curr_row][col] > prior_tallest {
                visible_trees[curr_row][col] = true;
                prior_tallest = trees[curr_row][col];
            }
        }
    }
    visible_trees
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&is_visible| *is_visible)
        .count()
}

fn part_2(trees: Vec<Vec<i32>>) -> i32 {
    let (height, width) = (trees.len(), trees[0].len());
    let mut scenic_scores = vec![vec![1; width]; height];
    for (row, col) in iproduct!(0..height, 0..width) {
        let curr_height = trees[row][col];
        let mut visible_count = 0;
        for curr_col in (0..col).rev() {
            visible_count += 1;
            if trees[row][curr_col] >= curr_height {
                break;
            }
        }
        scenic_scores[row][col] *= visible_count;

        let mut visible_count = 0;
        for curr_col in (col..width).skip(1) {
            visible_count += 1;
            if trees[row][curr_col] >= curr_height {
                break;
            }
        }
        scenic_scores[row][col] *= visible_count;

        let mut visible_count = 0;
        for curr_row in (0..row).rev() {
            visible_count += 1;
            if trees[curr_row][col] >= curr_height {
                break;
            }
        }
        scenic_scores[row][col] *= visible_count;

        let mut visible_count = 0;
        for curr_row in (row..height).skip(1) {
            visible_count += 1;
            if trees[curr_row][col] >= curr_height {
                break;
            }
        }
        scenic_scores[row][col] *= visible_count;
    }

    scenic_scores
        .iter()
        .flat_map(|row| row.iter())
        .max()
        .cloned()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = util::read_input("inputs/day08.txt").unwrap();
        b.iter(|| {
            let trees = process(input.clone());
            part_1(trees);
        })
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = util::read_input("inputs/day08.txt").unwrap();
        b.iter(|| {
            let trees = process(input.clone());
            part_2(trees);
        })
    }
}

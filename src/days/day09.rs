use crate::util;
use std::cell::Cell;
use std::collections::HashSet;

const UP: Pos = Pos { x: 1, y: 0 };
const RIGHT: Pos = Pos { x: 0, y: 1 };
const DOWN: Pos = Pos { x: -1, y: 0 };
const LEFT: Pos = Pos { x: 0, y: -1 };

pub fn run() {
    let raw_input = util::read_input("inputs/day09.txt").unwrap();
    let input = parse(raw_input);
    let rope = Rope::new(2);
    println!("part 1: {}", part_1(&input, rope));
    let rope = Rope::new(10);
    println!("part 2: {}", part_1(&input, rope));
}

fn parse(input: String) -> Vec<(Pos, usize)> {
    input
        .lines()
        .map(|line| {
            let (pos_str, amt_str) = line.split_once(' ').unwrap();
            (pos_str.into(), amt_str.parse().unwrap())
        })
        .collect()
}

fn part_1(moves: &[(Pos, usize)], rope: Rope) -> usize {
    let mut visited: HashSet<Pos> = HashSet::new();
    for (direction, amt) in moves {
        for _ in 0..*amt {
            rope.step(direction);
            visited.insert(rope.knots[rope.knots.len() - 1].get());
        }
    }
    visited.len()
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Pos {
    x: isize,
    y: isize,
}

impl std::ops::Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Pos {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl From<&str> for Pos {
    fn from(value: &str) -> Self {
        match value {
            "R" => RIGHT,
            "U" => UP,
            "D" => DOWN,
            "L" => LEFT,
            _ => panic!("invalid"),
        }
    }
}

#[derive(Debug, Clone)]
struct Rope {
    knots: Vec<Cell<Pos>>,
}

impl Rope {
    fn new(num_knots: usize) -> Self {
        let knots = vec![Cell::new(Pos { x: 0, y: 0 }); num_knots];
        Self { knots }
    }

    fn step(&self, direction: &Pos) {
        self.knots[0].set(self.knots[0].get() + *direction);
        for i in 1..self.knots.len() {
            let current_knot = self.knots[i].get();
            let prior_knot = self.knots[i - 1].get();
            let x_diff = prior_knot.x - current_knot.x;
            let y_diff = prior_knot.y - current_knot.y;
            if x_diff.abs() >= 2 || y_diff.abs() >= 2 {
                self.knots[i].set(move_towards_prior(
                    (prior_knot.x, prior_knot.y),
                    (current_knot.x, current_knot.y),
                ));
            }
        }
    }
}

fn move_towards_prior(prior: (isize, isize), current: (isize, isize)) -> Pos {
    let x_diff = prior.0 - current.0;
    let y_diff = prior.1 - current.1;
    match (x_diff, y_diff) {
        (x, 0) if x > 0 => Pos {
            x: current.0 + 1,
            y: current.1,
        },
        (x, 0) if x < 0 => Pos {
            x: current.0 - 1,
            y: current.1,
        },
        (0, y) if y > 0 => Pos {
            x: current.0,
            y: current.1 + 1,
        },
        (0, y) if y < 0 => Pos {
            x: current.0,
            y: current.1 - 1,
        },
        (x, y) if x > 0 && y > 0 => Pos {
            x: current.0 + 1,
            y: current.1 + 1,
        },
        (x, y) if x < 0 && y < 0 => Pos {
            x: current.0 - 1,
            y: current.1 - 1,
        },
        (x, y) if x > 0 && y < 0 => Pos {
            x: current.0 + 1,
            y: current.1 - 1,
        },
        (x, y) if x < 0 && y > 0 => Pos {
            x: current.0 - 1,
            y: current.1 + 1,
        },
        _ => panic!("unreachable"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let input = util::read_input("inputs/day09.txt").unwrap();
        b.iter(|| {
            let moves = parse(input.clone());
            let rope = Rope::new(2);
            part_1(&moves, rope)
        })
    }

    #[bench]
    fn bench_part_2(b: &mut Bencher) {
        let input = util::read_input("inputs/day09.txt").unwrap();
        b.iter(|| {
            let moves = parse(input.clone());
            let rope = Rope::new(10);
            part_1(&moves, rope)
        })
    }
}

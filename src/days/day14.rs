use itertools::Itertools;

use crate::util;
use std::{collections::HashSet, hash::Hash};

const STARTING_POINT: Point = Point { x: 500, y: 0 };

pub fn run() {
    let raw_input = util::read_input("inputs/day14.txt").unwrap();
    let mut cave = parse(raw_input);
    while cave.tick().is_some() {}
    println!("{}", cave.fallen_sand.len());
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl From<&str> for Point {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once(',').unwrap();
        Point {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

#[derive(Debug)]
struct RockStructure {
    obstacles: HashSet<Point>,
}

impl From<&str> for RockStructure {
    fn from(value: &str) -> Self {
        let points: Vec<Point> = value
            .split("->")
            .map(|point| point.trim())
            .map(Point::from)
            .collect();
        let mut obstacles = HashSet::new();
        for (curr, next) in points.iter().tuple_windows() {
            if curr.x == next.x {
                if curr.y < next.y {
                    for ys in curr.y..=next.y {
                        obstacles.insert(Point { x: curr.x, y: ys });
                    }
                } else {
                    for ys in next.y..=curr.y {
                        obstacles.insert(Point { x: curr.x, y: ys });
                    }
                }
                
            } else {
                if curr.x < next.x {
                    for xs in curr.x..=next.x {
                        obstacles.insert(Point { x: xs, y: curr.y });
                    }
                } else {
                    for xs in next.x..=curr.x {
                        obstacles.insert(Point { x: xs, y: curr.y });
                    }
                }
                
            }
        }
        RockStructure { obstacles }
    }
}

#[derive(Debug)]
struct Cave {
    obstacles: HashSet<Point>,
    fallen_sand: HashSet<Point>,
    curr_sand: Option<Point>,
    lowest_point: usize,
}

impl From<&str> for Cave {
    fn from(value: &str) -> Self {
        let mut obstacles = HashSet::new();
        value
            .lines()
            .map(|line| line.trim())
            .map(RockStructure::from)
            .for_each(|structure| {
                obstacles.extend(structure.obstacles);
            });

        let lowest_point = obstacles.iter().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
        // add floor
        Cave {
            obstacles,
            fallen_sand: HashSet::new(),
            curr_sand: None,
            lowest_point,
        }
    }
}

impl Cave {
    fn tick(&mut self) -> Option<()> {
        if self.fallen_sand.contains(&STARTING_POINT) {
            return None;
        }

        // if curr_sand is none spawn a new particle
        if self.curr_sand.is_none() {
            self.spawn_sand();
        }

        // check for floor
        if self.curr_sand.as_ref().unwrap().y == self.lowest_point+1 {
            self.fallen_sand.insert(self.curr_sand.take().unwrap());
            return Some(());
        }

        // try down
        let curr_sand = self.curr_sand.as_ref().unwrap();
        if !self.obstacles.contains(&Point {
            x: curr_sand.x,
            y: curr_sand.y + 1,
        }) && !self.fallen_sand.contains(&Point {
            x: curr_sand.x,
            y: curr_sand.y + 1,
        }) {
            self.curr_sand = Some(Point {
                x: curr_sand.x,
                y: curr_sand.y + 1,
            });
            return Some(());
        }

        // try down left
        if !self.obstacles.contains(&Point {
            x: curr_sand.x - 1,
            y: curr_sand.y + 1,
        }) && !self.fallen_sand.contains(&Point {
            x: curr_sand.x - 1,
            y: curr_sand.y + 1,
        }) {
            self.curr_sand = Some(Point {
                x: curr_sand.x - 1,
                y: curr_sand.y + 1,
            });
            return Some(());
        }

        // try down right
        if !self.obstacles.contains(&Point {
            x: curr_sand.x + 1,
            y: curr_sand.y + 1,
        }) && !self.fallen_sand.contains(&Point {
            x: curr_sand.x + 1,
            y: curr_sand.y + 1,
        }) {
            self.curr_sand = Some(Point {
                x: curr_sand.x + 1,
                y: curr_sand.y + 1,
            });
            return Some(());
        }

        // reaching this point means fallen sand has stopped
        self.fallen_sand.insert(self.curr_sand.take().unwrap());
        Some(())
    }

    fn spawn_sand(&mut self) {
        self.curr_sand = Some(STARTING_POINT);
    }
}

fn parse(input: String) -> Cave {
    Cave::from(input.as_str())
}


mod tests {
    use super::*;
    extern crate test;
    use test::Bencher;

    const TEST_OBSTACLES: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_cave() {
        let input = String::from(TEST_OBSTACLES);
        let mut cave = parse(input);
        while cave.tick().is_some() {}
        assert_eq!(cave.fallen_sand.len(), 24);
    }

    #[bench]
    fn bench_part_1(b: &mut Bencher) {
        let raw_input = util::read_input("inputs/day14.txt").unwrap();
        b.iter(|| {
            let mut cave = parse(raw_input.clone());
            while cave.tick().is_some() {}
        })
        
    }
}

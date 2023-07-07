use itertools::Itertools;

use crate::util;
use std::collections::HashSet;

pub fn run() {
    let raw_input = util::read_input("inputs/day15.txt").unwrap();
    let sensors_and_beacons = parse(raw_input);
    let manhattans = calculate_manhattans(sensors_and_beacons);
    let row_with_distress = find_row_with_distress(&manhattans, 4000000).unwrap();
    let tuning_freq = find_tuning_freq(row_with_distress);
    println!("part 2: {}", tuning_freq);
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn manhattan_distance(&self, other: &Position) -> usize {
        let x_diff = (self.x - other.x).abs();
        let y_diff = (self.y - other.y).abs();
        (x_diff + y_diff) as usize
    }
}

impl From<&str> for Position {
    fn from(value: &str) -> Self {
        let value = value.trim();
        let (x, y) = value.split_once(',').unwrap();
        let (_, x) = x.split_once('=').unwrap();
        let (_, y) = y.split_once('=').unwrap();
        Position {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

type SensorAndBeacon = (Position, Position);

fn parse(input: String) -> Vec<SensorAndBeacon> {
    input
        .lines()
        .map(|line| {
            let (sensor_str, beacon_str) = line.split_once(':').unwrap();
            let sensor_str = sensor_str.strip_prefix("Sensor at").unwrap();
            let beacon_str = beacon_str
                .trim()
                .strip_prefix("closest beacon is at")
                .unwrap();
            (Position::from(sensor_str), Position::from(beacon_str))
        })
        .collect()
}

fn calculate_manhattans(
    sensors_and_beacons: Vec<SensorAndBeacon>,
) -> Vec<(SensorAndBeacon, usize)> {
    sensors_and_beacons
        .into_iter()
        .map(|sensor_and_beacon| {
            (
                sensor_and_beacon,
                sensor_and_beacon.0.manhattan_distance(&sensor_and_beacon.1),
            )
        })
        .collect()
}

fn get_row_segment_at_manhattan(
    sensor_and_beacon: &SensorAndBeacon,
    dis: usize,
    row: i64,
) -> Option<(i64, i64)> {
    let (sensor, beacon) = sensor_and_beacon;
    // check if shortest path to row is more than distance away
    let y_diff: usize = sensor.manhattan_distance(&Position {
        x: sensor.x,
        y: row,
    });
    if y_diff > dis {
        return None;
    }
    let offset = dis - y_diff;
    let min_x = sensor.x - offset as i64;
    let max_x = sensor.x + offset as i64;
    Some((min_x, max_x))
}

fn get_row_segments(sensors_and_beacons: &[(SensorAndBeacon, usize)], row: i64) -> Vec<(i64, i64)> {
    let mut segments = Vec::new();

    sensors_and_beacons
        .iter()
        .for_each(|(sensor_and_beacon, distance)| {
            let segment = get_row_segment_at_manhattan(sensor_and_beacon, *distance, row);
            if segment.is_some() {
                segments.push(segment.unwrap());
            }
        });

    merge_segments(segments)
}

fn merge_segments(mut input: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let mut merged = Vec::new();
    if input.is_empty() {
        return merged;
    }
    input.sort_unstable();
    let mut curr_segment = input.first().unwrap().to_owned();
    for &next_segment in &input[1..] {
        if curr_segment.1 + 1 < next_segment.0 {
            merged.push(curr_segment);
            curr_segment = next_segment;
            continue;
        }

        if curr_segment.1 >= next_segment.1 {
            continue;
        }

        else if curr_segment.1 < next_segment.1 {
            curr_segment = (curr_segment.0, next_segment.1);
        }
    }
    merged.push(curr_segment);
    merged
}

fn find_row_with_distress(
    sensors_and_beacons: &[(SensorAndBeacon, usize)],
    row_max: usize,
) -> Option<(Vec<(i64, i64)>, usize)> {
    for row in 0..=row_max {
        let segments = get_row_segments(sensors_and_beacons, row as i64);
        if segments.len() > 1 {
            return Some((segments, row))
        }
    }
    None
}

fn find_tuning_freq(input: (Vec<(i64,i64)>, usize)) -> i64 {
    let (x_ranges, row) = input; 
    (x_ranges[0].1 + 1) * 4000000 + row as i64
}

mod tests {
    use super::*;

    const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    // #[test]
    // fn test_part_1() {
    //     let sensors_and_beacons = parse(String::from(TEST_INPUT));
    //     let manhattans = calculate_manhattans(sensors_and_beacons);
    //     let segments = get_row_segments(&manhattans, 10);
    //     assert_eq!(segments.len(), 26);
    // }

    #[test]
    fn test_part_2() {
        let sensors_and_beacons = parse(String::from(TEST_INPUT));
        let manhattans = calculate_manhattans(sensors_and_beacons);
        let row_with_distress = find_row_with_distress(&manhattans, 20).unwrap();
        let tuning_freq = find_tuning_freq(row_with_distress);
        assert_eq!(tuning_freq, 56000011);
    }
}

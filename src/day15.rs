use std::ops::Range;

use aoc_runner_derive::{aoc, aoc_generator};
use btree_range_map::RangeSet;

use crate::parse::ints;

struct Sensor {
    sensor: (i64, i64),
    beacon: (i64, i64),
    radius: i64,
}

impl Sensor {
    fn intersect_row(&self, y: i64) -> (i64, i64) {
        let diff = self.radius - (self.sensor.1 - y).abs();
        if diff < 0 {
            (0, 0)
        } else {
            (self.sensor.0 - diff, self.sensor.0 + diff + 1)
        }
    }

    fn intersect_point(&self, point: (i64, i64)) -> bool {
        self.radius >= Self::dist(self.sensor, point)
    }

    fn dist(p1: (i64, i64), p2: (i64, i64)) -> i64 {
        (p2.0 - p1.0).abs() + (p2.1 - p1.1).abs()
    }
}

impl From<Vec<i64>> for Sensor {
    fn from(data: Vec<i64>) -> Self {
        assert_eq!(data.len(), 4);

        let sensor = (data[0], data[1]);
        let beacon = (data[2], data[3]);
        let radius = Self::dist(sensor, beacon);

        Self {
            sensor,
            beacon,
            radius
        }
    }
}

#[aoc_generator(day15)]
fn parse(data: &str) -> Vec<Sensor> {
    data.lines().map(ints).map(Sensor::from).collect()
}

const ROW: i64 = 2000000;

#[aoc(day15, part1)]
fn part1(data: &Vec<Sensor>) -> u64 {
    let mut rset = RangeSet::new();
    let mut beacons = Vec::new();

    for sensor in data {
        let (rx, ry) = sensor.intersect_row(ROW);
        rset.insert(rx..ry);

        if sensor.beacon.1 == ROW {
            beacons.push(sensor.beacon.0);
        }
    }

    for bx in beacons {
        rset.remove(bx)
    }

    rset.len()
}

const SIZE: i64 = 4000000;

fn split(range: Range<i64>) -> (Range<i64>, Range<i64>) {
    let mid = range.start + (range.end - range.start) / 2;
    (range.start..mid, mid..range.end)
}

fn quadrants(x_range: Range<i64>, y_range: Range<i64>) -> [(Range<i64>, Range<i64>); 4] {
    let (x1, x2) = split(x_range);
    let (y1, y2) = split(y_range);
    [
        (x1.clone(), y1.clone()),
        (x2.clone(), y1),
        (x1, y2.clone()),
        (x2, y2),
    ]
}

fn corners(x_range: &Range<i64>, y_range: &Range<i64>) -> [(i64, i64); 4] {
    [
        (x_range.start, y_range.start),
        (x_range.start, y_range.end - 1),
        (x_range.end - 1, y_range.start),
        (x_range.end - 1, y_range.end - 1),
    ]
}

fn find_beacon(data: &Vec<Sensor>, x_range: Range<i64>, y_range: Range<i64>) -> Option<(i64, i64)> {
    if x_range.is_empty() || y_range.is_empty() {
        return None
    }

    if x_range.end - x_range.start == 1 && y_range.end - y_range.start == 1 {
        let point = (x_range.start, y_range.start);
        if data.iter().any(|sensor| sensor.intersect_point(point)) {
            return None
        } else {
            return Some(point)
        }
    }

    if data.iter().any(|sensor| {
        corners(&x_range, &y_range).into_iter().all(|corner| sensor.intersect_point(corner))
    }) {
        return None
    }

    quadrants(x_range, y_range).into_iter().find_map(|(x_range, y_range)| {
        find_beacon(data, x_range, y_range)
    })
}

#[aoc(day15, part2)]
fn part2_b(data: &Vec<Sensor>) -> i64 {
    let (x, y) = find_beacon(data, 0..SIZE, 0..SIZE).unwrap();
    x * SIZE + y
}
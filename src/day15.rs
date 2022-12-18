use aoc_runner_derive::{aoc, aoc_generator};
use btree_range_map::RangeSet;

use crate::parse::ints;

#[aoc_generator(day15)]
fn parse(data: &str) -> Vec<Vec<i32>> {
    data.lines().map(ints).collect()
}

fn intersect(origin: (i32, i32), radius: i32, y: i32) -> (i32, i32) {
    let diff = radius - (origin.1 - y).abs();
    if diff < 0 {
        (0, 0)
    } else {
        (origin.0 - diff, origin.0 + diff + 1)
    }
}

fn dist(p1: (i32, i32), p2: (i32, i32)) -> i32 {
    return (p2.0 - p1.0).abs() + (p2.1 - p1.1).abs()
}

const ROW: i32 = 2000000;

#[aoc(day15, part1)]
fn part1(data: &Vec<Vec<i32>>) -> u32 {
    let mut rset = RangeSet::new();
    let mut beacons = Vec::new();

    for line in data {
        let (sx, sy, bx, by) = (line[0], line[1], line[2], line[3]);
        let origin = (sx, sy);
        let radius = dist(origin, (bx, by));
        let (rx, ry) = intersect(origin, radius, ROW);
        rset.insert(rx..ry);

        if by == ROW {
            beacons.push(bx);
        }
    }

    for bx in beacons {
        rset.remove(bx)
    }

    rset.len()
}
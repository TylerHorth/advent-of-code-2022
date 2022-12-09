use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{FromStr, Display, ParseError};

#[derive(FromStr, Display)]
#[display("{direction} {steps}")]
struct Move {
    direction: Direction,
    steps: i32
}

#[derive(FromStr, Display)]
enum Direction {
    U, D, L, R
}

#[aoc_generator(day9)]
fn parse(data: &str) -> Result<Vec<Move>, ParseError> {
    data.lines().map(str::parse).collect()
}

#[aoc(day9, part1)]
fn part1(data: &Vec<Move>) -> usize {
    simulate_rope::<2>(data)
}

#[aoc(day9, part2)]
fn part2(data: &Vec<Move>) -> usize {
    simulate_rope::<10>(data)
}

fn simulate_rope<const N: usize>(data: &Vec<Move>) -> usize {
    let mut visited = HashSet::new();
    let mut rope = [(0i32, 0i32); N];

    for m in data {
        for _ in 0..m.steps {
            match m.direction {
                Direction::U => rope[0].1 -= 1,
                Direction::D => rope[0].1 += 1,
                Direction::L => rope[0].0 -= 1,
                Direction::R => rope[0].0 += 1,
            }

            for i in 0..N - 1 {
                let (head, tail) = (rope[i], &mut rope[i + 1]);
                let (dx, dy) = (head.0 - tail.0, head.1 - tail.1);

                if dx.abs() > 1 || dy.abs() > 1 {
                    tail.0 += dx.signum();
                    tail.1 += dy.signum();
                }
            }

            visited.insert(rope[N - 1]);
        }
    }

    visited.len()
}
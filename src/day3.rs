use std::ops::BitAnd;

use aoc_runner_derive::aoc;

fn priority(c: u8) -> u8 {
    match c {
        b'a'..=b'z' => c - b'a' + 1,
        b'A'..=b'Z' => c - b'A' + 27,
        _ => panic!("outside range: {}", c)
    }
}

fn bitmap(data: impl AsRef<[u8]>) -> u64 {
    data.as_ref().iter().cloned().fold(0, |map, c| {
        map | 1 << priority(c)
    })
}

#[aoc(day3, part1)]
fn part1(data: &str) -> u32 {
    data.lines().map(|line| {
        let (left, right) = line.split_at(line.len() / 2);
        (bitmap(left) & bitmap(right)).trailing_zeros()
    }).sum()
}

#[aoc(day3, part2)]
fn part2(data: &str) -> u32 {
    let lines: Vec<_> = data.lines().collect();
    lines.chunks_exact(3).map(|group| {
        group.iter().map(bitmap).reduce(BitAnd::bitand).unwrap().trailing_zeros()
    }).sum()
}
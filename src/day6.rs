use std::collections::BTreeSet;

use aoc_runner_derive::aoc;

fn message_offset<const N: usize>(data: &[u8]) -> usize {
    data.array_windows()
        .take_while(|arr: &&[u8; N]| {
            arr.iter().copied().collect::<BTreeSet<u8>>().len() != N
        })
        .count() + N
}

#[aoc(day6, part1)]
fn part1(data: &[u8]) -> usize {
    message_offset::<4>(data)
}

#[aoc(day6, part2)]
fn part2(data: &[u8]) -> usize {
    message_offset::<14>(data)
}
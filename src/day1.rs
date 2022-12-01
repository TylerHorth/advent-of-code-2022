use std::{num::ParseIntError, collections::BinaryHeap, cmp::Reverse};

use aoc_runner_derive::{aoc_generator, aoc};

#[aoc_generator(day1)]
fn parse(data: &str) -> Result<Vec<Vec<u32>>, ParseIntError> {
    data.split("\n\n")
        .map(|group| group.lines().map(str::parse).collect())
        .collect()
}

#[aoc(day1, part1)]
fn part1(data: &Vec<Vec<u32>>) -> u32 {
    top_n_sum::<1>(data)
}

#[aoc(day1, part2)]
fn part2(data: &Vec<Vec<u32>>) -> u32 {
    top_n_sum::<3>(data)
}

fn top_n_sum<const N: usize>(data: &Vec<Vec<u32>>) -> u32 {
    data.iter().map(|group| group.iter().sum())
        .fold(BinaryHeap::new(), |mut heap, n: u32| {
            heap.push(Reverse(n));

            if heap.len() > N {
                heap.pop();
            }

            heap
        })
        .iter().map(|r| r.0).sum()
}
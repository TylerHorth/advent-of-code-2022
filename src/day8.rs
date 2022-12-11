use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day8)]
fn parse(data: &str) -> Vec<Vec<u8>> {
    data.lines().map(|l| l.bytes().map(|c| c - b'0').collect()).collect()
}

fn in_range(data: &Vec<Vec<u8>>, x: i32, y: i32) -> bool {
    x >= 0 && y >= 0 && x < data[0].len() as i32 && y < data.len() as i32
}

fn visible_from_edge(data: &Vec<Vec<u8>>, visible: &mut HashSet<(i32, i32)>, (mut x, mut y): (i32, i32), (dx, dy): (i32, i32)) {
    visible.insert((x, y));
    let mut max = 0;
    while in_range(data, x, y) {
        let height = data[y as usize][x as usize];
        if height > max {
            max = height;
            visible.insert((x, y));
        }
        x += dx;
        y += dy;
    }
}

fn visible_from_tree(data: &Vec<Vec<u8>>, scores: &mut Vec<Vec<u64>>, (mut x, mut y): (i32, i32), (dx, dy): (i32, i32)) {
    let mut stack = vec![(10, 0)]; // (height, index)
    let mut i = 0;

    while in_range(data, x, y) {
        let height = data[y as usize][x as usize];

        while stack.last().unwrap().0 < height {
            stack.pop();
        }

        scores[y as usize][x as usize] *= i - stack.last().unwrap().1;
        stack.push((height, i));

        i += 1;
        x += dx;
        y += dy;
    }
}

fn each_direction<T>(data: &Vec<Vec<u8>>, result: &mut T, mut func: impl FnMut(&Vec<Vec<u8>>, &mut T, (i32, i32), (i32, i32))) {
    let width = data[0].len();
    let height = data.len();

    for x in 0..width {
        func(data, result, (x as i32, 0), (0, 1));
        func(data, result, (x as i32, height as i32 - 1), (0, -1));
    }

    for y in 0..height {
        func(data, result, (0, y as i32), (1, 0));
        func(data, result, (width as i32 - 1, y as i32), (-1, 0));
    }
}

#[aoc(day8, part1)]
fn part1(data: &Vec<Vec<u8>>) -> usize {
    let mut visible = HashSet::new();

    each_direction(data, &mut visible, visible_from_edge);

    visible.len()
}

#[aoc(day8, part2)]
fn part2(data: &Vec<Vec<u8>>) -> u64 {
    let mut scores = vec![vec![1; data[0].len()]; data.len()];

    each_direction(data, &mut scores, visible_from_tree);

    scores.iter().flatten().copied().max().unwrap()
}
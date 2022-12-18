use std::{collections::HashSet, iter};

use aoc_runner_derive::{aoc, aoc_generator};
use chumsky::prelude::*;

use crate::parse::parse_input;


fn paths(data: &str) -> Vec<Vec<(i32, i32)>> {
    let int = text::int(10).map(|s: String| s.parse().unwrap());
    let pair = int.then_ignore(just(',')).then(int);
    let path = pair.separated_by(just(" -> "));
    let paths = path.separated_by(text::newline());

    parse_input(paths, data, 14)
}

fn range((x1, y1): (i32, i32), (x2, y2): (i32, i32)) -> impl Iterator<Item=(i32, i32)> {
    let (dx, dy) = ((x2 - x1).signum(), (y2 - y1).signum());

    let mut next = Some((x1, y1));
    iter::from_fn(move || {
        if let Some((x, y)) = next {
            if (x, y) == (x2, y2) {
                next = None
            } else {
                next = Some((x + dx, y + dy))
            }

            Some((x, y))
       } else {
            None
       }
    })
}


#[aoc_generator(day14)]
fn parse(data: &str) -> (HashSet<(i32, i32)>, i32) {
    let paths = paths(data);
    let mut map = HashSet::new();
    let mut max_y = 0;

    for path in paths {
        for pair in path.windows(2) {
            for point in range(pair[0], pair[1]) {
                map.insert(point);
                max_y = max_y.max(point.1)
            }
        }
    }

    (map, max_y)
}

#[aoc(day14, part1)]
fn part1((map, max_y): &(HashSet<(i32, i32)>, i32)) -> usize {
    let mut map = map.clone();
    let mut count = 0;

    loop {
        let (mut x, mut y) = (500, 0);

        loop {
            if !map.contains(&(x, y + 1)) {
                y += 1;
            } else if !map.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else if !map.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                map.insert((x, y));
                count += 1;
                break
            }

            if y > *max_y {
                return count
            }
        }
    }
}

#[aoc(day14, part2)]
fn part2((map, max_y): &(HashSet<(i32, i32)>, i32)) -> usize {
    let mut map = map.clone();
    let mut count = 0;
    let floor = max_y + 2;

    loop {
        let (mut x, mut y) = (500, 0);

        loop {
            if y + 1 == floor {
                map.insert((x, y));
                count += 1;
                break
            } else if !map.contains(&(x, y + 1)) {
                y += 1;
            } else if !map.contains(&(x - 1, y + 1)) {
                x -= 1;
                y += 1;
            } else if !map.contains(&(x + 1, y + 1)) {
                x += 1;
                y += 1;
            } else {
                map.insert((x, y));
                count += 1;

                if x == 500 && y == 0 {
                    return count
                }

                break
            }
        }
    }
}
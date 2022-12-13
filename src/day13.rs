use aoc_runner_derive::{aoc, aoc_generator};
use chumsky::prelude::*;

use crate::parse::parse_input;

#[derive(Eq, Clone, Debug)]
enum IntList {
    Int(i32),
    List(Vec<IntList>)
}

impl Ord for IntList {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (IntList::Int(a), IntList::Int(b)) => a.cmp(b),
            (a @ IntList::Int(_), b @ IntList::List(_)) => IntList::List(vec![a.clone()]).cmp(b),
            (a @ IntList::List(_), b @ IntList::Int(_)) => a.cmp(&IntList::List(vec![b.clone()])),
            (IntList::List(a), IntList::List(b)) => {
                for (a, b) in a.iter().zip(b) {
                    let res = a.cmp(b);
                    if res.is_ne() {
                        return res;
                    }
                }
                return a.len().cmp(&b.len())
            },
        }
    }
}

impl PartialOrd for IntList {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for IntList {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

fn intlist_parser() -> impl Parser<char, IntList, Error=Simple<char>> {
    recursive(|bf| {
        choice((
            text::int(10).map(|s: String| IntList::Int(s.parse().unwrap())),
            just('[').ignore_then(bf.separated_by(just(','))).then_ignore(just(']')).map(IntList::List)
        ))
    })
}

#[aoc_generator(day13)]
fn parse(data: &str) -> Vec<(IntList, IntList)> {
    let parser = intlist_parser()
        .then_ignore(text::newline())
        .then(intlist_parser())
        .separated_by(text::newline().then(text::newline()))
        .then_ignore(end());
    
    parse_input(parser, data, 13)
}

#[aoc(day13, part1)]
fn part1(data: &Vec<(IntList, IntList)>) -> usize {
    data.iter()
        .enumerate()
        .filter(|(_, (a, b))| a < b)
        .map(|(i, _)| i + 1)
        .sum()
}

#[aoc(day13, part2)]
fn part2(data: &Vec<(IntList, IntList)>) -> usize {
    let mut first_count = 1;
    let mut second_count = 2;

    for intlist in data.iter().flat_map(|(a, b)| [a, b]) {
        if intlist < &IntList::Int(6) {
            second_count += 1;
            if intlist < &IntList::Int(2) {
                first_count += 1;
            }
        }
    }

    return first_count * second_count
}

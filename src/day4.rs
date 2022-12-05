use aoc_runner_derive::{aoc_generator, aoc};
use parse_display::{FromStr, Display, ParseError};


#[derive(Display, FromStr)]
#[display("{first},{second}")]
struct Pair {
    first: Range,
    second: Range
}

#[derive(Display, FromStr)]
#[display("{from}-{to}")]
struct Range {
    from: usize,
    to: usize
}

#[aoc_generator(day4)]
fn parse(data: &str) -> Result<Vec<Pair>, ParseError> {
    data.lines().map(str::parse).collect()
}

fn contains(first: &Range, second: &Range) -> bool {
    first.from <= second.from && first.to >= second.to
}

fn overlaps(first: &Range, second: &Range) -> bool {
    first.from <= second.to && first.to >= second.from
}

type Op = fn(&Range, &Range) -> bool;

fn either(pair: &Pair, op: Op) -> bool {
    op(&pair.first, &pair.second) || op(&pair.second, &pair.first)
}

fn count(data: &Vec<Pair>, op: Op) -> usize {
    data.iter().filter(|p| either(p, op)).count()
}

#[aoc(day4, part1)]
fn part1(data: &Vec<Pair>) -> usize {
    count(data, contains)
}

#[aoc(day4, part2)]
fn part2(data: &Vec<Pair>) -> usize {
    count(data, overlaps)
}
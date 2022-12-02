use std::cmp::Ordering;

use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{FromStr, Display, ParseError};

#[derive(Display, FromStr, Eq, PartialEq, Copy, Clone)]
enum Hand {
    #[from_str(regex = "A|X")]
    Rock,
    #[from_str(regex = "B|Y")]
    Paper,
    #[from_str(regex = "C|Z")]
    Scissors
}


#[derive(Display, FromStr, Copy, Clone)]
enum Outcome {
    #[display("X")]
    Loss,
    #[display("Y")]
    Draw,
    #[display("Z")]
    Win
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Hand::Rock, Hand::Rock) => Ordering::Equal,
            (Hand::Rock, Hand::Paper) => Ordering::Less,
            (Hand::Rock, Hand::Scissors) => Ordering::Greater,
            (Hand::Paper, Hand::Rock) => Ordering::Greater,
            (Hand::Paper, Hand::Paper) => Ordering::Equal,
            (Hand::Paper, Hand::Scissors) => Ordering::Less,
            (Hand::Scissors, Hand::Rock) => Ordering::Less,
            (Hand::Scissors, Hand::Paper) => Ordering::Greater,
            (Hand::Scissors, Hand::Scissors) => Ordering::Equal,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        return Some(self.cmp(other))
    }
}

#[derive(Display, FromStr)]
#[display("{oponent} {me}")]
struct Round {
    oponent: Hand,
    me: Hand
}

impl From<&Strategy> for Round {
    fn from(strategy: &Strategy) -> Self {
        let hand = match (strategy.oponent, strategy.outcome) {
            (Hand::Rock, Outcome::Loss) => Hand::Scissors,
            (Hand::Rock, Outcome::Draw) => Hand::Rock,
            (Hand::Rock, Outcome::Win) => Hand::Paper,
            (Hand::Paper, Outcome::Loss) => Hand::Rock,
            (Hand::Paper, Outcome::Draw) => Hand::Paper,
            (Hand::Paper, Outcome::Win) => Hand::Scissors,
            (Hand::Scissors, Outcome::Loss) => Hand::Paper,
            (Hand::Scissors, Outcome::Draw) => Hand::Scissors,
            (Hand::Scissors, Outcome::Win) => Hand::Rock,
        };

        Round {
            oponent: strategy.oponent,
            me: hand
        }
    }
}

#[derive(Display, FromStr)]
#[display("{oponent} {outcome}")]
struct Strategy {
    oponent: Hand,
    outcome: Outcome
}

impl Round {
    fn outcome_points(&self) -> u32 {
        match self.me.cmp(&self.oponent) {
            Ordering::Less => 0,
            Ordering::Equal => 3,
            Ordering::Greater => 6,
        }
    }

    fn shape_points(&self) -> u32 {
        match self.me {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }

    fn points(&self) -> u32 {
        self.shape_points() + self.outcome_points()
    }
}

#[aoc_generator(day2, part1)]
fn parse_part1(data: &str) -> Result<Vec<Round>, ParseError> {
    data.lines().map(str::parse).collect()
}

#[aoc(day2, part1)]
fn part1(data: &Vec<Round>) -> u32 {
    data.iter().map(Round::points).sum()
}

#[aoc_generator(day2, part2)]
fn parse_part2(data: &str) -> Result<Vec<Strategy>, ParseError> {
    data.lines().map(str::parse).collect()
}

#[aoc(day2, part2)]
fn part2(data: &Vec<Strategy>) -> u32 {
    data.iter().map(Round::from).map(|r| r.points()).sum()
}
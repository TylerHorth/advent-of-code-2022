use aoc_runner_derive::{aoc, aoc_generator};
use parse_display::{FromStr, Display, ParseError};

#[derive(Display, FromStr, Clone, Copy)]
#[display("move {count} from {from} to {to}")]
struct Move {
    count: usize,
    from: usize,
    to: usize
}

fn transpose(data: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let len = data[0].len();
    let mut result: Vec<_> = (0..len).map(|_| Vec::new()).collect();

    for line in data {
        for (i, c) in line.into_iter().enumerate() {
            if c == ' ' {
                continue;
            }
            result[i].push(c)
        }
    }

    result
}

fn parse_stacks(data: &str) -> Vec<Vec<char>> {
    let rows: Vec<Vec<char>> = data.lines()
        .rev()
        .skip(1)
        .map(|l| l.chars().skip(1).step_by(4).collect())
        .collect();

    transpose(rows)
}

#[aoc_generator(day5)]
fn parse(data: &str) -> Result<(Vec<Vec<char>>, Vec<Move>), ParseError> {
    let (stacks_data, moves_data) = data.split_once("\n\n").unwrap();

    let stacks = parse_stacks(stacks_data);
    let moves = moves_data.lines().map(str::parse).collect::<Result<_, _>>()?;

    Ok((stacks, moves))
}

fn create_string(stack: &Vec<Vec<char>>) -> String {
    let mut result = String::new();

    for s in stack {
        result.push(*s.last().unwrap());
    }

    result
}

#[aoc(day5, part1)]
fn part1((stack, moves): &(Vec<Vec<char>>, Vec<Move>)) -> String {
    let mut stack = stack.clone();

    for &Move { count, from, to } in moves {
        for _ in 0..count {
            let c = stack[from - 1].pop().unwrap();
            stack[to - 1].push(c);
        }
    }

    create_string(&stack)
}

#[aoc(day5, part2)]
fn part2((stack, moves): &(Vec<Vec<char>>, Vec<Move>)) -> String {
    let mut stack = stack.clone();

    for &Move { count, from, to } in moves {
        let range = stack[from - 1].len() - count..;
        let crates: Vec<_> = stack[from - 1].drain(range).collect();
        stack[to - 1].extend(crates);
    }

    create_string(&stack)
}
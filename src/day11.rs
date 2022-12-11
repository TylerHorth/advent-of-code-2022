use std::{collections::VecDeque, cmp::Reverse};

use aoc_runner_derive::aoc;

struct Monkey {
    items: VecDeque<u64>,
    operation: fn(u64) -> u64,
    test: u64,
    if_true: usize,
    if_false: usize,
    inspection_count: usize,
}

impl Monkey {
    fn new<const N: usize>(items: [u64; N], operation: fn(u64) -> u64, test: u64, if_true: usize, if_false: usize) -> Self {
        Monkey {
            items: VecDeque::from(items),
            operation,
            test,
            if_true,
            if_false,
            inspection_count: 0
        }
    }

    fn inspect(&mut self, stress_damper: impl Fn(u64) -> u64) -> Option<(usize, u64)> {
        let mut item = self.items.pop_front()?;
        item = (self.operation)(item);
        item = (stress_damper)(item);

        self.inspection_count += 1;

        let index = if item % self.test == 0 {
            self.if_true
        } else {
            self.if_false
        };

        Some((index, item))
    }

    fn add(&mut self, item: u64) {
        self.items.push_back(item)
    }
}

fn monkeys() -> [Monkey; 8] { 
    [
        Monkey::new([85, 79, 63, 72], |old| old * 17, 2, 2, 6),
        Monkey::new([53, 94, 65, 81, 93, 73, 57, 92], |old| old * old, 7, 0, 2),
        Monkey::new([62, 63], |old| old + 7, 13, 7, 6),
        Monkey::new([57, 92, 56], |old| old + 4, 5, 4, 5),
        Monkey::new([67], |old| old + 5, 3, 1, 5),
        Monkey::new([85, 56, 66, 72, 57, 99], |old| old + 6, 19, 1, 0),
        Monkey::new([86, 65, 98, 97, 69], |old| old * 13, 11, 3, 7),
        Monkey::new([87, 68, 92, 66, 91, 50, 68], |old| old + 2, 17, 4, 3)
    ]
}

fn simulate(monkeys: &mut [Monkey; 8], rounds: usize, stress_damper: impl Fn(u64) -> u64) -> usize {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some((index, item)) = monkeys[i].inspect(&stress_damper) {
                monkeys[index].add(item);
            }
        }
    }

    monkeys.sort_by_key(|monkey| Reverse(monkey.inspection_count));
    monkeys[0].inspection_count * monkeys[1].inspection_count
}

#[aoc(day11, part1)]
fn part1(_: &[u8]) -> usize {
    let mut monkeys = monkeys();
    simulate(&mut monkeys, 20, |item| item / 3)
}

#[aoc(day11, part2)]
fn part2(_: &[u8]) -> usize {
    let mut monkeys = monkeys();
    let product: u64 = monkeys.iter().map(|m| m.test).product();
    simulate(&mut monkeys, 10000, |item| item % product)
}
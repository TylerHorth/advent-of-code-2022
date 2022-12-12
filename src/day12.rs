use std::{collections::{HashSet, BinaryHeap}, ops::Index, cmp::Reverse};

use aoc_runner_derive::{aoc, aoc_generator};

struct Grid {
    start: (usize, usize),
    dest: (usize, usize),
    width: usize,
    height: usize,
    grid: Vec<Vec<u8>>
}

impl Grid {
    fn new(mut grid: Vec<Vec<u8>>) -> Self {
        let height = grid.len();
        let width = grid[0].len();

        let mut start = None;
        let mut dest = None;

        for y in 0..height {
            for x in 0..width {
                if grid[y][x] == b'S' {
                    start = Some((x, y));
                    grid[y][x] = b'a';
                }
                if grid[y][x] == b'E' {
                    dest = Some((x, y));
                    grid[y][x] = b'z';
                }
            }
        }

        Self {
            start: start.unwrap(),
            dest: dest.unwrap(),
            width,
            height,
            grid
        }
    }

    pub fn distance_from_dest(&self, success: impl Fn((usize, usize)) -> bool) -> usize {
        let mut visited = HashSet::new();
        let mut heap = BinaryHeap::new();

        visited.insert(self.dest);
        heap.push((Reverse(0usize), self.dest));

        while let Some((steps, next)) = heap.pop() {
            if success(next) {
                return steps.0
            }

            for neighbor in self.neighbors(next) {
                if visited.insert(neighbor) {
                    heap.push((Reverse(steps.0 + 1), neighbor));
                }
            }
        }

        panic!("no solution")
    }

    fn add(&self, (x, y): (usize, usize), (dx, dy): (isize, isize)) -> Option<(usize, usize)> {
        let (x, y) = (x as isize + dx, y as isize + dy);

        if x < 0 || y < 0 {
            return None
        }

        let (x, y) = (x as usize, y as usize);
        if x < self.width && y < self.height {
            Some((x, y))
        } else {
            None
        }
    }

    fn can_step(&self, (x1, y1): (usize, usize), (x2, y2): (usize, usize)) -> bool {
        self.grid[y1][x1] >= self.grid[y2][x2] - 1
    }

    const DIRECTIONS: [(isize, isize); 4] = [
        (1, 0),
        (-1, 0),
        (0, 1),
        (0, -1),
    ];

    fn neighbors<'a>(&'a self, source: (usize, usize)) -> impl Iterator<Item=(usize, usize)> + 'a {
        Self::DIRECTIONS.iter()
            .copied()
            .filter_map(move |dir| self.add(source, dir))
            .filter(move |&to| self.can_step(to, source))
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = u8;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.grid[y][x]
    }
}

#[aoc_generator(day12)]
fn parse(data: &str) -> Grid {
    let grid = data.lines().map(|l| l.as_bytes().to_vec()).collect();
    Grid::new(grid)
}

#[aoc(day12, part1)]
fn part1(grid: &Grid) -> usize{
    grid.distance_from_dest(|p| p == grid.start)
}

#[aoc(day12, part2)]
fn part2(grid: &Grid) -> usize{
    grid.distance_from_dest(|p| grid[p] == b'a')
}
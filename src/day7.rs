use std::{path::{Path, PathBuf}, collections::BTreeMap};

use aoc_runner_derive::{aoc, aoc_generator};

const MAX_DIR_SIZE: u64 = 100000;
const SPACE_AVAILABLE: u64 = 70000000;
const MIN_SPACE: u64 = 30000000;

#[derive(Default)]
struct Dir {
    children: BTreeMap<String, Dir>,
    size: u64,
}

impl Dir {
    pub fn insert(&mut self, path: &Path, size: u64) {
        self.size += size;

        let os_str = path.as_os_str();
        if os_str.is_empty() || os_str == "/" {
            return;
        }

        let mut components = path.components();
        let dir = components.next().unwrap();
        let path = components.as_path();

        self.children.entry(dir.as_os_str().to_str().unwrap().to_string())
            .or_insert_with(Default::default)
            .insert(path, size);
    }

    pub fn sum_under(&self) -> u64 {
        let mut sum = self.children.values().map(Self::sum_under).sum();
        if self.size < MAX_DIR_SIZE { 
            sum += self.size;
        }
        sum
    }
    
    pub fn find_smallest(&self, min_size: u64) -> Option<u64> {
        if self.size < min_size {
            return None;
        }

        let min = self.children.values().fold(self.size, |min, dir| {
            dir.find_smallest(min_size).unwrap_or(min).min(min)
        });

        Some(min)
    }
}

#[aoc_generator(day7)]
fn parse(data: &str) -> Dir {
    let mut path = PathBuf::from("/");
    let mut dir = Dir::default();

    for line in data.lines().skip(1) {
        if line.starts_with("$") {
            let command = line.strip_prefix("$ ").unwrap();
            match &command[0..2] {
                "cd" => {
                    let new_path = &command[3..];
                    if new_path == ".." {
                        path.pop();
                    } else {
                        path.push(new_path);
                    }
                },
                "ls" => continue,
                cmd => panic!("unrecognized command {}", cmd)
            }
        } else if line.starts_with("dir") {
            continue
        } else {
            let (size, _name) = line.split_once(' ').unwrap();
            let size = size.parse::<u64>().unwrap();
            dir.insert(&path, size);
        }
    }

    dir
}

#[aoc(day7, part1)]
fn part1(dir: &Dir) -> u64 {
    dir.sum_under()
}

#[aoc(day7, part2)]
fn part2(dir: &Dir) -> u64 {
    dir.find_smallest(dir.size - (SPACE_AVAILABLE - MIN_SPACE)).unwrap()
}
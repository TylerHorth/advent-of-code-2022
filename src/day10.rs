use aoc_runner_derive::{aoc, aoc_generator};

use parse_display::{FromStr, Display, ParseError};

#[derive(Display, FromStr, Clone, Copy)]
enum Instruction {
    #[display("noop")]
    Noop,
    #[display("addx {0}")]
    Addx(i32)
}

#[aoc_generator(day10)]
fn parse(data: &str) -> Result<Vec<Instruction>, ParseError> {
    data.lines().map(str::parse).collect()
}

struct Cpu<'a> {
    program: &'a [Instruction],
    adding: Option<i32>,
    cycle: usize,
    pc: usize,
    x: i32,
}

impl <'a> Cpu<'a> {
    fn new(program: &'a [Instruction]) -> Cpu {
        Cpu {
            program,
            cycle: 1,
            adding: None,
            pc: 0,
            x: 1
        }
    }

    fn tick(&mut self) {
        self.cycle += 1;

        if let Some(add) = self.adding.take() {
            self.x += add;
            self.pc += 1;
            return;
        }

        match self.program[self.pc] {
            Instruction::Noop => self.pc += 1,
            Instruction::Addx(add) => {
                self.adding = Some(add);
            },
        }
    }

    fn print(&self) {
        let pos = (self.cycle - 1) % 40;

        if (self.x - pos as i32).abs() <= 1 {
            print!("#");
        } else {
            print!(" ");
        }

        if pos == 39 {
            println!()
        }
    }

    fn signal(&self) -> i32 {
        self.x * self.cycle as i32
    }
}

#[aoc(day10, part1)]
fn part1(program: &Vec<Instruction>) -> i32 {
    let mut signal = 0;
    let mut cpu = Cpu::new(program);

    for i in 1..=220 {
        if (i - 20) % 40 == 0 {
            signal += cpu.signal();
        }

        cpu.tick();
    }

    signal
}

#[aoc(day10, part2)]
fn part2(program: &Vec<Instruction>) -> i32 {
    let mut cpu = Cpu::new(program);

    for i in 0..240 {
        if i % 5 == 0 {
            if i % 10 == 0 {
                print!("\u{001b}[31;1m"); // bright red
            } else {
                print!("\u{001b}[32;1m"); // bright green
            }
        }
        cpu.print();
        cpu.tick();
    }

    println!("\u{001b}[0m"); // reset

    0
}
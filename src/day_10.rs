#![allow(dead_code)]

use std::collections::VecDeque;

pub trait Cadence: Sized {
    fn start(&mut self);
    fn during(&mut self);
    fn after(&mut self);
}

pub trait Draw {
    fn draw(&mut self, cpu: &CPU, cycle: &Cycle);
}

#[derive(Debug, Clone, Copy)]
pub struct Pixel(u8);

impl Pixel {
    fn low() -> Self {
        Self(0)
    }
    fn high() -> Self {
        Self(1)
    }
}

pub struct Sprite {
    cycle: Cycle,
    cpu: CPU,
}

impl From<(&Cycle, &CPU)> for Sprite {
    fn from((cycle, cpu): (&Cycle, &CPU)) -> Self {
        Self {
            cycle: cycle.clone(),
            cpu: cpu.clone(),
        }
    }
}

impl Sprite {
    fn middle(&self) -> isize {
        self.cpu.0
    }
    fn visible(&self) -> bool {
        self.middle() == self.cycle.pixel_row_position() as isize
            || (self.middle() - 1) == self.cycle.pixel_row_position() as isize
            || (self.middle() + 1) == self.cycle.pixel_row_position() as isize
    }
    fn produce(&self) -> Pixel {
        if self.visible() {
            return Pixel::high();
        }
        Pixel::low()
    }
    fn column(&self) -> usize {
        self.cycle.pixel_colum_position()
    }
}

#[derive(Debug, Clone)]
pub struct CRT([[Pixel; 40]; 6]);

impl Default for CRT {
    fn default() -> Self {
        Self([[Pixel::low(); 40]; 6])
    }
}

impl Draw for CRT {
    fn draw(&mut self, cpu: &CPU, cycle: &Cycle) {
        let sprite: Sprite = (cycle, cpu).into();
        self.0[sprite.column()][cycle.pixel_row_position()] = sprite.produce();
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cycle(usize);

impl Cycle {
    fn relevant(&self) -> bool {
        [20, 60, 100, 140, 180, 220].contains(&self.0)
    }
    fn pixel_row_position(&self) -> usize {
        (self.0 - 1) % 40
    }
    fn pixel_colum_position(&self) -> usize {
        (self.0 - 1) / 40
    }
}

impl Cadence for Clock {
    fn start(&mut self) {
        if self.busy == 0 {
            self.current = self.instructions.0.pop_front();
            self.busy = self.current.as_ref().unwrap().as_cycles();
        }
    }

    fn during(&mut self) {
        self.draw(&self.cpu.clone(), &self.cycle.clone());
        if self.cycle.relevant() {
            self.signals.push((self.cycle, self.cpu))
        }
    }

    fn after(&mut self) {
        if self.busy > 0 {
            self.busy -= 1;
        }
        if self.busy == 0 {
            match self.current {
                Some(Instruction::AddX(v)) => {
                    self.cpu.0 += v;
                }
                _ => {}
            };
        }
    }
}

impl Draw for Clock {
    fn draw(&mut self, cpu: &CPU, cycle: &Cycle) {
        self.crt.draw(cpu, cycle);
    }
}

pub struct Clock {
    cpu: CPU,
    cycle: Cycle,
    crt: CRT,
    busy: usize,
    current: Option<Instruction>,
    instructions: Instructions,
    signals: Vec<(Cycle, CPU)>,
}

impl Clock {
    pub fn new(instructions: Instructions) -> Self {
        Self {
            cpu: Default::default(),
            cycle: Default::default(),
            crt: Default::default(),
            busy: 0,
            current: None,
            instructions,
            signals: vec![],
        }
    }

    pub fn execute(&mut self) {
        for tick in 1..=240 {
            self.cycle = Cycle(tick);
            self.start();
            self.during();
            self.after();
        }
    }

    pub fn total_signals_strength(&self) -> isize {
        self.signals
            .iter()
            .fold(0, |acc, (cycle, cpu)| acc + (cycle.0 as isize * cpu.0))
    }

    pub fn draw_picture(&self) -> String {
        let mut buf = String::from("");
        for line in self.crt.0.iter() {
            for pixel in line.iter() {
                match pixel {
                    &Pixel(0) => buf.push_str("."),
                    &Pixel(1) => buf.push_str("#"),
                    _ => panic!("should not happen"),
                };
            }
            buf.push_str("\n");
        }
        buf
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CPU(isize);

impl Default for CPU {
    fn default() -> Self {
        Self(1)
    }
}

impl Default for Cycle {
    fn default() -> Self {
        Self(0)
    }
}

#[derive(Debug)]
pub enum Instruction {
    AddX(isize),
    NoOp,
}

impl Instruction {
    fn as_cycles(&self) -> usize {
        match self {
            Instruction::AddX(_) => 2,
            Instruction::NoOp => 1,
        }
    }
}

pub struct Instructions(pub VecDeque<Instruction>);

impl From<&str> for Instruction {
    fn from(v: &str) -> Self {
        match &v[0..4] {
            "addx" => Self::AddX(v[5..].parse().expect("should be a digit")),
            "noop" => Self::NoOp,
            _ => panic!("unknown instruction"),
        }
    }
}

impl From<&str> for Instructions {
    fn from(v: &str) -> Self {
        let mut instructions = VecDeque::new();
        for line in v.lines() {
            instructions.push_back(Instruction::from(line));
        }
        Self(instructions)
    }
}

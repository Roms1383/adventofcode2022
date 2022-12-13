#![allow(dead_code)]

use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
pub struct Cycle(usize);

impl Cycle {
    fn relevant(&self) -> bool {
        [20, 60, 100, 140, 180, 220].contains(&self.0)
    }
}

pub trait Cadence: Sized {
    fn start(&mut self);
    fn during(&mut self);
    fn after(&mut self);
}

impl Cadence for Clock {
    fn start(&mut self) {
        if self.busy == 0 {
            self.current = self.instructions.0.pop_front();
            self.busy = self.current.as_ref().unwrap().as_cycles();
        }
    }

    fn during(&mut self) {
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

pub struct Clock {
    cpu: CPU,
    cycle: Cycle,
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
            busy: 0,
            current: None,
            instructions,
            signals: vec![],
        }
    }

    pub fn execute(&mut self) {
        for tick in 1..=220 {
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

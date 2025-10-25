use std::fs::File;
use std::io::{BufReader, Lines};

#[derive(Debug)]
pub struct Cpu {
    pub cycle: usize,
    pub register: isize,
}

impl Cpu {
    pub fn new() -> Self {
        Cpu {
            cycle: 0,
            register: 1,
        }
    }
    pub fn do_op(&self, operation: Option<isize>) -> Self {
        if let Some(dr) = operation {
            return Cpu {
                cycle: self.cycle + 2,
                register: self.register + dr,
            };
        } else {
            return Cpu {
                cycle: self.cycle + 1,
                register: self.register,
            };
        }
    }
}

pub fn read_ops(lines: Lines<BufReader<File>>) -> impl Iterator<Item = Option<isize>> {
    lines.map_while(Result::ok).map(|elem| {
        if elem == "noop" {
            None
        } else {
            let mut vals = elem.split_whitespace();
            let ds = vals.nth(1).unwrap().parse::<isize>().unwrap();
            Some(ds)
        }
    })
}

mod cpu;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("data.txt").unwrap();
    let reader = BufReader::new(file).lines();
    let ops = cpu::read_ops(reader);
    let mut cpu = cpu::Cpu::new();
    let mut vals: Vec<isize> = Vec::new();
    let mut allvals: Vec<isize> = Vec::with_capacity(241);
    for op in ops {
        let new_cpu = cpu.do_op(op);
        if (new_cpu.cycle + 20) % 40 == 0 {
            vals.push(cpu.register);
        } else if (new_cpu.cycle + 19) % 40 == 0 && (new_cpu.cycle - cpu.cycle) == 2 {
            vals.push(cpu.register);
        }

        if new_cpu.cycle - cpu.cycle == 2 {
            allvals.push(cpu.register);
            allvals.push(cpu.register);
        } else {
            allvals.push(cpu.register);
        }
        cpu = new_cpu;
    }
    let score = vals
        .iter()
        .enumerate()
        .fold(0, |acc, (i, &val)| acc + (i * 40 + 20) * val as usize);

    let pixels: Vec<char> = allvals
        .iter()
        .enumerate()
        .map(|(i, &val)| {
            let xdraw = i % 40;
            let dx = val - xdraw as isize;
            if dx.abs() <= 1 { '#' } else { '.' }
        })
        .collect();

    println!("{:?}", score);
    for row in pixels.chunks(40) {
        let s = String::from_iter(row);
        println!("{}", s);
    }
}

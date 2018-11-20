#[macro_use]
extern crate aoc;

mod reg;
mod value;
mod op;
mod cpu;

use reg::Reg;
use value::Value;
use op::Op;
use cpu::Cpu;

#[aoc(2017, 23, 1)]
fn main(input: &str) -> usize {
    let code = parse_code(input);
    let mut cpu = Cpu::with_code(code);

    cpu.run();

    cpu.mul_count()
}

fn parse_code(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|line| Op::from_str(line).unwrap())
        .collect()
}

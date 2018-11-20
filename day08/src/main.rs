#[macro_use]
extern crate aoc;

use std::collections::HashMap;

#[aoc(2017, 08, 1)]
fn main(input: &str) -> i32 {
    let instructions = input
        .lines()
        .map(Instruction::from_str)
        .collect::<Vec<_>>();
    let mut regs = HashMap::<String, i32>::new();

    for instr in instructions {
        instr.execute(&mut regs);
    }

    *regs.values().max().unwrap()
}

struct Instruction<'a> {
    register: &'a str,
    op: Op,
    arg: i32,
    cond_lhs: &'a str,
    cond_rel: Rel,
    cond_rhs: i32,
}

impl<'a> Instruction<'a> {
    fn from_str(instr: &'a str) -> Self {
        let mut instr = instr.trim().split(' ');
        let mut next = || instr.next().unwrap();

        Instruction {
            register: next(),
            op: Op::from_str(next()),
            arg: next().parse::<i32>().unwrap(),
            cond_lhs: { next(); next() },
            cond_rel: Rel::from_str(next()),
            cond_rhs: next().parse::<i32>().unwrap(),
        }
    }

    fn execute(&self, regs: &mut HashMap<String, i32>) {
        let cond_lhs = *regs.get(self.cond_lhs).unwrap_or(&0);
        if self.cond_rel.test(cond_lhs, self.cond_rhs) {
            let target = regs.entry(self.register.into()).or_insert(0);
            self.op.apply(target, self.arg);
        }
    }
}

enum Op {
    Inc,
    Dec,
}

impl Op {
    fn from_str(op: &str) -> Self {
        match op {
            "inc" => Op::Inc,
            "dec" => Op::Dec,
            other => panic!("Unknown op {:?}", other),
        }
    }

    fn apply(&self, target: &mut i32, value: i32) {
        match *self {
            Op::Inc => *target += value,
            Op::Dec => *target -= value,
        }
    }
}

enum Rel {
    LT,
    GT,
    EQ,
    LEQ,
    GEQ,
    NEQ,
}

impl Rel {
    fn from_str(relation: &str) -> Self {
        match relation {
            "<" => Rel::LT,
            ">" => Rel::GT,
            "==" => Rel::EQ,
            "<=" => Rel::LEQ,
            ">=" => Rel::GEQ,
            "!=" => Rel::NEQ,
            other => panic!("Unknown relation {:?}", other),
        }
    }

    fn test(&self, lhs: i32, rhs: i32) -> bool {
        match *self {
            Rel::LT => lhs < rhs,
            Rel::GT => lhs > rhs,
            Rel::EQ => lhs == rhs,
            Rel::LEQ => lhs <= rhs,
            Rel::GEQ => lhs >= rhs,
            Rel::NEQ => lhs != rhs,
        }
    }
}

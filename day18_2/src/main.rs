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

aoc!(2017, 18, 2, |input| {
    let code = parse_code(input);
    let mut cpu0 = Cpu::with_code(code.clone(), 0);
    let mut cpu1 = Cpu::with_code(code        , 1);
    let mut count = 0;

    loop {
        let mut blocked = true;

        cpu0.run();
        
        while let Some(value) = cpu0.sent_mut().pop_front() {
            blocked = false;
            cpu1.received_mut().push_back(value);
        }

        cpu1.run();

        while let Some(value) = cpu1.sent_mut().pop_front() {
            blocked = false;
            cpu0.received_mut().push_back(value);
            count += 1;
        }

        if blocked {
            break;
        }
    }

    count
});

fn parse_code(input: &str) -> Vec<Op> {
    input
        .lines()
        .map(|line| Op::from_str(line).unwrap())
        .collect()
}

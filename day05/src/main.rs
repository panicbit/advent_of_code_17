#[macro_use]
extern crate aoc;

aoc!(2017, 5, 1, |input| {
    let mut ip: i32 = 0;
    let mut steps = 0;
    let mut instructions = input
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    while let Some(instruction) = instructions.get_mut(ip as usize) {
        ip += *instruction as i32;
        steps += 1;
        *instruction += 1;
    }

    steps
});

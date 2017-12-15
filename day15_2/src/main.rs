#[macro_use]
extern crate aoc;
extern crate itertools;

use itertools::iterate;

aoc!(2017, 15, 2, |input| {
    let mut lines = input.lines();
    let init_a = lines.next().unwrap()[24..].trim().parse::<u64>().unwrap();
    let init_b = lines.next().unwrap()[24..].trim().parse::<u64>().unwrap();
    let gen_a = iterate(init_a, |last| last * 16807 % 2147483647).filter(|a| a % 4 == 0);
    let gen_b = iterate(init_b, |last| last * 48271 % 2147483647).filter(|b| b % 8 == 0);

    gen_a.zip(gen_b)
    .take(5_000_000)
    .filter(|&(a, b)| (a & 0xFFFF) == (b & 0xFFFF))
    .count()
});

#[macro_use]
extern crate aoc;
extern crate itertools;

use itertools::{Itertools, iterate};

#[aoc(2017, 15, 1)]
fn main(input: &str) -> usize {
    let (init_a, init_b) = input.lines().map(|line| line[24..].trim().parse::<u64>().unwrap()).tuple_windows().next().unwrap();
    let gen_a = iterate(init_a, |last| last * 16807 % 2147483647);
    let gen_b = iterate(init_b, |last| last * 48271 % 2147483647);

    gen_a.zip(gen_b)
    .take(40_000_000)
    .filter(|&(a, b)| (a & 0xFFFF) == (b & 0xFFFF))
    .count()
}

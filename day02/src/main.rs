extern crate itertools;
#[macro_use] extern crate aoc;

use itertools::Itertools;

#[aoc(2017, 2, 1)]
fn main(input: &str) -> i32 {
    input
    .lines()
    .map(|line| {
        let (min, max) =
            line
            .trim()
            .split('\t')
            .map(|n| n.parse::<i32>().unwrap())
            .minmax()
            .into_option()
            .unwrap();
        max - min
    })
    .sum::<i32>()
}

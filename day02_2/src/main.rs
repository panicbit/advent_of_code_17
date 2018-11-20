extern crate itertools;
#[macro_use] extern crate aoc;

use itertools::Itertools;
use std::cmp::max;

#[aoc(2017, 2, 2)]
fn main(input: &str) -> i32 {
    let input = include_str!("../input.txt");

    let res =
        input
        .lines()
        .map(|line|
            line
            .trim()
            .split('\t')
            .map(parse_i32)
            .tuple_combinations()
            .find(|&(a, b)| a % b == 0 || b % a == 0)
            .map(|(a, b)| max(a / b, b / a))
            .unwrap()
        )
        .sum::<i32>();
    
    res
}

fn parse_i32(str: &str) -> i32 {
    str.parse::<i32>().unwrap()
}

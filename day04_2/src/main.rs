#[macro_use] extern crate aoc;
extern crate itertools;

use itertools::Itertools;

#[aoc(2017, 4, 2)]
fn main(input: &str) -> usize {
    input.lines().map(|line| {
        line.trim()
            .split_whitespace()
            .map(|word| word.chars().sorted())
            .sorted()
            .into_iter()
            .group_by(|word| word.clone())
            .into_iter()
            .all(|(_, dupes)| dupes.count() == 1)
    })
    .filter(|&valid| valid)
    .count()
}

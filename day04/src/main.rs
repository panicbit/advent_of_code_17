#[macro_use] extern crate aoc;
extern crate itertools;

use itertools::Itertools;

#[aoc(2017, 4, 1)]
fn main(input: &str) -> usize {
    input.lines().map(|line| {
        line.trim()
            .split_whitespace()
            .sorted()
            .into_iter()
            .group_by(|&word| word)
            .into_iter()
            .all(|(_, dupes)| dupes.count() == 1)
    })
    .filter(|&valid| valid)
    .count()
}

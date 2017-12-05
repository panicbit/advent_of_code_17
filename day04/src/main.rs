extern crate itertools;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");

    let res = input.lines().map(|line| {
        line.trim()
            .split_whitespace()
            .sorted()
            .into_iter()
            .group_by(|&word| word)
            .into_iter()
            .all(|(_, dupes)| dupes.count() == 1)
    })
    .filter(|&valid| valid)
    .count();

    println!("{}", res);
}

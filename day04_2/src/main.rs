extern crate permutohedron;
extern crate itertools;

use std::collections::HashMap;
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");

    let res = input.lines().map(|line| {
        line.trim()
            .split_whitespace()
            .into_iter()
            .fold(HashMap::new(), |mut hm, word| {
                let word = word.chars().sorted().into_iter().collect_vec();
                *hm.entry(word).or_insert(0) += 1;
                hm
            })
            .values()
            .all(|&amount| amount == 1)
    })
    .filter(|&valid| valid)
    .count();

    println!("{}", res);
}

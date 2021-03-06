#[macro_use]
extern crate aoc;

use std::collections::HashSet;

#[aoc(2017, 6, 1)]
fn main(input: &str) -> usize {
    let mut count = 0;
    let mut visited = HashSet::new();
    let mut banks = input
        .trim()
        .split('\t')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let num_banks = banks.len();

    while !visited.contains(&banks) {
        visited.insert(banks.clone());

        let (bank_index, blocks) = banks.iter().cloned().enumerate().rev().max_by_key(|&(_, n)| n).unwrap();
        banks[bank_index] = 0;

        (0..num_banks).cycle().skip(bank_index+1).take(blocks).for_each(|i| banks[i] += 1);

        count += 1;
    }

    count
}

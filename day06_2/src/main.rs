#[macro_use]
extern crate aoc;

use std::collections::HashSet;

aoc!(2017, 6, 1, |input| {
    let mut count = 0;
    let mut visited = HashSet::new();
    let mut banks = input
        .trim()
        .split('\t')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let num_banks = banks.len();
    let mut second_cycle = false;

    while !visited.contains(&banks) {
        visited.insert(banks.clone());

        let (bank_index, blocks) = banks.iter().cloned().enumerate().rev().max_by_key(|&(_, n)| n).unwrap();
        banks[bank_index] = 0;

        for i in (0..num_banks).cycle().skip(bank_index+1).take(blocks) {
            banks[i] += 1;
        }

        count += 1;

        if visited.contains(&banks) && !second_cycle {
            second_cycle = true;
            visited.clear();
            count = 0;
        }
    }

    count
});

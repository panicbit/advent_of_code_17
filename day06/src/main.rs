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

    visited.insert(banks.clone());

    loop {
        let max = *banks.iter().max().unwrap();
        let bank_index = banks.iter().position(|&n| n == max).unwrap();
        let mut blocks = banks[bank_index];
        let num_banks = banks.len();
        banks[bank_index] = 0;

        for i in (0..num_banks).cycle().skip(bank_index+1) {
            if blocks == 0 {
                break;
            }

            banks[i] += 1;
            blocks -= 1;
        }

        count += 1;

        if visited.contains(&banks) {
            break;
        }

        visited.insert(banks.clone());
    }

    count
});

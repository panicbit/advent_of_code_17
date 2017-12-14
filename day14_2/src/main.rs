#[macro_use]
extern crate aoc;
extern crate itertools;

use itertools::Itertools;
use std::collections::HashSet;

aoc!(2017, 14, 2, |input| {
    let input = input.trim();
    let mut to_check = (0..128)
        .flat_map(|row|
            knot_hash(&format!("{}-{}", input, row))
            .into_iter()
            .enumerate()
            .filter(|&(_, used)| used)
            .map(move |(col, _)| (row, col as isize))
        )
        .collect::<HashSet<_>>();
    let mut count = 0;

    while !to_check.is_empty() {
        let &coord = to_check.iter().next().unwrap();
        check(coord, &mut to_check);
        count += 1;
    }

    count
});

fn check(coord: (isize, isize), to_check: &mut HashSet<(isize,isize)>) {
    if !to_check.contains(&coord) {
        return;
    }

    to_check.remove(&coord);

    let (x, y) = coord;
    check((x, y + 1), to_check);
    check((x, y - 1), to_check);
    check((x + 1, y), to_check);
    check((x - 1, y), to_check);
}

fn knot_hash(input: &str) -> Vec<bool> {
    let mut numbers: Vec<i32> = (0..256).collect();
    let mut current_position = 0;
    let mut skip_size = 0;
    let lengths: Vec<usize> = input
        .trim()
        .bytes()
        .map(|n| n as usize)
        .chain([17, 31, 73, 47, 23].iter().cloned())
        .collect();

    for _ in 0..64 {
        for &length in &lengths {
            assert!(length <= numbers.len());
            reverse(&mut numbers, current_position, length);
            current_position += length + skip_size;
            skip_size += 1;
        }
    }

    numbers
        .iter().cloned()
        .chunks(16).into_iter()
        .take(16)
        .map(|block| block.fold1(|a, b| a ^ b).unwrap())
        .flat_map(|n| format!("{:08b}", n).chars().collect_vec())
        .map(|b| b == '1')
        .collect()
}

fn reverse<T>(data: &mut Vec<T>, mut start: usize, len: usize) {
    if len == 0 {
        return;
    }

    let mut end = start + len - 1;
    
    while start < end {
        let len = data.len();

        data.swap(start % len, end % len);

        start += 1;
        if start != end {
            end -= 1;
        }
    }
}

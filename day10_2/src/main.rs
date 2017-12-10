#[macro_use]
extern crate aoc;
extern crate itertools;

use itertools::Itertools;

aoc!(2017, 10, 2, |input| {
    let mut numbers: Vec<i32> = (0..256).collect();
    let mut current_position = 0;
    let mut skip_size = 0;
    let lengths: Vec<usize> = input
        .trim()
        .as_bytes()
        .iter()
        .cloned()
        .chain([17, 31, 73, 47, 23].iter().cloned())
        .map(|n| n as usize)
        .collect();

    for _ in 0..64 {
        for &length in &lengths {
            assert!(length <= numbers.len());
            reverse(&mut numbers, current_position, length);
            current_position += length + skip_size;
            skip_size += 1;
        }
    }

    let dense_hash = numbers
        .iter().cloned()
        .chunks(16).into_iter()
        .take(16)
        .map(|block| block.fold1(|a, b| a ^ b).unwrap())
        .map(|n| format!("{:02x}", n))
        .collect::<String>();

    dense_hash
});

fn reverse(data: &mut Vec<i32>, mut start: usize, len: usize) {
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

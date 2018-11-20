#[macro_use]
extern crate aoc;

#[aoc(2017, 10, 1)]
fn main(input: &str) -> i32 {
    let mut numbers: Vec<i32> = (0..256).collect();
    let mut current_position = 0;
    let mut skip_size = 0;
    let lengths: Vec<usize> = input.trim().split(',').map(|n| n.parse().unwrap()).collect();

    for length in lengths {
        assert!(length <= numbers.len());
        reverse(&mut numbers, current_position, length);
        current_position += length + skip_size;
        skip_size += 1;
    }

    numbers[0] * numbers[1]
}

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

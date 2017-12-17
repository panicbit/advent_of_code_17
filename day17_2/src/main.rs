#[macro_use]
extern crate aoc;

aoc!(2017, 17, 2, |input| {
    let dist = input.trim().parse::<usize>().unwrap();
    let mut current_pos = 0;
    let mut val1 = 0;
    let mut len = 1;

    for num in 1..(50000000) {
        current_pos = (current_pos + dist) % len + 1;
        if current_pos == 1 {
            val1 = num;
        }
        len += 1;
    }

    val1
});

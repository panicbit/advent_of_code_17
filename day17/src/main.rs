#[macro_use]
extern crate aoc;

aoc!(2017, 17, 1, |input| {
    let dist = input.trim().parse::<usize>().unwrap();
    let mut current_pos = 0;
    let mut buf = vec![0];
    
    for num in 1..2018 {
        current_pos = (current_pos + dist) % buf.len() + 1;
        buf.insert(current_pos, num);
    }

    buf[(current_pos + 1) % buf.len()]
});

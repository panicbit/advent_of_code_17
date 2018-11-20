#[macro_use] extern crate aoc;

#[aoc(2017, 1, 2)]
fn main(input: &str) -> u32 {
    let mut sum = 0;
    let digits: Vec<u32> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    for i in 0..digits.len() {
        let j = (i + digits.len() / 2) % digits.len();
        if digits[i] == digits[j] {
            sum += digits[i];
        }
    }

    sum
}

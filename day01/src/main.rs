fn main() {
    let input = include_str!("../input.txt").trim();
    let mut sum = 0;
    let digits: Vec<u32> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    for i in 0..digits.len() {
        let j = (i + 1) % digits.len();
        if digits[i] == digits[j] {
            sum += digits[i];
        }
    }

    println!("{}", sum);
}

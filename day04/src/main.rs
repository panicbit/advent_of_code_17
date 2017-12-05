use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    let res = input.lines().map(|line| {
        line.trim()
            .split_whitespace()
            .into_iter()
            .fold(HashMap::new(), |mut hm, word| {
                *hm.entry(word).or_insert(0) += 1;
                hm
            })
            .values()
            .all(|&amount| amount == 1)
    })
    .filter(|&valid| valid)
    .count();

    println!("{}", res);
}

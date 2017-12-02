extern crate itertools;

use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");

    let res = input
        .lines()
        .map(|line| {
            let (min, max) =
                line
                .trim()
                .split('\t')
                .map(|n| n.parse::<i32>().unwrap())
                .minmax()
                .into_option()
                .unwrap();
            max - min
        })
        .sum::<i32>();
    
    println!("{}", res);
}

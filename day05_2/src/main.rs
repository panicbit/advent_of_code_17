fn main() {
    let input = include_str!("../input.txt");
    let mut ip: i32 = 0;
    let mut steps = 0;
    let mut instructions = input
        .lines()
        .map(|line| line.trim().parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    while let Some(instruction) = instructions.get_mut(ip as usize) {
        ip += *instruction as i32;
        steps += 1;

        if *instruction >= 3 {
            *instruction -= 1;
        }
        else {
            *instruction += 1;
        }
    }

    println!("{}", steps);
}

#[macro_use]
extern crate aoc;

aoc!(2017, 16, 2, |input| {
    let inital_programs = "abcdefghijklmnop".chars().collect::<Vec<_>>();
    let moves = input.trim()
        .split(',')
        .map(Move::from_str)
        .collect::<Vec<Move>>();
    let mut cycle_size = 0;
    
    let mut programs = inital_programs.clone();
    loop {
        cycle_size += 1;
        for mov in &moves {
            mov.execute(&mut programs);
        }

        if programs == inital_programs {
            break;
        }
    }

    let num_iters = 1_000_000_000 % cycle_size;
    for _ in 0..num_iters {
        for mov in &moves {
            mov.execute(&mut programs);
        }
    }

    programs.into_iter().collect::<String>()
});

#[derive(Copy,Clone,PartialEq,Eq,Hash)]
enum Move {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl Move {
    fn from_str(mov: &str) -> Self {
        let mut args = mov[1..].split('/');
        match mov.as_bytes()[0] {
            b's' => Move::Spin(args.next().unwrap().parse().unwrap()),
            b'x' => Move::Exchange(
                args.next().unwrap().parse().unwrap(),
                args.next().unwrap().parse().unwrap(),
            ),
            b'p' => Move::Partner(
                args.next().unwrap().parse().unwrap(),
                args.next().unwrap().parse().unwrap(),
            ),
            mov => panic!("Unknown dance move {:?}", mov as char),
        }
    }

    fn execute(&self, programs: &mut Vec<char>) {
        match *self {
            Move::Exchange(i, j) => programs.swap(i, j),
            Move::Partner(a, b) => {
                let i = programs.iter().position(|&c| c == a).unwrap();
                let j = programs.iter().position(|&c| c == b).unwrap();
                programs.swap(i, j);
            }
            Move::Spin(times) => for _ in 0..times {
                let program = programs.pop().unwrap();
                programs.insert(0, program);
            },
        }
    }
}
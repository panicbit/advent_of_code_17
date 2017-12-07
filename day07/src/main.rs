#[macro_use]
extern crate aoc;
extern crate regex;

use regex::Regex;

aoc!(2017, 07, 1, |input| {
    let re = r"(?P<name>\w+) \((?P<val>\d+)\)( -> (?P<carried>.*))?";
    let re = Regex::new(re).unwrap();

    let towers = input.trim().lines().map(|line| {
        let caps = re.captures(line).expect("caps");
        let carried = caps
            .name("carried")
            .into_iter()
            .flat_map(|carried| carried.as_str().split(", "))
            .collect::<Vec<_>>();
        
        Tower {
            name: caps.name("name").unwrap().as_str(),
            val: caps["val"].parse::<i32>().unwrap(),
            carried,
        }
    }).collect::<Vec<_>>();

    let subtowers = towers
        .iter()
        .flat_map(|tower| tower.carried.iter())
        .map(|&name| name)
        .collect::<Vec<_>>();
    
    let bottom = towers
        .iter()
        .find(|tower| !subtowers.contains(&tower.name))
        .expect("bottom not found");

    bottom.name.to_string()
});

#[derive(Debug,Clone)]
struct Tower<'a> {
    name: &'a str,
    val: i32,
    carried: Vec<&'a str>,
}

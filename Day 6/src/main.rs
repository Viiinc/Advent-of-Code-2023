use std::{fs, path::Path};

use regex::Regex;

fn winners(time: i64, distance: i64) -> i64 {
    let mut res = 0;
    for i in 1..time {
        if i*(time-i) > distance{
            res += 1;
        }
    }
    res
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let lines = data.split("\n").collect::<Vec<_>>();

    let times = Regex::new(r"\d+").unwrap().captures_iter(lines[0]).map(|o| o.get(0).unwrap().as_str().parse::<i64>().unwrap()).collect::<Vec<_>>();
    let distances = Regex::new(r"\d+").unwrap().captures_iter(lines[1]).map(|o| o.get(0).unwrap().as_str().parse::<i64>().unwrap()).collect::<Vec<_>>();

    let mut part1 = 1;

    for i in 0..times.len() {
        part1 *= winners(times[i], distances[i]);
    }

    let part2 = winners(62737565, 644102312401023);

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

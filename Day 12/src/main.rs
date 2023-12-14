use std::{fs, path::Path};

use regex::Regex;

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let numberre = Regex::new(r"\d+").unwrap();

    let input: Vec<(Vec<char>, Vec<usize>)> = data.lines().map(|l| {
            let temp = l.split(" ").collect::<Vec<_>>();
            (temp[0].chars().collect(), numberre.find_iter(temp[1]).map(|n| n.as_str().parse().unwrap()).collect())
        }).collect();
    
    let part1 = 0;
    let part2 = 0;

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

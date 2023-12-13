use std::{fs, path::Path};

use regex::Regex;

fn score(line: &Vec<i64>, back: bool) -> i64 {
    let mut derivatives: Vec<Vec<i64>> = vec![line.clone()];
    let mut current = line;
    while current.iter().any(|i| *i != 0) {
        let mut next = vec![];
        for i in 0..current.len() - 1 {
            next.push(current[i+1] - current[i]);
        }
        derivatives.push(next.clone());
        current = &derivatives[derivatives.len() - 1];
    }
    let mut next = vec![0];
    for i in 0..derivatives.len() - 1 {
        if back {
            next.push(derivatives[derivatives.len() - 2 - i].first().unwrap() - next.last().unwrap());
        } else {
            next.push(next.last().unwrap() + derivatives[derivatives.len() - 2 - i].last().unwrap());
        }
    }

    // derivatives.iter().rev().enumerate().for_each(f)
    *next.last().unwrap()
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let linere = Regex::new(r"-?\d+").unwrap();

    let input = data.lines().map(|l| linere.find_iter(l).map(|c| c.as_str().parse::<i64>().unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let part1 = input.iter().map(|l| score(l, false)).reduce(|a,b| a+b).unwrap();

    let part2 = input.iter().map(|l| score(l, true)).reduce(|a,b| a+b).unwrap();

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

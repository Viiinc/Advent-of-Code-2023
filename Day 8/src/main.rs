use std::{fs, path::Path, collections::{HashMap, HashSet}};

use regex::Regex;

fn gcd(mut a:usize, mut b:usize) -> usize{
    if a==b { return a; }
    if b > a { let temp = a; a = b; b = temp; }
    while b>0 { let temp = a; a = b; b = temp%b; }
    return a;
}

fn lcm(a:usize, b:usize) -> usize{
    return a*(b/gcd(a,b));
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let instrre = Regex::new(r"([LR]+)").unwrap();

    let instructions = instrre.captures(&data).unwrap().get(0).unwrap().as_str();

    let mapre = Regex::new(r"([A-Z]*) = \(([A-Z]*), ([A-Z]*)\)").unwrap();

    let maps = mapre.captures_iter(&data).map(|c| c.extract::<3>()).map(|c| {
        (c.1[0], (c.1[1], c.1[2]))
    }).collect::<HashMap<&str,(&str,&str)>>();

    let mut part1 = 0;
    let mut next = "AAA";

    while next != "ZZZ" {
        let direction = instructions.chars().nth(part1 % instructions.len()).unwrap();
        match direction {
            'L' => {
                next = maps.get(next).expect("Unknown map value").0
            },
            'R' => {
                next = maps.get(next).expect("Unknown map value").1
            },
            _ => {panic!("Unforeseen instruction")}
        }
        part1 += 1;
    }

    let starterre = Regex::new(r"([A-Z]{2}A) ").unwrap();
    let endre = Regex::new(r"([A-Z]{2}Z) ").unwrap();

    let starters = starterre.captures_iter(&data).map(|c| c.extract::<1>().1[0]).collect::<Vec<_>>();
    let ends = endre.captures_iter(&data).map(|c| c.extract::<1>().1[0]).collect::<HashSet<_>>();

    let temp = starters.iter().map(|s| {
        let mut next = *s;
        let mut res1 = 0;
        while !ends.contains(next) {
            let direction = instructions.chars().nth(res1 % instructions.len()).unwrap();
            match direction {
                'L' => {
                    next = maps.get(next).expect("Unknown map value").0
                },
                'R' => {
                    next = maps.get(next).expect("Unknown map value").1
                },
                _ => {panic!("Unforeseen instruction")}
            }
            res1 += 1;
        }
        let mut res2 = res1;
        let direction = instructions.chars().nth(res2 % instructions.len()).unwrap();
        match direction {
            'L' => {
                next = maps.get(next).expect("Unknown map value").0
            },
            'R' => {
                next = maps.get(next).expect("Unknown map value").1
            },
            _ => {panic!("Unforeseen instruction")}
        }
        res2 += 1;
        while !ends.contains(next) {
            let direction = instructions.chars().nth(res2 % instructions.len()).unwrap();
            match direction {
                'L' => {
                    next = maps.get(next).expect("Unknown map value").0
                },
                'R' => {
                    next = maps.get(next).expect("Unknown map value").1
                },
                _ => {panic!("Unforeseen instruction")}
            }
            res2 += 1;
        }
        (res1, res2)
    }).collect::<Vec<_>>();

    let part2 = temp.into_iter().map(|(a,b)| a).reduce(|a,b| lcm(a, b)).unwrap();

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

use std::{fs, path::Path, collections::HashSet};

use regex::Regex;

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    // Build grid representation (2d vector)
    let mut lines = data.split("\n").map(|g| {
        let line = g.chars().map(|l| {
            match l {
                '.' => 0,
                '*' => -2,
                _  if l.is_digit(10) => 1,
                _ => -1
            }
        });
        
        let mut res = line.collect::<Vec<_>>();
        let re = Regex::new(r"\d+").unwrap();

        let numbers = re.find_iter(g);
        for number in numbers {
            let n = number.as_str().parse::<i32>().unwrap();
            for i in number.range() {
                res[i] = n;
            }
        }

        res
    }).collect::<Vec<Vec<_>>>();

    // Add padding to not worry about out of bounds
    lines.push(vec![0; lines[0].len()]);
    lines.insert(0, vec![0; lines[0].len()]);
    lines.iter_mut().for_each(|l| {l.insert(0, 0); l.push(0)});

    let mut part1 = 0;
    let mut part2 = 0;

    for i in 1..lines.len()-1 {
        for j in 1..lines[1].len()-1 {
            if lines[i][j] <= -1 {
                // Note: This only works, if there are no duplicate numbers (e.g., doesn't work with "5@5")
                let mut res = HashSet::new();
                res.insert(lines[i-1][j]);
                res.insert(lines[i-1][j+1]);
                res.insert(lines[i-1][j-1]);
                res.insert(lines[i][j-1]);
                res.insert(lines[i][j+1]);
                res.insert(lines[i+1][j-1]);
                res.insert(lines[i+1][j]);
                res.insert(lines[i+1][j+1]);
                res.iter().for_each(|v| part1 += v);
                if res.contains(&0) {res.remove(&0);}
                if lines[i][j] == -2 && res.len() ==2 {
                    let mut temp = 1;
                    res.iter().for_each(|v| temp *= v);
                    part2 += temp;
                }
            }
        }
    }

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

use std::{fs, path::Path};

// use regex::Regex;
fn distance(a: (usize, usize), b: (usize, usize)) -> u64 {
    (a.0.abs_diff(b.0) + a.1.abs_diff(b.1)) as u64
}

fn zerocrossings(a: (usize, usize), b: (usize, usize), zerorows: &Vec<usize>, zerocols: &Vec<usize>) -> u64 {
    (zerorows.iter().filter(|i| (a.0 < **i && b.0 > **i) || (a.0 > **i && b.0 < **i)).count() 
    + zerocols.iter().filter(|j| (a.1 < **j && b.1 > **j) || (a.1 > **j && b.1 < **j)).count()) as u64
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let input: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();
    let mut input1 = input.clone();

    // Add double space rows
    let zerorows = input1.iter().enumerate().filter(|(_, row)| row.iter().all(|c| *c == '.')).map(|(i, _)| i).rev().collect::<Vec<_>>();
    zerorows.iter().for_each(|r| input1.insert(*r, vec!['.'; input1[0].len()]));

    // Add double space columns
    let mut zerocols = vec![];
    for i in 0..input1[0].len() {
        if input1.iter().all(|r| r[i] == '.') {zerocols.push(i)}
    }
    zerocols.iter().rev().for_each(|c| {
        for i in 0..input1.len() {
            input1[i].insert(*c, '.');
        }
    });

    let mut galaxies = vec![];

    for i in 0..input1.len() {
        for j in 0..input1[0].len() {
            if input1[i][j] == '#' {
                galaxies.push((i,j));
            }
        }
    }

    let mut part1 = 0;
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            part1 += distance(galaxies[i], galaxies[j]);
        }
    }

    let mut galaxies = vec![];

    for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == '#' {
                galaxies.push((i,j));
            }
        }
    }

    let mut part2 = 0;
    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            let res = distance(galaxies[i], galaxies[j]) + 999999 * zerocrossings(galaxies[i], galaxies[j], &zerorows, &zerocols);
            part2 += res;
        }
    }

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

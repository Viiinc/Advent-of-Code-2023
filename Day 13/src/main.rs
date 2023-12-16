use std::{fs, path::Path};

// use regex::Regex;
fn toggle(c: char) -> char {
    if c == '#' {
        '.'
    } else {
        '#'
    }
}

fn find_mirror_row(grid: &Vec<Vec<char>>, exclude: u64) -> u64 {
    'outer: for i in 0..grid.len()-1 {
        for j in 0..=i {
            if i+j+1 >= grid.len() {
                if (i+1) as u64 != exclude {
                    return (i+1) as u64
                } else {
                    continue 'outer;
                }
            }
            if grid[i-j] != grid[i+j+1] {continue 'outer}
        }
        if (i+1) as u64 != exclude {
            return (i+1) as u64
        }
    }
    0
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let mut input = data.split("\n\n").map(|g| g.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>()).collect::<Vec<_>>();

    let temp = input.iter().map(|g| {
        let mut transpose = vec![vec!['.'; g.len()]; g[0].len()];
        for i in 0..g.len() {
            for j in 0..g[0].len() {
                transpose[j][i] = g[i][j]
            }
        }
        100 * find_mirror_row(g, 0) + find_mirror_row(&transpose, 0)
    }).collect::<Vec<_>>();
    
    let part1 = temp.clone().into_iter().reduce(|a,b| a+b).unwrap();

    let temp2 = input.iter_mut().enumerate().map(|(index, g)| {
        let mut transpose = vec![vec!['.'; g.len()]; g[0].len()];
        for i in 0..g.len() {
            for j in 0..g[0].len() {
                transpose[j][i] = g[i][j]
            }
        }
        for i in 0..g.len() {
            for j in 0..g[0].len() {
                g[i][j] = toggle(g[i][j]);
                transpose[j][i] = toggle(transpose[j][i]);
                let res_row = 100 * find_mirror_row(g, temp[index]/100);
                let res_col = find_mirror_row(&transpose, temp[index]);
                let res = res_row + res_col;
                if res > 0 {return res}
                g[i][j] = toggle(g[i][j]);
                transpose[j][i] = toggle(transpose[j][i]);
            }
        }
        return 0;
    }).collect::<Vec<_>>();

    let part2 = temp2.into_iter().reduce(|a,b| a+b).unwrap();

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

use std::{fs, path::Path};

const START: (usize, usize) = (0,1);

fn possibilities_slippery(grid: &Vec<Vec<char>>, position: (usize, usize)) -> Vec<(usize, usize)> {
    let mut possibilities = vec![];
    if grid[position.0 + 1][position.1] == '.' || grid[position.0 + 1][position.1] == 'v' || grid[position.0 + 1][position.1] == '>' {
        possibilities.push((position.0 + 1, position.1));
    }
    if grid[position.0 - 1][position.1] == '.' || grid[position.0 - 1][position.1] == '>' {
        possibilities.push((position.0 - 1, position.1));
    }
    if grid[position.0][position.1 + 1] == '.' || grid[position.0][position.1 + 1] == 'v' || grid[position.0][position.1 + 1] == '>' {
        possibilities.push((position.0, position.1 + 1));
    }
    if grid[position.0][position.1 - 1] == '.' || grid[position.0][position.1 - 1] == 'v' {
        possibilities.push((position.0, position.1 - 1));
    }
    possibilities
}

fn possibilities_good(grid: &Vec<Vec<char>>, position: (usize, usize)) -> Vec<(usize, usize)> {
    let mut possibilities = vec![];
    if grid[position.0 + 1][position.1] != 'O' && grid[position.0 + 1][position.1] != '#' {
        possibilities.push((position.0 + 1, position.1));
    }
    if grid[position.0 - 1][position.1] != 'O' && grid[position.0 - 1][position.1] != '#' {
        possibilities.push((position.0 - 1, position.1));
    }
    if grid[position.0][position.1 + 1] != 'O' && grid[position.0][position.1 + 1] != '#' {
        possibilities.push((position.0, position.1 + 1));
    }
    if grid[position.0][position.1 - 1] != 'O' && grid[position.0][position.1 - 1] != '#' {
        possibilities.push((position.0, position.1 - 1));
    }
    possibilities
}

fn longest_path(grid: &mut Vec<Vec<char>>, position: (usize, usize), slippery: bool) -> usize {
    if position.0 == grid.len() - 1 {return grid.iter().map(|r| r.iter().filter(|c| **c == 'O').count()).reduce(|a,b| a+b).unwrap() - 1}
    let possibilities = if slippery {
        possibilities_slippery(grid, position)
    } else {
        possibilities_good(grid, position)
    };
    match possibilities.len() {
        0 => {
            0
        },
        1 => {
            let next = possibilities[0];
            grid[next.0][next.1] = 'O';
            longest_path(grid, next, slippery)
        },
        _ => {
            possibilities.iter().map(|p| {
                let mut new_grid = grid.clone();
                new_grid[p.0][p.1] = 'O';
                longest_path(&mut new_grid, *p, slippery)
            }).max().unwrap()
        }
    }
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let grid= data.split("\n").map(|r| r.chars().collect::<Vec<char>>()).collect::<Vec<_>>();

    let mut part1_grid = grid.clone();
    part1_grid[START.0][START.1] = 'O';
    part1_grid[START.0 + 1][START.1] = 'O';

    let mut part2_grid = part1_grid.clone();

    // 2439 too high
    let part1 = longest_path(&mut part1_grid, (START.0 + 1, START.1), true);
    let part2 = longest_path(&mut part2_grid, (START.0 + 1, START.1), false);
    println!("Part 1: {},\nPart 2: {}", part1, part2);
}
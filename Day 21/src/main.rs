use std::{fs, path::Path};

const STEPS_PART_2: u128 = 26501365;

fn step<'a>(helper: &mut Vec<Vec<&'a str>>) -> Vec<Vec<&'a str>> {
    let mut temp = helper.clone();
    for i in 1..helper.len() - 1 {
        for j in 1..helper[0].len() - 1 {
            if helper[i][j] == "#" {continue;}
            else if helper[i][j+1] == "S" || helper[i][j-1] == "S" || helper[i+1][j] == "S" || helper[i-1][j] == "S" {
                temp[i][j] = "S";
            } else {
                temp[i][j] = ".";
            }
        }
    }
    return temp;
}

fn value(helper: &Vec<Vec<&str>>) -> u128 {
    helper.iter().map(|r| r.iter().map(|s| *s == "S").filter(|q| *q).count() as u128).reduce(|a,b| a+b).unwrap()
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let mut grid = data.split("\n").map(|l| l.split("").filter(|s| s.len() > 0).collect::<Vec<_>>()).collect::<Vec<_>>();
    grid.iter_mut().for_each(|r| {
        r.insert(0, "#");
        r.push("#"); 
    });

    grid.insert(0, vec!["#"; grid[1].len()]);
    grid.push(vec!["#"; grid[0].len()]);

    let mut helper = grid.clone();

    for _ in 0..64 {
        helper = step(&mut helper);
    }

    let part1 = value(&helper);

    for _ in 0..199 {
        helper = step(&mut helper);
    }
    let full_grid_even = value(&helper);
    let full_grid_uneven = value(&step(&mut helper));

    let mut part2 = full_grid_even;

    // hacky - assumes input grid
    let reached_grids_straight = (STEPS_PART_2 - 65) / 131;
    let _spare_steps = (STEPS_PART_2 - 65) % 131; // 0 with input grid

    let (mut helper_east, mut helper_west, mut helper_north, mut helper_south, mut ne_grid, mut nw_grid, mut se_grid, mut sw_grid) = (grid.clone(), grid.clone(), grid.clone(), grid.clone(), grid.clone(), grid.clone(), grid.clone(), grid.clone());
    'outer: for i in 0..helper.len() {
        for j in 0..helper[0].len() {
            if helper_east[i][j] == "S" {
                helper_east[i][j] = ".";
                (helper_west, helper_north, helper_south, ne_grid, nw_grid, se_grid, sw_grid) = (helper_east.clone(), helper_east.clone(), helper_east.clone(), helper_east.clone(), helper_east.clone(), helper_east.clone(), helper_east.clone());
                helper_south[1][j] = "S";
                helper_west[i][grid[0].len() - 2] = "S";
                helper_east[i][1] = "S";
                helper_north[grid.len() - 2][j] = "S";
                ne_grid[grid.len() - 2][1] = "S";
                nw_grid[grid.len() - 2][grid[0].len() - 2] = "S";
                se_grid[1][1] = "S";
                sw_grid[1][grid[0].len() - 2] = "S";
                break 'outer;
            }
        }
    }

    for _ in 0..130 {
        (helper_south, helper_east, helper_west, helper_north) = (step(&mut helper_south), step(&mut helper_east), step(&mut helper_west), step(&mut helper_north));
    }

    for _ in 0..64 {
        (se_grid, sw_grid, ne_grid, nw_grid) = (step(&mut se_grid), step(&mut sw_grid), step(&mut ne_grid), step(&mut nw_grid));
    }

    for i in 1..=(reached_grids_straight - 1) {
        if i % 2 == 0 {
            part2 += i*4*full_grid_even;
        } else {
            part2 += i*4*full_grid_uneven;
        }
    }

    part2 += value(&helper_east) + value(&helper_north) + value(&helper_south) + value(&helper_west);

    part2 += (reached_grids_straight)*(value(&se_grid) + value(&sw_grid) + value(&ne_grid) + value(&nw_grid));

    for _ in 0..131 {
        (se_grid, sw_grid, ne_grid, nw_grid) = (step(&mut se_grid), step(&mut sw_grid), step(&mut ne_grid), step(&mut nw_grid));
    }

    part2 += (reached_grids_straight - 1)*(value(&se_grid) + value(&sw_grid) + value(&ne_grid) + value(&nw_grid));

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}
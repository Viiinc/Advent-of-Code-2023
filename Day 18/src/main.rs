use std::{fs, path::Path, collections::HashSet};

use regex::Regex;

fn parse_dir(c: &str) -> usize {
    match c {
        "U" => 0,
        "R" => 1,
        "D" => 2,
        "L" => 3,
        _ => panic!(),
    }
}

fn next_pos(pos: (i32, i32), dir: usize) -> (i32, i32) {
    match dir {
        0 => (pos.0 - 1, pos.1),
        1 => (pos.0, pos.1 + 1),
        2 => (pos.0 + 1, pos.1),
        3 => (pos.0, pos.1 - 1),
        _ => panic!()
    }
}

fn next_pos_dist(pos: (i32, i32), dir: usize, steps: i32) -> (i32, i32){
    match dir {
        0 => (pos.0, pos.1 + steps),
        1 => (pos.0 + steps, pos.1),
        2 => (pos.0, pos.1 - steps),
        3 => (pos.0 - steps, pos.1),
        _ => panic!()
    }
}

fn update_inside(inside: &mut Vec<(i32, i32)>, corner1: i32, corner2: i32) -> u128 {
    if inside.iter().all(|(l, r)| *l > corner2 || *r < corner1) {
        // let temp = inside.iter().filter(|(l,r)| *l == corner2 || *r == corner1).collect::<Vec<_>>();
        inside.push((corner1, corner2));
        inside.sort();
        return 0;
    } else {
        // let temp = inside.iter().filter(|(l,r)| *l == corner2 || *r == corner1).collect::<Vec<_>>();
        // Need to remove some section - PAIN
        let equal = inside.iter().find(|s| s.0 == corner1 && s.1 == corner2);
        if equal.is_some() {
            let mut index = 0;
            for i in 0..inside.len() {
                if inside[i] == *equal.unwrap() {
                    index = i;
                    break;
                }
            }
            inside.remove(index);
            return (corner2 - corner1 + 1) as u128;
        }

        let right_smaller = inside.iter_mut().find(|s| s.1 == corner2).and_then(|r| {
            let res = corner2 - corner1;
            r.1 = corner1;
            Option::Some(res as u128)
        });
        if right_smaller.is_some() {return right_smaller.unwrap()}

        let left_smaller = inside.iter_mut().find(|s| s.0 == corner1).and_then(|r| {
            let res = corner2 - corner1;
            r.0 = corner2;
            Option::Some(res as u128)
        });
        if left_smaller.is_some() {return left_smaller.unwrap()}

        let split = inside.iter_mut().find(|s| s.0 < corner1 && s.1 > corner2).and_then(|r| {
            let res = r.1;
            r.1 = corner1;
            Option::Some(res)
        });
        if split.is_some() {
            inside.push((corner2, split.unwrap()));
            inside.sort();
            return (corner2 - corner1 - 1) as u128;
        }

        let left = inside.iter_mut().find(|s| s.1 == corner1).and_then(|i| {
            i.1 = corner2;
            Option::Some(true)
        });
        let right = inside.iter_mut().find(|s| s.0 == corner2).and_then(|i| {
            i.0 = corner1;
            Option::Some(true)
        });
        if left.is_some() && right.is_some() {
            for i in 0..inside.len() {
                if inside[i].1 > inside[i+1].0 {
                    inside[i].1 = inside[i+1].1;
                    inside.remove(i+1);
                    return 0;
                }
            }
        }
    }
    0
}

fn length(inside: &Vec<(i32,i32)>) -> u128 {
    inside.iter().map(|e| (e.1 - e.0 + 1) as u128).reduce(|a,b| a+b).unwrap()
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let parser = Regex::new(r"([LRUD]) (\d+) \(#([a-f0-9]{5})(.)\)").unwrap();

    let input = data.split("\n").map(|r| {
        parser.captures(r).unwrap().extract::<4>().1
    }).map(|r| (parse_dir(r[0]), r[1].parse::<usize>().unwrap(), (r[2], r[3]))).collect::<Vec<_>>();
    
    let mut visited = HashSet::new();

    let mut pos = (0,0);

    input.iter().for_each(|l| {
        for _ in 0..l.1 {
            pos = next_pos(pos, l.0);
            visited.insert(pos);
        }
    });

    // part1 : Reduce to algorithm used earlier day (reimplemented, too lazy to search and call function)
    let min_i = visited.iter().map(|l| l.0).min().unwrap();
    let min_j = visited.iter().map(|l| l.1).min().unwrap();
    let max_i = visited.iter().map(|l| l.0).max().unwrap();
    let max_j = visited.iter().map(|l| l.1).max().unwrap();

    let mut vis_res = vec![vec!['.'; (max_j - min_j + 1) as usize]; (max_i - min_i + 1) as usize];

    visited.iter().for_each(|v| {
        if visited.contains(&(v.0 - 1, v.1)) && visited.contains(&(v.0 + 1, v.1)) {
            vis_res[(v.0 - min_i) as usize][(v.1 - min_j) as usize] = '|'
        } else if visited.contains(&(v.0 - 1, v.1)) && visited.contains(&(v.0, v.1 - 1)) {
            vis_res[(v.0 - min_i) as usize][(v.1 - min_j) as usize] = 'J'
        } else if visited.contains(&(v.0 - 1, v.1)) && visited.contains(&(v.0, v.1 + 1)) {
            vis_res[(v.0 - min_i) as usize][(v.1 - min_j) as usize] = 'L'
        } else if visited.contains(&(v.0, v.1 - 1)) && visited.contains(&(v.0, v.1 + 1)) {
            vis_res[(v.0 - min_i) as usize][(v.1 - min_j) as usize] = '-'
        } else if visited.contains(&(v.0 + 1, v.1)) && visited.contains(&(v.0, v.1 - 1)) {
            vis_res[(v.0 - min_i) as usize][(v.1 - min_j) as usize] = '7'
        } else if visited.contains(&(v.0 + 1, v.1)) && visited.contains(&(v.0, v.1 + 1)) {
            vis_res[(v.0 - min_i) as usize][(v.1 - min_j) as usize] = 'F'
        } else {panic!()}
    });

    let mut part1 = 0;

    for i in 0..vis_res.len() {
        let mut inside = false;
        let mut last = '.';
        for j in 0..vis_res[0].len() {
            match vis_res[i][j] {
                '.' => if inside {part1 += 1; vis_res[i][j] = '#'},
                '-' => part1 += 1,
                'F' => {part1 += 1; last = 'F';}
                'L' => {part1 += 1; last = 'L';}
                'J' => {part1 += 1; if last == 'F' {inside = !inside}}
                '7' => {part1 += 1; if last == 'L' {inside = !inside}}
                '|' => {part1 += 1; inside = !inside},
                _ => panic!()
            }
        }
    }

    let input2 = input.iter().map(|i| (i32::from_str_radix(i.2.0, 16).unwrap(), i.2.1.parse::<usize>().unwrap())).collect::<Vec<_>>();
    let mut pos = (0,0);
    let mut corners =  vec![];
    input2.iter().for_each(|d| {
        pos = next_pos_dist(pos, d.1, d.0);
        corners.push(pos);
    });
    corners.sort();
    let mut stops = corners.clone().into_iter().map(|(i,_)| i).collect::<HashSet<_>>().into_iter().collect::<Vec<_>>();
    stops.sort();
    let mut part2: u128 = 0;
    let mut inside = vec![];
    for s in 0..stops.len() {
        let stop = stops[s];
        let lines = corners.iter().filter(|c| c.0 == stop).collect::<Vec<_>>();
        for i in 0..lines.len()/2 {
            let (temp1, temp2) = (lines[i*2], lines[i*2 + 1]);
            part2 += update_inside(&mut inside, temp1.1, temp2.1);
        }
        if inside.len() > 0 {
            part2 += length(&inside) * ((stops[s+1] - stop) as u128)
        }
    }

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

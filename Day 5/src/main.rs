use std::{fs, path::Path};

use regex::Regex;

fn resolve_single(input: i64, maps: &Vec<Vec<Vec<i64>>>) -> i64 {
    let mut res = input;
        for map in maps.iter() {
            for range in map.iter() {
                if range[1] <= res && range[1] + range[2] >= res {
                    res = range[0] + (res - range[1]);
                    break;
                }
            }
        }
    res
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let all_maps = data.split("\n\n").collect::<Vec<_>>();

    let seeds = Regex::new(r"\d+").unwrap().captures_iter(all_maps[0]).map(|o| o.get(0).unwrap().as_str().parse::<i64>().unwrap()).collect::<Vec<_>>();

    let (_, temp) = all_maps.split_first().unwrap();

    let re = Regex::new(r"(\d+) (\d+) (\d+)").unwrap();

    let maps = temp.iter().map(|m| {
        re.captures_iter(m).map(|c| vec![c.get(1).unwrap().as_str().parse::<i64>().unwrap(), c.get(2).unwrap().as_str().parse::<i64>().unwrap(), c.get(3).unwrap().as_str().parse::<i64>().unwrap()]).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let part1 = seeds.iter().map(|s| {
        resolve_single(*s, &maps)
    }).min().unwrap();

    let mut seed_ranges: Vec<i64> = vec![];
    for (i, seed) in seeds.iter().enumerate() {
        if i % 2 == 0 {
            for j in 0..seeds[i+1] {
                seed_ranges.push(seed + j);
            }
        }
    }

    let part2 = seed_ranges.iter().map(|s| {
        resolve_single(*s, &maps)
    }).min().unwrap();

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

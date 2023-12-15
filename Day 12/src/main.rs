use std::{fs, path::Path, collections::HashMap};

use regex::Regex;

fn dp(record: &str, groups: &Vec<usize>, mem: &mut HashMap<String, u64>) -> u64 {
    // Base cases
    if groups.len() == 0 {
        if record.contains("#") {
            return 0;
        }
        return 1;
    } else if groups[0] > record.len() {
        return 0;
    }

    // Check memory
    let key = record.to_string() + &groups.iter().map(|c| c.to_string()).collect::<String>();
    if mem.contains_key(&key) {return *mem.get(&key).unwrap()}

    // Calculate actual value
    if record.chars().nth(0).unwrap() == '.' {
        // Nothing to see here
        return dp(record.strip_prefix(".").unwrap(), groups, mem);
    }
    let mut res = 0;
    let mut done = false;
    let mut forced = false;
    for i in 0..groups[0] {
        if record.chars().nth(i).unwrap() == '#' {forced = true;}
        if record.chars().nth(i).unwrap() == '.' {
            // Can't start there, continue from '.' onwards
            if forced {
                res = 0;
            } else {
                res = dp(record.chars().skip(i).collect::<String>().as_str(), groups, mem);
            }
            done = true;
        }
    }
    if !done {
        if groups.len() >= 1 && record.len() > groups[0] && record.chars().nth(groups[0]).unwrap() == '#'{
            if record.starts_with("?") {
                res = dp(record.chars().skip(1).collect::<String>().as_str(), groups, mem);
            } else {
                res = 0;
            }
        } else {
            res = dp(record.chars().skip(groups[0] + 1).collect::<String>().as_str(), &groups.iter().skip(1).map(|i| *i).collect::<Vec<usize>>(), mem);
            if record.starts_with("?") {
                res += dp(record.chars().skip(1).collect::<String>().as_str(), groups, mem);
            }
        }
    }
    mem.insert(key, res);
    res
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let numberre = Regex::new(r"\d+").unwrap();

    let input: Vec<(&str, Vec<usize>)> = data.lines().map(|l| {
            let temp = l.split(" ").collect::<Vec<_>>();
            (temp[0], numberre.find_iter(temp[1]).map(|n| n.as_str().parse().unwrap()).collect())
        }).collect();
    
    let part1 = input.iter().map(|r| {
        let mut mem = HashMap::new();
        dp(r.0, &r.1, &mut mem)
    }).reduce(|a,b| a+b).unwrap();

    // Hacky way of duplicating input, but at least I can reuse everything else
    let part2 = input.iter().map(|r| ((r.0.to_string() +"?"+ r.0 +"?"+ r.0 +"?"+ r.0 +"?"+ r.0), vec![r.1.clone() ; 5].iter().flatten().map(|i| *i).collect())).map(|r| {
        let mut mem = HashMap::new();
        dp(&r.0, &r.1, &mut mem)
    }).reduce(|a, b| a+b).unwrap();

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

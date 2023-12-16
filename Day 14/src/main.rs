use std::{fs, path::Path, collections::HashMap};

fn load (platform: &Vec<Vec<char>>) -> usize {
    let mut res = 0;
    for i in 0..platform.len() {
        for j in 0..platform[0].len() {
            if platform[i][j] == 'O' {
                res += platform.len() - i;
            }
        }
    }
    res
}

fn north(platform: &mut Vec<Vec<char>>) {
    let mut change = true;
    while change {
        change = false;
        for i in 1..platform.len() {
            for j in 0..platform[0].len() {
                if platform[i][j] == 'O' && platform[i-1][j] == '.' {
                    platform[i-1][j] = 'O';
                    platform[i][j] = '.';
                    change = true;
                }
            }
        }
    }
}

fn east(platform: &mut Vec<Vec<char>>) {
    let mut change = true;
    while change {
        change = false;
        for i in 0..platform.len() {
            for j in 1..platform[0].len() {
                let j = platform[0].len() - j - 1;
                if platform[i][j] == 'O' && platform[i][j+1] == '.' {
                    platform[i][j+1] = 'O';
                    platform[i][j] = '.';
                    change = true;
                }
            }
        }
    }
}

fn south(platform: &mut Vec<Vec<char>>) {
    let mut change = true;
    while change {
        change = false;
        for i in 1..platform.len() {
            for j in 0..platform[0].len() {
                let i = platform.len() - i - 1;
                if platform[i][j] == 'O' && platform[i+1][j] == '.' {
                    platform[i+1][j] = 'O';
                    platform[i][j] = '.';
                    change = true;
                }
            }
        }
    }
}

fn west(platform: &mut Vec<Vec<char>>) {
    let mut change = true;
    while change {
        change = false;
        for i in 0..platform.len() {
            for j in 1..platform[0].len() {
                if platform[i][j] == 'O' && platform[i][j-1] == '.' {
                    platform[i][j-1] = 'O';
                    platform[i][j] = '.';
                    change = true;
                }
            }
        }
    }
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let input = data.lines().map(|l| l.chars().collect::<Vec<_>>()).collect::<Vec<_>>();

    let mut platform = input.clone();
    north(&mut platform);

    let part1 = load(&platform);

    platform = input.clone();

    let mut mem = HashMap::new();

    let cycles = 1000000000;

    for i in 0..cycles {
        let key = platform.clone();
        if mem.contains_key(&key) {
            let cycle_time = i - mem.get(&key).unwrap();
            if (cycles - i) % cycle_time == 0 {break}
        } else {
            mem.insert(key.clone(), i);
        }
        north(&mut platform);
        west(&mut platform);
        south(&mut platform);
        east(&mut platform);
    }

    let part2 = load(&platform);

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

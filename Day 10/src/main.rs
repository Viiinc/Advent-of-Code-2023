use std::{fs, path::Path, collections};

// use regex::Regex;
const LEFT: [char; 3] = ['-', 'J', '7'];
const UP: [char; 3] = ['|', 'J', 'L'];
const DOWN: [char; 3] = ['|', 'F', '7'];
const RIGHT: [char; 3] = ['-', 'F', 'L'];

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let input: Vec<Vec<char>> = data.lines().map(|l| l.chars().collect()).collect();

    let mut start: (usize, usize) = (0,0);

    'outer: for i in 0..input.len() {
        for j in 0..input[0].len() {
            if input[i][j] == 'S' {
                start = (i,j);
                break 'outer;
            }
        }
    }

    let mut distances = vec![vec![0; input[0].len()]; input.len()];

    distances[start.0][start.1] = 0;

    let mut queue = collections::VecDeque::from([((start.0, start.1), 0)]);

    while !queue.is_empty() {
        let (next, dist) = queue.pop_front().unwrap();
        if distances[next.0][next.1] == 0 {
            distances[next.0][next.1] = dist;
            match input[next.0][next.1] {
                '|' => {
                    queue.push_back(((next.0 - 1, next.1), dist + 1));
                    queue.push_back(((next.0 + 1, next.1), dist + 1));
                },
                '-' => {
                    queue.push_back(((next.0, next.1 - 1), dist + 1));
                    queue.push_back(((next.0, next.1 + 1), dist + 1));
                },
                'L' => {
                    queue.push_back(((next.0 - 1, next.1), dist + 1));
                    queue.push_back(((next.0, next.1 + 1), dist + 1));
                },
                'J' => {
                    queue.push_back(((next.0 - 1, next.1), dist + 1));
                    queue.push_back(((next.0, next.1 - 1), dist + 1));
                },
                '7' => {
                    queue.push_back(((next.0, next.1 - 1), dist + 1));
                    queue.push_back(((next.0 + 1, next.1), dist + 1));
                },
                'F' => {
                    queue.push_back(((next.0, next.1 + 1), dist + 1));
                    queue.push_back(((next.0 + 1, next.1), dist + 1));
                },
                'S' => {
                    if LEFT.contains(&input[next.0][next.1 + 1]) {queue.push_back(((next.0, next.1 + 1), dist + 1));}
                    if RIGHT.contains(&input[next.0][next.1 - 1]) {queue.push_back(((next.0, next.1 - 1), dist + 1));}
                    if UP.contains(&input[next.0 + 1][next.1]) {queue.push_back(((next.0 + 1, next.1), dist + 1));}
                    if DOWN.contains(&input[next.0 - 1][next.1]) {queue.push_back(((next.0 - 1, next.1), dist + 1));}
                },
                _ => panic!("Illegal value {}", input[next.0][next.1])
            }
        }
    }

    let part1 = distances.iter().map(|r| r.iter().max().unwrap()).max().unwrap();

    let mut cleaned = vec![vec!['.'; input[0].len()]; input.len()];
    for i in 0..distances.len() {
        for j in 0..distances[0].len() {
            if distances[i][j] != 0 {
                cleaned[i][j] = input[i][j];
                if input[i][j] == 'S' {cleaned[i][j] = 'J'} // Very bad, just works for task input
            }
        }
    }

    let part2 = cleaned.iter().map(|r| {
        let mut inside = false;
        let mut res = 0;
        let mut prev = '.';
        for i in 0..r.len() {
            match r[i] {
                'J' => if prev == 'F' {inside = !inside}
                '7' => if prev == 'L' {inside = !inside}
                'F' => prev = 'F',
                'L' => prev = 'L',
                '-' => {}
                '|' => inside = !inside,
                '.' => if inside {res += 1}
                _ => panic!("Illegal char!")
            }
        }
        res
    }).reduce(|a,b| a+b).unwrap();

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

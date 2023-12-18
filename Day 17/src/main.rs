use std::{fs, path::Path, collections::VecDeque};

const NORTH1: usize = 0;
const NORTH2: usize = 1;
const NORTH3: usize = 2;
const EAST1: usize = 3;
const EAST2: usize = 4;
const EAST3: usize = 5;
const SOUTH1: usize = 6;
const SOUTH2: usize = 7;
const SOUTH3: usize = 8;
const WEST1: usize = 9;
const WEST2: usize = 10;
const WEST3: usize = 11;

const EAST:usize = 0;
const SOUTH:usize = 1;
const WEST:usize = 2;
const NORTH: usize = 3;

fn next_directions(dir: usize) -> Vec<usize> {
    let base = dir / 3;
    let mut res = vec![(base*3 + 3) % 12, (base*3 + 9) % 12];
    if dir % 3 != 2 {res.push(dir + 1);}
    return res;
}

fn coordinate_for_direction(dir: usize, coordinates: (usize, usize)) -> (usize, usize) {
    match dir {
        NORTH1 | NORTH2 | NORTH3 => (coordinates.0 - 1, coordinates.1),
        EAST1 | EAST2 | EAST3 => (coordinates.0, coordinates.1 + 1),
        SOUTH1 | SOUTH2 | SOUTH3 => (coordinates.0 + 1, coordinates.1),
        WEST1 | WEST2 | WEST3 => (coordinates.0, coordinates.1 - 1),
        _ => panic!()
    }
}

fn next_position(dir: usize, coordinates: (usize, usize)) -> (usize, usize) {
    match dir {
        NORTH => (coordinates.0 - 1, coordinates.1),
        EAST => (coordinates.0, coordinates.1 + 1),
        SOUTH => (coordinates.0 + 1, coordinates.1),
        WEST => (coordinates.0, coordinates.1 - 1),
        _ => panic!()
    }
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let mut input = data.split("\n").map(|r| r.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
    
    input.iter_mut().for_each(|r| {r.insert(0, u32::MAX); r.push(u32::MAX)});
    input.insert(0, vec![u32::MAX; input[0].len()]);
    input.push(vec![u32::MAX; input[0].len()]);

    let mut distances = vec![vec![vec![u32::MAX; input[0].len()]; input.len()]; 12];
    distances[EAST1][1][2] = input[1][2];
    distances[SOUTH1][2][1] = input[2][1];

    let mut queue = VecDeque::new();

    queue.push_back((EAST2, (1, 3), distances[EAST1][1][2]));
    queue.push_back((SOUTH1, (2, 2), distances[EAST1][1][2]));
    queue.push_back((SOUTH2, (3, 1), distances[SOUTH1][2][1]));
    queue.push_back((EAST1, (2, 2), distances[SOUTH1][2][1]));
    
    while !queue.is_empty() {
        let (direction, coordinates, distance_before) = queue.pop_front().unwrap();
        if input[coordinates.0][coordinates.1] == u32::MAX || distances[direction][coordinates.0][coordinates.1] <= distance_before + input[coordinates.0][coordinates.1]  {
            continue;
        }
        let distance = distance_before + input[coordinates.0][coordinates.1];
        distances[direction][coordinates.0][coordinates.1] = distance;
        next_directions(direction).iter().for_each(|d| queue.push_back((*d, coordinate_for_direction(*d, coordinates), distance)));
    }

    // Part 1 is actually a really inefficient solution - I didn't need 12 dimensions of distances, but I guess it works
    let part1 = distances.iter().map(|g| g[g.len() - 2][g[0].len() - 2]).min().unwrap();

    let mut distances = vec![vec![vec![u32::MAX; input[0].len()]; input.len()]; 4];
    let mut queue = VecDeque::new();

    for steps in 5..=11 {
        distances[EAST][1][steps] = {
            let mut res = 0;
            for j in 2..=steps {res += input[1][j];}
            res
        };
        distances[SOUTH][steps][1] = {
            let mut res = 0;
            for i in 2..=steps {res += input[i][1];}
            res
        };
        for next_steps in 4..=10 {
            let south_dist = {
                let mut res = distances[EAST][1][steps];
                // double check if inclusive
                for i in 1..=next_steps {res += input[1+i][steps]}
                res
            };
            queue.push_back((SOUTH, (1+next_steps,steps), south_dist));
            let east_dist = {
                let mut res = distances[SOUTH][steps][1];
                // double check if inclusive
                for j in 1..=next_steps {res += input[steps][1+j]}
                res
            };
            queue.push_back((EAST, (steps,1+next_steps), east_dist));
        }
    }

    while !queue.is_empty() {
        let (direction, coordinates, distance) = queue.pop_front().unwrap();
        if input[coordinates.0][coordinates.1] == u32::MAX || distances[direction][coordinates.0][coordinates.1] <= distance {
            continue;
        }
        distances[direction][coordinates.0][coordinates.1] = distance;

        add_to_queue((direction + 1) % 4, coordinates, distance, &input, &mut queue);
        add_to_queue((direction + 3) % 4, coordinates, distance, &input, &mut queue);
    }

    let part2 = distances.iter().map(|g| g[g.len() - 2][g[0].len() - 2]).min().unwrap();

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

fn add_to_queue(direction: usize, coordinates: (usize, usize), distance: u32, input: &Vec<Vec<u32>>, queue: &mut VecDeque<(usize, (usize, usize), u32)>) {
    let mut dist = distance;
    let mut pos = coordinates;

    for index in 1..=10 {
        pos = next_position(direction, pos);
        let d = input[pos.0][pos.1];
        if d == u32::MAX {break;}
        dist += d;
        if index < 4 {continue}
        queue.push_back((direction, pos, dist));
    }
}

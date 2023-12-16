use std::{fs, path::Path, collections::{VecDeque, HashSet}};

fn valid(coordinate: (usize, usize), direction: (i16, i16), grid: &Vec<Vec<char>>, mem: &mut HashSet<((usize, usize), (i16, i16))>) -> bool {
    if mem.contains(&(coordinate, direction))
        || coordinate.0 == 0 && direction.0 == -1 || coordinate.0 == grid.len() - 1 && direction.0 == 1
        || coordinate.1 == 0 && direction.1 == -1 || coordinate.1 == grid[0].len() - 1 && direction.1 == 1
    {
        false
    } else {
        mem.insert((coordinate, direction));
        true
    }
}

fn solve(input: &Vec<Vec<char>>, start_coordinate: (usize, usize), start_dir: (i16, i16)) -> i32 {
    let mut energized = vec![vec![0; input[0].len()]; input.len()];

    let mut queue = VecDeque::new();
    let mut mem: HashSet<((usize, usize),(i16, i16))> = HashSet::new();
    queue.push_back((start_coordinate, start_dir));

    while !queue.is_empty() {
        let ((i,j), direction) = queue.pop_front().unwrap();
        energized[i][j] = 1;
        match input[i][j] {
            '\\' => {
                match direction {
                    (1,0) => {
                        if valid((i,j), (0,1), &input, &mut mem) {
                            queue.push_back(((i, j+1), (0,1)));
                        }
                    }
                    (0,1) => {
                        if valid((i,j), (1,0), &input, &mut mem) {
                            queue.push_back(((i + 1, j), (1,0)));
                        }
                    }
                    (-1,0) => {
                        if valid((i,j), (0,-1), &input, &mut mem) {
                            queue.push_back(((i, j-1), (0, -1)));
                        }
                    }
                    (0,-1) => {
                        if valid((i,j), (-1,0), &input, &mut mem) {
                            queue.push_back(((i - 1, j), (-1,0)));
                        }
                    }
                    _ => {}
                }
            }
            '/' => {
                match direction {
                    (1,0) => {
                        if valid((i,j), (0,-1), &input, &mut mem) {
                            queue.push_back(((i, j-1), (0, -1)));
                        }
                    }
                    (0,1) => {
                        if valid((i,j), (-1,0), &input, &mut mem) {
                            queue.push_back(((i - 1, j), (-1,0)));
                        }
                    }
                    (-1,0) => {
                        if valid((i,j), (0,1), &input, &mut mem) {
                            queue.push_back(((i, j+1), (0,1)));
                        }
                    }
                    (0,-1) => {
                        if valid((i,j), (1,0), &input, &mut mem) {
                            queue.push_back(((i + 1, j), (1,0)));
                        }
                    }
                    _ => {}
                }
            }
            '|' => {
                if direction == (1, 0) || direction == (-1, 0) {
                    if valid((i,j), direction, &input, &mut mem) {
                        queue.push_back((((i as i16 + direction.0) as usize,(j as i16 + direction.1) as usize), direction));
                    }
                } else {
                    if valid((i,j), (1, 0), &input, &mut mem) {
                        queue.push_back(((i + 1,j), (1, 0)));
                    }
                    if valid((i,j), (-1, 0), &input, &mut mem) {
                        queue.push_back(((i - 1, j), (-1, 0)));
                    }
                }
            }
            '-' => {
                if direction == (0, 1) || direction == (0, -1) {
                    if valid((i,j), direction, &input, &mut mem) {
                        queue.push_back((((i as i16 + direction.0) as usize,(j as i16 + direction.1) as usize), direction));
                    }
                } else {
                    if valid((i,j), (0, 1), &input, &mut mem) {
                        queue.push_back(((i,j + 1), (0, 1)));
                    }
                    if valid((i,j), (0, -1), &input, &mut mem) {
                        queue.push_back(((i,j - 1), (0, -1)));
                    }
                }
            }
            '.' => {
                if valid((i,j), direction, &input, &mut mem) {
                    queue.push_back((((i as i16 + direction.0) as usize,(j as i16 + direction.1) as usize), direction));
                }
            }
            _ => panic!("")
        }
    }
    energized.iter().map(|r| r.into_iter().map(|i| *i).reduce(|a,b| a+b).unwrap()).reduce(|a,b| a+b).unwrap()
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let input = data.split("\n").map(|r| r.chars().collect::<Vec<_>>()).collect::<Vec<_>>();
    

    let part1 = solve(&input, (0,0), (0,1));
    let mut part2 = 0;

    for i in 0..input.len() {
        part2 = part2.max(solve(&input, (i, 0), (0,1)));
        part2 = part2.max(solve(&input, (i, input[i].len() - 1), (0,-1)));
    }
    for j in 0..input[0].len() {
        part2 = part2.max(solve(&input, (0, j), (1,0)));
        part2 = part2.max(solve(&input, (input.len() - 1, j), (-1,0)));
    }

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

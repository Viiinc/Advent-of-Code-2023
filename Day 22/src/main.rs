use std::{fs, path::Path, collections::HashSet};

use regex::Regex;

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
struct Block {
    x: (usize, usize),
    y: (usize, usize),
    z: (usize, usize)
}

impl Block {
    fn new(arg: [usize; 6]) -> Self{
        Block { x: (arg[0], arg[3]), y: (arg[1], arg[4]), z: (arg[2], arg[5]) }
    }

    fn intersect(&self, other: &Self) -> bool {
        intersect(self.x, other.x) && intersect(self.y, other.y)
    }

    fn supports(&self, other: &Self) -> bool {
        self.intersect(other) && other.z.0 == self.z.1 + 1
    }

    fn fall(&mut self, height: usize) {
        self.z.1 = self.z.1 - self.z.0 + height;
        self.z.0 = height;
    }

    fn _illegal(&self, other: &Self) -> bool {
        self.intersect(other) && intersect(self.z, other.z)
    }

    fn supporters<'a, 'b>(&'a self, stack: &'b Vec<Block>) -> HashSet<Block>{
        stack.iter().filter(|o| o.supports(self)).map(|t| *t).collect()
    }
}

impl PartialOrd for Block {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Block {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.z.0.cmp(&other.z.0)
    }
}

fn intersect(a: (usize, usize), b: (usize, usize)) -> bool {
    (a.0 <= b.1 && a.0 >= b.0) || (a.1 <= b.1 && a.1 >= b.0) || (b.0 >= a.0 && b.0 <= a.1)
}

fn fall(tower: &Vec<Block>, target: usize) -> usize {
    let mut fallen = HashSet::new();
    fallen.insert(tower[target].clone());
    let mut oldcount = 2;
    let mut newcount = 0;
    while oldcount != newcount {
        oldcount = newcount;
        let toppled = tower.iter().filter(|b| {
            let sup = b.supporters(tower);
            return sup.len() != 0 && sup.intersection(&fallen).count() == sup.len()
        }).collect::<Vec<_>>();
        toppled.iter().for_each(|t| {fallen.insert((*t).clone());});
        newcount = fallen.len() - 1;
    }
    fallen.len() - 1
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let re = Regex::new(r"(\d+),(\d+),(\d+)~(\d+),(\d+),(\d+)").unwrap();

    let mut blocks = re.captures_iter(&data).map(|r| r.extract::<6>().1.map(|d| d.parse::<usize>().unwrap())).map(|s| Block::new(s)).collect::<Vec<_>>();
    blocks.sort();

    let mut fallen: Vec<Block> = vec![];

    blocks.iter().for_each(|b| {
        let mut c = b.clone();
        let bottom = fallen.iter().rev().find(|o| o.intersect(&c));
        if bottom.is_some() {
            c.fall(bottom.unwrap().z.1 + 1);
            fallen.push(c);
        } else {
            c.fall(1);
            fallen.push(c);
        }
        fallen.sort_by(|a, b| a.z.1.cmp(&b.z.1));
    });

    let part1 = fallen.iter().filter(|b| {
        let supported = fallen.iter().filter(|g| b.supports(g)).collect::<Vec<_>>();
        supported.iter().all(|&q| {
            fallen.iter().filter(|t| t.supports(q) && t!= b).count() != 0
        })
    }).count();
    let mut part2 = 0;
    for i in 0..fallen.len() {
        part2 += fall(&fallen, i);
    }

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}
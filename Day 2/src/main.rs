use std::{fs, path::Path, cmp};

use regex::Regex;

#[derive(Debug)]
struct Game {
    game: u32,
    rounds: Vec<Round>
}

impl Game {
    fn illegal(&self) -> bool {
        self.rounds.iter().any(|r| r.illegal())
    }

    fn power(&self) -> u64 {
        let (mut r, mut g, mut b) = (0,0,0);
        self.rounds.iter().for_each(|round| {r = cmp::max(r, round.red); g = cmp::max(g, round.green); b = cmp::max(b, round.blue)});
        r*g*b
    }
}

#[derive(Debug)]
struct Round {
    blue: u64,
    red: u64,
    green: u64
}


impl Round {
    fn illegal (&self) -> bool {
        self.blue > 14 || self.green > 13 || self.red > 12
    }
}

fn parse_round(round: &str) -> Round {
    let re_red = Regex::new(r"(\d+) red").unwrap();
    let re_blue = Regex::new(r"(\d+) blue").unwrap();
    let re_green = Regex::new(r"(\d+) green").unwrap();

    let red = re_red.captures(round).and_then(|m| m.get(1).and_then(|c| Some(c.as_str().parse::<u64>().unwrap()))).or_else(|| Some(0)).unwrap();
    let blue = re_blue.captures(round).and_then(|m| m.get(1).and_then(|c| Some(c.as_str().parse::<u64>().unwrap()))).or_else(|| Some(0)).unwrap();
    let green = re_green.captures(round).and_then(|m| m.get(1).and_then(|c| Some(c.as_str().parse::<u64>().unwrap()))).or_else(|| Some(0)).unwrap();

    Round{blue, red, green}
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let games = data.split("\n").map(|g| {
        let game = g.split(": ").collect::<Vec<_>>();
        let id = game[0].split(" ").collect::<Vec<_>>()[1].parse::<u32>().unwrap();
        let rounds = game[1].split("; ").collect::<Vec<_>>();
        return Game{game: id, rounds: rounds.iter().map(|r| parse_round(r)).collect()};
    }).collect::<Vec<_>>();

    let part1 = games.iter().filter(|g| !g.illegal()).map(|g| g.game).reduce(|a,b|a+b).unwrap();
    let part2 = games.iter().map(|g| g.power()).reduce(|a,b|a+b).unwrap();

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

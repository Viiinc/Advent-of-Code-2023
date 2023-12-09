use std::{fs, path::Path, collections::HashSet};

use regex::Regex;

#[derive(Debug)]
struct Card {
    id: u32,
    winners: HashSet<u32>,
    candidates: HashSet<u32>
}

impl Card {
    fn value(&self) -> u32 {
        let intersection = self.winners.intersection(&self.candidates).collect::<Vec<_>>().len() as u32;
        if intersection > 0 {
            2^(intersection-1)
        } else {
            0
        }
    }

    // fn illegal(&self) -> bool {
    //     self.rounds.iter().any(|r| r.illegal())
    // }

    // fn power(&self) -> u64 {
    //     let (mut r, mut g, mut b) = (0,0,0);
    //     self.rounds.iter().for_each(|round| {r = cmp::max(r, round.red); g = cmp::max(g, round.green); b = cmp::max(b, round.blue)});
    //     r*g*b
    // }
}

// fn parse_round(round: &str) -> Round {
//     let re_red = Regex::new(r"(\d+) red").unwrap();
//     let re_blue = Regex::new(r"(\d+) blue").unwrap();
//     let re_green = Regex::new(r"(\d+) green").unwrap();

//     let red = re_red.captures(round).and_then(|m| m.get(1).and_then(|c| Some(c.as_str().parse::<u64>().unwrap()))).or_else(|| Some(0)).unwrap();
//     let blue = re_blue.captures(round).and_then(|m| m.get(1).and_then(|c| Some(c.as_str().parse::<u64>().unwrap()))).or_else(|| Some(0)).unwrap();
//     let green = re_green.captures(round).and_then(|m| m.get(1).and_then(|c| Some(c.as_str().parse::<u64>().unwrap()))).or_else(|| Some(0)).unwrap();

//     Round{blue, red, green}
// }

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let re = Regex::new(r"Card\s+(\d+): ([\d\s]+)|([\d\s]+)").unwrap();
    
    let cards = data.split("\n").map(|c| {
        let all = re.captures(c).unwrap();
        let set = Regex::new(r"(\d+)").unwrap();
        let winners = set.captures(all.get(1).unwrap().as_str()).unwrap().iter().map(|o| o.unwrap().as_str().parse::<u32>().unwrap()).collect::<HashSet<_>>();
        let candidates = set.captures(all.get(2).unwrap().as_str()).unwrap().iter().map(|o| o.unwrap().as_str().parse::<u32>().unwrap()).collect::<HashSet<_>>();
        println!("{:?}", all);
        Card{id: all.get(0).unwrap().as_str().parse::<u32>().unwrap(), winners: winners, candidates: candidates}
    }).collect::<Vec<_>>();

    // let games = data.split("\n").map(|g| {
    //     let game = g.split(": ").collect::<Vec<_>>();
    //     let id = game[0].split(" ").collect::<Vec<_>>()[1].parse::<u32>().unwrap();
    //     let rounds = game[1].split("; ").collect::<Vec<_>>();
    //     return Game{game: id, rounds: rounds.iter().map(|r| parse_round(r)).collect()};
    // }).collect::<Vec<_>>();

    let part1 = cards.iter().map(|g| g.value()).reduce(|a,b|a+b).unwrap();
    let part2 = 0;

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

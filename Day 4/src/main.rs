use std::{fs, path::Path, collections::HashSet};

use regex::Regex;

#[derive(Debug)]
struct Card {
    winners: HashSet<u32>,
    candidates: HashSet<u32>
}

impl Card {
    fn value(&self) -> u32 {
        let intersection = self.winners.intersection(&self.candidates).collect::<Vec<_>>().len() as u32;
        if intersection > 0 {
            (2 as u32).pow(intersection-1)
        } else {
            0
        }
    }

    fn inter(&self) -> usize {
        self.winners.intersection(&self.candidates).collect::<Vec<_>>().len()
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

    let re = Regex::new(r"Card\s+(\d+): ([\d\s]+)\|([\d\s]+)").unwrap();
    
    let cards = data.split("\n").map(|c| {
        let all = re.captures(c).unwrap();
        // println!("{:?}", all);
        // let id = all.get(0).unwrap();
        let set = Regex::new(r"\d+").unwrap();
        // let blah = set.captures_iter(all.get(2).unwrap().as_str()).map(|o| o.get(0).unwrap().as_str().parse::<u32>().unwrap()).collect::<HashSet<_>>();
        let winners = set.captures_iter(all.get(2).unwrap().as_str()).map(|o| o.get(0).unwrap().as_str().parse::<u32>().unwrap()).collect::<HashSet<_>>();
        let candidates = set.captures_iter(all.get(3).unwrap().as_str()).map(|o| o.get(0).unwrap().as_str().parse::<u32>().unwrap()).collect::<HashSet<_>>();
        Card{winners, candidates}
    }).collect::<Vec<_>>();

    let part1 = cards.iter().map(|g| g.value()).reduce(|a,b|a+b).unwrap();

    let mut count = cards.iter().map(|_| 1).collect::<Vec<_>>();

    for (i, card) in cards.iter().enumerate() {
        let wins = card.inter();
        for j in 1..(wins+1) as usize {
            if i+j >= cards.len() {break;}
            count[i+j] += count[i]
        }
    }

    let part2 = count.into_iter().reduce(|a,b| b+a).unwrap();

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

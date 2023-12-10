use std::{fs, path::Path, collections};

// use regex::Regex;

#[derive(Clone)]
struct Hand {
    cards: Vec<u8>,
    bid: u64,
    value: u8
}

fn value(hand: &Vec<u8>) -> u8 {
    let mut helper = collections::HashMap::new();
    hand.iter().for_each(|i| {
        if helper.contains_key(i) {
            helper.insert(i, helper.get(i).unwrap() + 1);
        } else {
            helper.insert(i, 1);
        }
    });
    let temp = helper.iter().filter(|(_,v)| **v > 1).map(|(_,v)| *v).collect::<Vec<_>>();
    match temp.len() {
        2 => {
            if temp[0] + temp[1] == 5 {
                4
            } else {
                2
            }
        }
        1 => {
            if temp[0] == 5 {
                6
            } else if temp[0] == 4 {
                5
            } else if temp[0] == 3 {
                3
            } else {
                1
            }
        }
        _ => 0
    }
}

fn value_jokers(hand: &Vec<u8>) -> u8 {
    if *hand == vec![0 as u8; 5] {
        return 6;
    } 
    let mut helper = collections::HashMap::new();
    let mut jokers = 0;
    hand.iter().for_each(|i| {
        if *i == 0 {jokers += 1; return;}
        if helper.contains_key(i) {
            helper.insert(i, helper.get(i).unwrap() + 1);
        } else {
            helper.insert(i, 1);
        }
    });
    if jokers == 0 {return value(hand)}
    // still bad => JXXXY !+ JXXYY
    if helper.len() == 2 && helper.iter().all(|(_, v)| *v == 2) && jokers == 1 {return 4}
    match helper.iter().map(|(_, v)| *v).max().unwrap() + jokers {
        5 => 6,
        4 => 5,
        3 => 3,
        2 => 1,
        _ => panic!("WTH")
    }
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let hands = data.split("\n").map(|h| {
        let temp = h.split(" ").collect::<Vec<_>>();
        let cards = temp[0].split("").filter(|s| s.len() > 0).map(|c| match c {
            "T" => 10,
            "J" => 11,
            "Q" => 12,
            "K" => 13,
            "A" => 14,
            _ => c.parse::<u8>().unwrap()
        }).collect::<Vec<_>>();
        let value = value(&cards);

        Hand{cards, bid: temp[1].parse::<u64>().unwrap(), value}
    }).collect::<Vec<_>>();

    let mut ordered_hands = hands.clone();

    ordered_hands.sort_by(|a, b| {
        if a.value != b.value {
            a.value.cmp(&b.value)
        } else {
            a.cards.cmp(&b.cards)
        }
    });

    let part1 = ordered_hands.iter().enumerate().map(|(i, hand)| (i+1) as u64 * hand.bid).reduce(|a,b| a+b).unwrap();

    let hands = data.split("\n").map(|h| {
        let temp = h.split(" ").collect::<Vec<_>>();
        let cards = temp[0].split("").filter(|s| s.len() > 0).map(|c| match c {
            "T" => 10,
            "J" => 0,
            "Q" => 12,
            "K" => 13,
            "A" => 14,
            _ => c.parse::<u8>().unwrap()
        }).collect::<Vec<_>>();
        let value = value_jokers(&cards);

        Hand{cards, bid: temp[1].parse::<u64>().unwrap(), value}
    }).collect::<Vec<_>>();

    let mut ordered_hands = hands.clone();

    ordered_hands.sort_by(|a, b| {
        if a.value != b.value {
            a.value.cmp(&b.value)
        } else {
            a.cards.cmp(&b.cards)
        }
    });

    let part2 = ordered_hands.iter().enumerate().map(|(i, hand)| (i+1) as u64 * hand.bid).reduce(|a,b| a+b).unwrap();

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

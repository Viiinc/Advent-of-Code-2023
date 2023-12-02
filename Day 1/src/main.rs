use std::{fs, path::Path};

// Generic solution working for part 1 and 2
fn calculate(input: &Vec<&str>, match_set: Vec<(&str, u32)>) -> u32 {
    input.iter()
        // Find all numbers within each string
        .map(|l| {
            let mut first = l.len();
            let mut last = 0;
            let mut left: u32 = 0;
            let mut right: u32 = 0;
            match_set.iter().for_each(|(m, i)| {
                let li = l.find(m);
                if li.is_some() && li.unwrap() < first {
                    left = *i;
                    first = li.unwrap();
                }
                let ri = l.rfind(m);
                if ri.is_some() && ri.unwrap() > last {
                    right = *i;
                    last = ri.unwrap();
                }
            });
            if left == 0 {left = right}
            else if right == 0 {right = left}
            left*10 + right
        }).reduce(|a,b| a + b).unwrap()
}

fn main() {
    let match_1:Vec<(&str, u32)> = [("0", 0), ("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9)].to_vec();
    let match_2:Vec<(&str, u32)> = [("zero", 0), ("one", 1), ("two", 2), ("three", 3), ("four", 4), ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9),
        ("0", 0), ("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6), ("7", 7), ("8", 8), ("9", 9)].to_vec();

    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");
    
    let numbers: Vec<&str> = data.split("\n").filter(|q| q.len() > 0).collect::<Vec<_>>();

    // Part 1 only solution
    let part1 = numbers.iter()
        // Find all numbers within each string
        .map(|n| n.matches(char::is_numeric).map(|i| i.parse::<u32>().unwrap()))
        .map(|u| u.collect::<Vec<u32>>())
        // Calculate values
        .map(|u| u.first().unwrap()*10 + u.last().unwrap())
        // Sum for solution
        .reduce(|a,b| a + b).unwrap();

    assert_eq!(part1, calculate(&numbers, match_1));

    let part2 = calculate(&numbers, match_2);

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

/*
SKETCH JS SOLUTION

let digits = input.map(s => s.split('')).map(r => r.filter(q => q >= '0' && q <= '9'))

digits.map(d => {
    if (d.length == 1) {return d[0] + d[0]}
    else {return d[0] + d[d.length-1]}}).map(q => q*1).reduce((a, b) => a + b)

let digitVals = [['zero',0], ['one',1], ['two',2], ['three',3], ['four',4], ['five',5], ['six',6], ['seven',7], ['eight',8], ['nine',9], ['0',0],['1',1],['2',2],['3',3],['4',4],['5',5],['6',6],['7',7],['8',8],['9',9]]

let reverseInput = input.map(i => i.split('').reverse().join(''))
let reverseVals = digitVals.map(d => [d[0].split('').reverse().join(''),d[1]])

let first = input.map(i => digitVals.map(d => [i.search(d[0]),d[1]]).filter(d => d[0]>=0).sort((a,b) => {if (a[0] < b[0]) {return -1} else {return 1}})).map(i => i[0][1])
let last = reverseInput.map(i => reverseVals.map(d => [i.search(d[0]),d[1]]).filter(d => d[0]>=0).sort((a,b) => {if (a[0] < b[0]) {return -1} else {return 1}})).map(i => i[0][1])

first.map((d, i) => d*10 + last[i]).reduce((a,b) => a+b)

*/
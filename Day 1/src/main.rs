use std::{fs, path::Path};

const BASE: u64 = 5;

fn snafu_to_dec(n: Vec<char>) -> i64 {
    let mut res: i64 = 0;
    for i in 0..n.len() {
        res += BASE.pow(i as u32) as i64 * match n[n.len() - 1 -i] {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            _ => unreachable!()
        }
    }
    res
}


fn dec_to_snafu(d: i64) -> Vec<char> {
    match d {
        2 => return vec!['2'],
        1 => return vec!['1'],
        0 => return vec!['0'],
        -1 => return vec!['-'],
        -2 => return vec!['='],
        _ => {}
    }
    let mut exponent = 1;
    loop {
        if BASE.pow(exponent) >= d.abs() as u64 {
            break;
        }
        exponent += 1;
    }
    if d.abs() > (BASE.pow(exponent) / 2) as i64 {
        let mut res;
        if d > 0 {
            res = vec!['1'];
        } else {
            res = vec!['-'];
        }
        for _ in 0..exponent {res.push('0')}
        // TODO: if negative we need to ADD to d instead
        let rest = dec_to_snafu(d - BASE.pow(exponent) as i64 * d.clamp(-1, 1));
        for i in 1..=rest.len() {
            let length = res.len();
            res[length - i] = rest[rest.len() - i];
        }
        return res;
    } else {
        exponent -= 1;
        let mut count = d / BASE.pow(exponent) as i64;
        let remainder = d - count * BASE.pow(exponent) as i64;
        if count.abs() == 1 && remainder.abs() > BASE.pow(exponent) as i64 / 2 {count *= 2}
        let mut res;
        match count {
            2 => res = vec!['2'],
            1 => res = vec!['1'],
            -1 => res = vec!['-'],
            -2 => res = vec!['='],
            _ => unreachable!()
        }
        for _ in 0..exponent {res.push('0')}
        let rest = dec_to_snafu(d - count * BASE.pow(exponent) as i64);
        for i in 1..=rest.len() {
            let length = res.len();
            res[length - i] = rest[rest.len() - i];
        }
        return res;
    }
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");
    
    let numbers: Vec<Vec<char>> = data.split("\n").map(|s| s.split("").filter(|q| q.len() > 0).map(|r| r.chars().nth(0).unwrap()).collect::<Vec<_>>()).collect::<Vec<_>>();
        
    let temp = numbers.iter().map(|n| snafu_to_dec(n.to_vec())).collect::<Vec<_>>();
    let _test = temp.iter().map(|n| dec_to_snafu(*n)).collect::<Vec<_>>();
    assert_eq!(numbers, _test);
    let sum = temp.iter().sum::<i64>();

    let part1 = dec_to_snafu(sum).iter().collect::<String>();
    let part2 = 0;

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
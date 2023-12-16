use std::{fs, path::Path};

fn hash(input: &str) -> usize {
    let mut res = 0;
    input.chars().for_each(|c| {
        res += c as usize;
        res *= 17;
        res = res % 256;
    });
    res
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let input = data.split(",").collect::<Vec<_>>();

    let part1 = input.iter().map(|i| hash(*i)).reduce(|a,b| a+b).unwrap();

    let mut boxes: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];

    input.iter().for_each(|instr| {
        if instr.contains("=") {
            let temp = instr.split("=").collect::<Vec<_>>();
            let (label, strength) = (temp[0], temp[1].parse::<usize>().unwrap());
            let boxnum = hash(label);
            boxes[boxnum].iter_mut().filter(|c| c.0 == label).for_each(|c| c.1 = strength);
            if boxes[boxnum].iter().all(|c| c.0 != label) {
                boxes[boxnum].push((label, strength));
            }
        } else {
            let label = instr.strip_suffix("-").unwrap();
            let boxnum = hash(label);
            let index = boxes[boxnum].iter().enumerate().find_map(|(index, l)| {if l.0 == label {Some(index)} else {None}});
            if index.is_some() {
                boxes[boxnum].remove(index.unwrap());
            }
        }
    });

    let part2 = boxes.iter().enumerate().map(|(i, b)| {
        if b.len() == 0 {
            0
        } else {
            b.iter().enumerate().map(|(j, l)| (i+1)*(j+1)*l.1).reduce(|a,b| a+b).unwrap()
        }
    }).reduce(|a,b| a+b).unwrap();

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

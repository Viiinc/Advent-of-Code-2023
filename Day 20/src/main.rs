use std::{fs, path::Path, collections::{HashMap, VecDeque}};

// use regex::Regex;
const FLIPFLOP: usize = 0;
const CONJUNCTION: usize = 1;

#[derive(Clone)]
struct Pulse {
    high: bool,
    target: String,
    src: String
}

impl Pulse {
    fn new(high: bool, target: &str, src: &str) -> Self {
        Pulse { high, target: target.to_owned(), src: src.to_owned() }
    }
}

struct Module<'a> {
    predecessors: Vec<(&'a str, bool)>,
    state: bool,
    name: &'a str,
    targets: Vec<&'a str>,
    module_type: usize
}

impl<'a> Module<'a> {
    fn handle_pulse(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        match self.module_type {
            CONJUNCTION => self.handle_conjunction(pulse),
            FLIPFLOP => self.handle_flipflop(pulse),
            _ => panic!()
        }
    }
    
    fn handle_flipflop(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        if pulse.high {
            vec![]
        } else {
            self.state = !self.state;
            self.targets.iter().map(|t| Pulse::new(self.state, t, self.name)).collect()
        }
    }

    fn handle_conjunction(&mut self, pulse: &Pulse) -> Vec<Pulse> {
        self.predecessors.iter_mut().find(|o| o.0 == pulse.src).unwrap().1 = pulse.high;
        self.targets.iter().map(|t| Pulse::new(
            !self.predecessors.iter().all(|i| i.1),
            t,
            self.name
        )).collect()
    }

    fn add_predecessor(&mut self, pred: &'a str) {
        let temp = pred.clone();
        self.predecessors.push((temp, false));
    }

    fn reset_state(&mut self) {
        self.state = false;
        self.predecessors.iter_mut().for_each(|i| i.1 = false);
    }

    fn new(name: &'a str, targets: Vec<&'a str>, module_type: usize) -> Self {
        Module { predecessors: vec![], state: false, name, targets, module_type }
    }
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let mut broadcast = vec![];
    let mut modules: HashMap<&str, Module> = HashMap::new();

    data.split("\n").for_each(|r| {
        let row = r.split(" -> ").collect::<Vec<_>>();
        match row[0].chars().rev().last().unwrap() {
            'b' => {
                row[1].split(", ").for_each(|t| broadcast.push(t));
            },
            '%' => {
                let name = row[0].strip_prefix("%").unwrap();
                modules.insert(name, Module::new(name, row[1].split(", ").collect(), FLIPFLOP));
            },
            '&' => {
                let name = row[0].strip_prefix("&").unwrap();
                modules.insert(name, Module::new(name, row[1].split(", ").collect(), CONJUNCTION));
            }
            _ => {}
        }
    });

    let mut temp = vec![];

    modules.iter().for_each(|(_,v)| v.targets.iter().for_each(|t| {
        temp.push(((*t).clone(), v.name));
    }));

    temp.iter().for_each(|(t, v)| {
        if modules.contains_key(*t) {
            let target: &mut Module<'_> = modules.get_mut(*t).unwrap();
            target.add_predecessor(v);
        }
    });

    broadcast.iter().for_each(|t| {
        let target: &mut Module<'_> = modules.get_mut(*t).unwrap();
        target.add_predecessor("broadcast");
    });

    let mut low = 0;
    let mut high = 0;

    for _ in 0..1000 {
        let mut queue = VecDeque::new();
        low += 1; // Button
        broadcast.iter().for_each(|t| {
            queue.push_back(Pulse::new(false, t, "broadcast"));
            low += 1;
        });
        while !queue.is_empty() {
            let next = queue.pop_front().unwrap();
            if modules.contains_key(next.target.as_str()) {
                let res = modules.get_mut(next.target.as_str()).unwrap().handle_pulse(&next);
                res.into_iter().for_each(|p| {
                    if p.high {high += 1} else {low += 1}
                    queue.push_back(p);
                })
            }
        }
    }

    modules.iter_mut().for_each(|(_,v)| v.reset_state());

    let mut iteration = 0;
    let mut helper = modules.get("jz").unwrap().predecessors.iter().map(|l| (l.0.to_owned(), 0)).collect::<HashMap<_,u64>>();

    while helper.iter().any(|(_,i)| *i == 0) {
        let mut queue = VecDeque::new();
        low += 1; // Button
        broadcast.iter().for_each(|t| {
            queue.push_back(Pulse::new(false, t, "broadcast"));
        });
        iteration += 1;
        while !queue.is_empty() {
            let next = queue.pop_front().unwrap();
            if helper.contains_key(next.src.as_str()) && next.high {
                if *helper.get(next.src.as_str()).unwrap() == 0 {
                    helper.insert(next.src.clone(), iteration);
                }
            }
            if modules.contains_key(next.target.as_str()) {
                let res = modules.get_mut(next.target.as_str()).unwrap().handle_pulse(&next);
                res.into_iter().for_each(|p| {
                    queue.push_back(p);
                })
            }
        }
    }

    let part1 = low*high;
    let part2 = helper.into_iter().map(|(_,v)| v).reduce(|a,b| a*b).unwrap();

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

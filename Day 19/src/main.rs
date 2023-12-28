use std::{fs, path::Path, collections::{HashMap, VecDeque}};

use regex::Regex;

const IN: &str = "in";

#[derive(Clone)]
struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32
}

impl Part {
    fn sum(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}

struct Test<'a> {
    dim: char,
    test: char,
    num: u32,
    success: &'a str
}

impl Test<'_> {
    fn applies(&self, part: &Part) -> bool {
        let cmp = match self.dim {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => panic!()
        };
        if self.test == '<' {
            cmp < self.num
        } else {
            cmp > self.num
        }
    }

    fn ranges(&self, range: &Range) -> (Option<Range>, Option<Range>) {
        match self.dim {
            'x' => {
                if self.test == '<' {
                    if range.x.0 >= self.num {
                        return (None, Some(range.clone()));
                    } else if range.x.1 < self.num {
                        return (Some(range.clone()), None);
                    }
                    let (mut matching, mut other) = (range.clone(), range.clone());
                    matching.x.1 = self.num - 1;
                    other.x.0 = matching.x.1 + 1;
                    return (Some(matching), Some(other));
                } else {
                    if range.x.1 <= self.num {
                        return (None, Some(range.clone()));
                    } else if range.x.0 > self.num {
                        return (Some(range.clone()), None);
                    }
                    let (mut matching, mut other) = (range.clone(), range.clone());
                    matching.x.0 = self.num + 1;
                    other.x.1 = matching.x.0 - 1;
                    return (Some(matching), Some(other));
                }
            },
            'm' => {
                if self.test == '<' {
                    if range.m.0 >= self.num {
                        return (None, Some(range.clone()));
                    } else if range.m.1 < self.num {
                        return (Some(range.clone()), None);
                    }
                    let (mut matching, mut other) = (range.clone(), range.clone());
                    matching.m.1 = self.num - 1;
                    other.m.0 = matching.m.1 + 1;
                    return (Some(matching), Some(other));
                } else {
                    if range.m.1 <= self.num {
                        return (None, Some(range.clone()));
                    } else if range.m.0 > self.num {
                        return (Some(range.clone()), None);
                    }
                    let (mut matching, mut other) = (range.clone(), range.clone());
                    matching.m.0 = self.num + 1;
                    other.m.1 = matching.m.0 - 1;
                    return (Some(matching), Some(other));
                }
            },
            'a' => {
                if self.test == '<' {
                    if range.a.0 >= self.num {
                        return (None, Some(range.clone()));
                    } else if range.a.1 < self.num {
                        return (Some(range.clone()), None);
                    }
                    let (mut matching, mut other) = (range.clone(), range.clone());
                    matching.a.1 = self.num - 1;
                    other.a.0 = matching.a.1 + 1;
                    return (Some(matching), Some(other));
                } else {
                    if range.a.1 <= self.num {
                        return (None, Some(range.clone()));
                    } else if range.a.0 > self.num {
                        return (Some(range.clone()), None);
                    }
                    let (mut matching, mut other) = (range.clone(), range.clone());
                    matching.a.0 = self.num + 1;
                    other.a.1 = matching.a.0 - 1;
                    return (Some(matching), Some(other));
                }
            },
            's' => {
                if self.test == '<' {
                    if range.s.0 >= self.num {
                        return (None, Some(range.clone()));
                    } else if range.s.1 < self.num {
                        return (Some(range.clone()), None);
                    }
                    let (mut matching, mut other) = (range.clone(), range.clone());
                    matching.s.1 = self.num - 1;
                    other.s.0 = matching.s.1 + 1;
                    return (Some(matching), Some(other));
                } else {
                    if range.s.1 <= self.num {
                        return (None, Some(range.clone()));
                    } else if range.s.0 > self.num {
                        return (Some(range.clone()), None);
                    }
                    let (mut matching, mut other) = (range.clone(), range.clone());
                    matching.s.0 = self.num + 1;
                    other.s.1 = matching.s.0 - 1;
                    return (Some(matching), Some(other));
                }
            },
            _ => panic!()
        }
    }
}

struct Rule<'a> {
    // dzh{m>2601:rfn,a>3285:js,m<2527:scz,lt}
    name: &'a str,
    tests: Vec<Test<'a>>,
    default: &'a str
}

impl Rule<'_> {
    fn next(&self, part: &Part) -> &str {
        let matching = self.tests.iter().find(|t| t.applies(part));
        if matching.is_some() {
            matching.unwrap().success
        } else {
            self.default
        }
    }
}

#[derive(Clone)]
struct Range {
    x: (u32, u32),
    m: (u32, u32),
    a: (u32, u32),
    s: (u32, u32)
}

impl Range {
    fn possibilities(&self) -> u128 {
        (self.x.1 - self.x.0 + 1) as u128
        *(self.m.1 - self.m.0 + 1) as u128
        *(self.a.1 - self.a.0 + 1) as u128
        *(self.s.1 - self.s.0 + 1) as u128
    }
}

fn parse_rule(input: &str) -> Rule {
    // dzh{m>2601:rfn,a>3285:js,m<2527:scz,lt}
    let steps = Regex::new(r"([a-z]+)([><])(\d+):([a-zRA]+)").unwrap();
    let other = Regex::new(r"([a-z]+).*,([a-zAR]+)\}").unwrap();
    let other = other.captures(input).unwrap();
    let steps = steps.captures_iter(input).map(|t| {
        Test { dim: t.get(1).unwrap().as_str().chars().last().unwrap(), test: t.get(2).unwrap().as_str().chars().last().unwrap(), num: t.get(3).unwrap().as_str().parse::<u32>().unwrap(), success: t.get(4).unwrap().as_str()}
    }).collect::<Vec<_>>();
    Rule { name: other.get(1).unwrap().as_str(), tests: steps, default: other.get(2).unwrap().as_str() }
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let [c, p] = data.split("\n\n").collect::<Vec<_>>()[..] else {panic!()};

    let point_parser = Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
    let parts = p.split("\n").map(|s| {
        point_parser.captures(s).unwrap().extract::<4>().1
    }).map(|r| Part{x: r[0].parse::<u32>().unwrap(), m: r[1].parse::<u32>().unwrap(), a: r[2].parse::<u32>().unwrap(), s: r[3].parse::<u32>().unwrap()}).collect::<Vec<_>>();
    
    let mut instructions: HashMap<&str, Rule> = HashMap::new();
    c.split("\n").for_each(|r| {
        let rule = parse_rule(r);
        instructions.insert(rule.name.clone(), rule);
    });

    let part1 = parts.iter().filter(|part| {
        let mut next = IN;
        while next != "A" && next != "R" {
            let rule = instructions.get(next).unwrap();
            next = rule.next(part);
        }
        if next == "A" {true}
        else {false}
    }).map(|p| p.sum()).reduce(|a,b| a+b).unwrap();

    let mut valid = vec![];
    let mut queue = VecDeque::new();
    queue.push_back(("in", Range{ x: (1, 4000), m: (1, 4000), a: (1, 4000), s: (1, 4000)}));
    'outer: while !queue.is_empty() {
        let (rule, range) = queue.pop_front().unwrap();
        match rule {
            "A" => valid.push(range),
            "R" => {}
            _ => {
                let rule = instructions.get(rule).unwrap();
                let mut working_range = range.clone();
                for test in rule.tests.iter() {
                    let (this, next) = test.ranges(&working_range);
                    if this.is_some() {
                        queue.push_back((test.success, this.unwrap()));
                    }
                    if next.is_some() {
                        working_range = next.unwrap();
                    } else {
                        continue 'outer
                    }
                }
                queue.push_back((rule.default, working_range));
            }
        }
    }
    let part2 = valid.iter().map(|r| r.possibilities()).reduce(|a,b| a+b).unwrap();

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}

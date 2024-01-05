use std::{fs, path::Path};

use regex::Regex;

const TESTMIN: i128 = 200000000000000;
const TESTMAX: i128 = 400000000000000;

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
struct Hailstone {
    px: i128,
    py: i128,
    pz: i128,
    vx: i128,
    vy: i128,
    vz: i128
}

impl Hailstone {
    fn new(arg: [i128; 6]) -> Self{
        Hailstone { px: arg[0], py: arg[1], pz: arg[2], vx: arg[3], vy: arg[4], vz: arg[5] }
    }

    fn intersect(&self, other: &Self) -> (i128, i128) {
        // algo copied from https://stackoverflow.com/questions/2931573/determining-if-two-rays-intersect
        let dx = other.px - self.px;
        let dy = other.py - self.py;
        let det = other.vx * self.vy - other.vy * self.vx;
        if det != 0 {
            let u = (dy * other.vx - dx * other.vy) as f64 / det as f64;
            let v = (dy * self.vx - dx * self.vy) as f64 / det as f64;
            if u > 0.0 && v > 0.0 {
                let resx = self.px + self.vx * u as i128;
                let resy = self.py + self.vy * u as i128;
                return (resx, resy);
            }
        }
        (0,0)
    }
}

fn main() {
    let data = fs::read_to_string(Path::new("src/input.txt"))
        .expect("Should have been able to read the file");

    let re = Regex::new(r"(\d+),\s+(\d+),\s+(\d+)\s+@\s+(-?\d+),\s+(-?\d+),\s+(-?\d+)").unwrap();

    let hailstones = re.captures_iter(&data).map(|r| r.extract::<6>().1.map(|d| d.parse::<i128>().unwrap())).map(|s| Hailstone::new(s)).collect::<Vec<_>>();

    let mut part1 = 0;

    for i in 0..hailstones.len() {
        for j in i+1..hailstones.len() {
            let intersection = hailstones[i].intersect(&hailstones[j]);
            if TESTMIN <= intersection.0 && TESTMAX >= intersection.0 && TESTMIN <= intersection.1 && TESTMAX >= intersection.1 {part1 += 1}
        }
    }

    // let dims = vec!["u", "v", "w"];

    // for i in 0..3 {
    //     println!("{} + {} * ({}) = x + {} * A", hailstones[i].px, dims[i], hailstones[i].vx, dims[i]);
    //     println!("{} + {} * ({}) = y + {} * B", hailstones[i].py, dims[i], hailstones[i].vy, dims[i]);
    //     println!("{} + {} * ({}) = z + {} * C", hailstones[i].pz, dims[i], hailstones[i].vz, dims[i]);
    // }

    // entered numbers from print statements above here to find solution: https://quickmath.com/webMathematica3/quickmath/equations/solve/advanced.jsp
    let x: u64 = 108375683349444;
    let y: u64 = 289502736377988;
    let z: u64 = 220656145109505;
    
    let part2 = x + y + z;

    println!("Part 1: {},\nPart 2: {}", part1, part2);
}
use std::collections::HashMap;
use std::hash::Hasher;
use std::hash::Hash;
use std::collections::HashSet;
use std::io::Read;
use std::fs::OpenOptions;
use std::io::BufReader;

struct Line {
    points: HashMap<Point, u64>,
}

#[derive(Debug, Clone, Eq, Hash)]
struct Point(i64, i64);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        if self.0 == other.0 && self.1 == other.1 {
            true
        } else {
            false
        }
    }
}

// impl Hash for Point {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         self.0.hash(state);
//         self.1.hash(state);
//     }
// }


impl Line {
    fn new(instr: &str) -> Self {
        let mut points = HashMap::new();

        let instr = instr.split(',');
        let mut last_x = 0;
        let mut last_y = 0;
        let mut steps = 0;

        for l in instr {
            if l.is_empty() {
                continue
            }
            match &l[0..1] {
                "R" => {
                    let r : i64 = l[1..].parse().unwrap();
                    for i in last_x..last_x+r {
                        points.insert(Point(i, last_y), steps);
                        steps += 1;
                    }
                    last_x += r;
                },
                "L" => {
                    let r : i64 = l[1..].parse().unwrap();
                    for i in (last_x-r+1..=last_x).rev() {
                        points.insert(Point(i, last_y), steps);
                        steps += 1;
                    }
                    last_x -= r;
                },
                "U" => {
                    let r : i64 = l[1..].parse().unwrap();
                    for i in (last_y-r+1..=last_y).rev() {
                        points.insert(Point(last_x, i), steps);
                        steps += 1;
                    }
                    last_y -= r;
                },
                "D" => {
                    let r : i64 = l[1..].parse().unwrap();
                    for i in last_y..last_y+r {
                        points.insert(Point(last_x, i), steps);
                        steps += 1;
                    }
                    last_y += r;
                },
                _ => panic!("oops: {:?}", &l[..0]),
            }
        }
        points.remove(&Point(0, 0));

        Self {
            points,
        }
    }

    fn run(&self, other: &Line) -> u64 {
        // let mut map = map.clone();
        let points1 : HashSet<Point> = self.points.iter().map(|x| x.0.clone()).collect();
        let points2 : HashSet<Point> = other.points.iter().map(|x| x.0.clone()).collect();
        let intersects = points1.intersection(&points2);

        intersects
            .map(|x| self.points.get(&x).unwrap() + other.points.get(&x).unwrap())
            .min()
            .unwrap()
    }
}

fn main() {
    let f = OpenOptions::new().read(true).open("input").unwrap();
    let mut buf = BufReader::new(f);
    let mut str = "".to_owned();
    match buf.read_to_string(&mut str) {
        Ok(n) => println!("{} n bytes read from input", n),
        Err(e) => {
            println!("Error while reading input: {:?}", e);
            return
        }
    }
    let str : Vec<&str> = str.split('\n').collect();
    let mut lines = Vec::new();

    for p in str {
        lines.push(Line::new(p));
    }

    // let mut distances = HashMap::new();
    let distances = lines[0].run(&lines[1]);
    dbg!(&distances);
    // println!("distance: {:?}", distances.into_iter().min().unwrap());

}

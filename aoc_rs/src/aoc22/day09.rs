static DAY: usize = 09;

use std::collections::HashSet;
use std::ops::AddAssign;

pub fn input() -> String {
    crate::aoc::input_raw(super::YEAR, DAY)
        //.lines()
        //.map(|ls| ls.parse::<_>().unwrap())
        //.collect()
}
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Coord { x: i32, y: i32 }
impl AddAssign for Coord {
    fn add_assign(&mut self, other: Self) {
        *self = Self { x: self.x+other.x, y: self.y+other.y }
    }
}
impl Coord {
    pub fn new(x:i32, y:i32) -> Self { Self { x, y } }
}

pub fn part1(inputs: &str) -> u32 {
    let mut h = Coord::new(0,0);
    let mut t = Coord::new(0,0);
    let mut pos = HashSet::new();

    inputs.lines()
    .for_each(|line| {
        let (d, l) = line.split_at(1);
        for _ in 0..l.trim().parse::<usize>().unwrap() {
            h += match d {
                "U" => Coord::new( 0, 1),
                "D" => Coord::new( 0,-1),
                "R" => Coord::new( 1, 0),
                "L" => Coord::new(-1, 0),
                _ => unreachable!()
            };
            if h.x - t.x > 1 {
                t.x += 1;
                if h.y - t.y > 0 {
                    t.y += 1;
                }
                if t.y - h.y > 0 {
                    t.y -= 1;
                }
            }
            if t.x - h.x > 1 {
                t.x -= 1;
                if h.y - t.y > 0 {
                    t.y += 1;
                }
                if t.y - h.y > 0 {
                    t.y -= 1;
                }
            }
            if h.y - t.y > 1 {
                t.y += 1;
                if h.x - t.x > 0 {
                    t.x += 1;
                }
                if t.x - h.x > 0 {
                    t.x -= 1;
                }
            }
            if t.y - h.y > 1 {
                t.y -= 1;
                if h.x - t.x > 0 {
                    t.x += 1;
                }
                if t.x - h.x > 0 {
                    t.x -= 1;
                }
            }
            pos.insert(t);
        }
    });
    pos.len() as u32
}

pub fn part2(inputs: &str) -> u32 {
    let mut knots = vec![Coord::new(0,0); 10];
    let mut pos = HashSet::new();

    inputs.lines()
    .for_each(|line| {
        let (d, l) = line.split_at(1);
        for _ in 0..l.trim().parse::<usize>().unwrap() {
            knots[0] += match d {
                "U" => Coord::new( 0, 1),
                "D" => Coord::new( 0,-1),
                "R" => Coord::new( 1, 0),
                "L" => Coord::new(-1, 0),
                _ => unreachable!()
            };
            for i in 1..10 {
                if knots[i-1].x - knots[i].x > 1 {
                    knots[i].x += 1;
                    if knots[i-1].y - knots[i].y > 0 {
                        knots[i].y += 1;
                    }
                    if knots[i].y - knots[i-1].y > 0 {
                        knots[i].y -= 1;
                    }
                }
                if knots[i].x - knots[i-1].x > 1 {
                    knots[i].x -= 1;
                    if knots[i-1].y - knots[i].y > 0 {
                        knots[i].y += 1;
                    }
                    if knots[i].y - knots[i-1].y > 0 {
                        knots[i].y -= 1;
                    }
                }
                if knots[i-1].y - knots[i].y > 1 {
                    knots[i].y += 1;
                    if knots[i-1].x - knots[i].x > 0 {
                        knots[i].x += 1;
                    }
                    if knots[i].x - knots[i-1].x > 0 {
                        knots[i].x -= 1;
                    }
                }
                if knots[i].y - knots[i-1].y > 1 {
                    knots[i].y -= 1;
                    if knots[i-1].x - knots[i].x > 0 {
                        knots[i].x += 1;
                    }
                    if knots[i].x - knots[i-1].x > 0 {
                        knots[i].x -= 1;
                    }
                }
            }
            pos.insert(knots[9]);
        }
    });
    pos.len() as u32
}

#[test]
fn test_day9_part1() {
    let inputs = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    assert_eq!(part1(inputs), 13);
}

#[test]
fn test_day9_part2() {
    let inputs = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    assert_eq!(part2(inputs), 1);
    let inputs = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
    assert_eq!(part2(inputs), 36);
}

#[allow(unused)]
pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    let res = part1(&inputs);
    print!("Day {} part 1: ", DAY);
    println!("{}", res);
    println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    let res = part2(&inputs);
    print!("Day {} part 2: ", DAY);
    println!("{}", res);
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}



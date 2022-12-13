static DAY: usize = 13;

use std::{str::{Chars}, fmt::Debug, cmp::Ordering};

pub fn input() -> String {
    crate::aoc::input_raw(13)
}


#[derive(Clone)]
enum Packet {
    List(Vec<Box<Packet>>),
    Val(usize),
}
impl Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Packet::List(x) => write!(f, "{:?}", x),
            Packet::Val(x) => write!(f, "{}", x)
        }
    }
}
impl Packet {
    pub fn new(s: &mut Chars) -> Self {
        let mut content = Vec::new();
        let mut val = 0;
        let mut tok = false;
        loop {
            match s.next() {
                Some('[') => content.push(Box::new(Packet::new(s))),
                Some(']') | None => {
                    if tok {
                        content.push(Box::new(Packet::Val(val)));
                    }
                    break
                },
                Some(',') => {
                    if tok {
                        content.push(Box::new(Packet::Val(val)));
                    }
                    val = 0;
                    tok = false;
                },
                Some(c) => {
                    tok = true;
                    val = val * 10 + c.to_digit(10).unwrap() as usize;
                }
            };
        }
        return Packet::List(content)
    }

    pub fn comp(&self, other: &Packet) -> bool {
        self.inner_comp(other) != Ordering::Less
    }
    pub fn inner_comp(&self, other: &Packet) -> Ordering {
        match (self, other) {
            (Packet::List(pl), Packet::List(pr)) => {
                for i in 0..pr.len() {
                    // Left side ran out of items
                    if i >= pl.len() {
                        return Ordering::Greater;
                    }
                    // If right evaluates to larger or smaller, return result,
                    // if undecided, continue
                    match pl[i].inner_comp(&pr[i]) {
                        Ordering::Equal => (),
                        c => return c
                    };
                }
                // Right ran out of items
                if pr.len() < pl.len() {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            },
            (Packet::Val(l), Packet::Val(r)) => {
                if r > l {
                    Ordering::Greater
                } else if  r < l {
                    Ordering::Less
                } else {
                    Ordering::Equal
                }
            },
            (Packet::List(_pl), Packet::Val(r)) => {
                self.inner_comp(&Packet::List(vec![Box::new(Packet::Val(*r))]))
            },
            (Packet::Val(l), Packet::List(_pr)) => {
                Packet::List(vec![Box::new(Packet::Val(*l))]).inner_comp(other)
            }
        }
    }
}

fn parse_packet(packet: &str) -> Packet {
    Packet::new(&mut packet.trim().chars())
}
fn parse_pair(pair: &str) -> (Packet, Packet) {
    let mut lines = pair.split("\n");
    (

        parse_packet(&mut lines.next().unwrap().trim()),
        parse_packet(&mut lines.next().unwrap().trim()),
    )
}

pub fn part1(inputs: &str) -> u32 {
    inputs.split("\n\n")
    .map(parse_pair)
    .enumerate()
    .filter(|(_, (pl,pr))| {pl.comp(pr)})
    .map(|(i, (_,_))| {i as u32+1})
    .sum::<u32>()
}

pub fn part2(inputs: &str) -> u32 {
    let mut packets = inputs.split("\n\n")
    .flat_map(|pair| pair.split("\n"))
    .filter(|s| !s.trim().is_empty())
    .map(parse_packet)
    .collect::<Vec<_>>();
    let p0 = Packet::new(&mut "[[2]]".chars());
    let p1 = Packet::new(&mut "[[6]]".chars());
    for (i,p) in packets.clone().into_iter().enumerate() {
        println!("{}: {:?}", i, p);
    }
    packets.push(p0.clone());
    packets.push(p1.clone());
    packets.sort_by(|l,r| r.inner_comp(l));
    let i0 = packets.binary_search_by(|probe| p0.inner_comp(probe)).unwrap();
    let i1 = packets.binary_search_by(|probe| p1.inner_comp(probe)).unwrap();
    ((i0+1) * (i1+1)) as u32
}

#[test]
fn test_day13_part1() {
    let inputs = "[1,1,3,1,1]
    [1,1,5,1,1]

    [[1],[2,3,4]]
    [[1],4]

    [9]
    [[8,7,6]]

    [[4,4],4,4]
    [[4,4],4,4,4]

    [7,7,7,7]
    [7,7,7]

    []
    [3]

    [[[]]]
    [[]]

    [1,[2,[3,[4,[5,6,7]]]],8,9]
    [1,[2,[3,[4,[5,6,0]]]],8,9]";
    let pairs = inputs.split("\n\n").map(parse_pair).collect::<Vec<_>>();
    assert!(pairs[0].0.comp(&pairs[0].1));
    assert!(pairs[1].0.comp(&pairs[1].1));
    assert!(!pairs[2].0.comp(&pairs[2].1));
    assert!(pairs[3].0.comp(&pairs[3].1));
    assert!(!pairs[4].0.comp(&pairs[4].1));
    assert!(pairs[5].0.comp(&pairs[5].1));
    assert!(!pairs[6].0.comp(&pairs[6].1));
    assert!(pairs[6].0.inner_comp(&pairs[6].0) == Ordering::Equal);

    assert_eq!(part1(inputs), 13);
}

#[test]
fn test_day13_part2() {
    let inputs = "[1,1,3,1,1]
    [1,1,5,1,1]

    [[1],[2,3,4]]
    [[1],4]

    [9]
    [[8,7,6]]

    [[4,4],4,4]
    [[4,4],4,4,4]

    [7,7,7,7]
    [7,7,7]

    []
    [3]

    [[[]]]
    [[]]

    [1,[2,[3,[4,[5,6,7]]]],8,9]
    [1,[2,[3,[4,[5,6,0]]]],8,9]";
    assert_eq!(part2(inputs), 140);
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



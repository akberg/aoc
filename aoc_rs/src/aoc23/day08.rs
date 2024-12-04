use std::collections::HashMap;

use super::YEAR;
static DAY: usize = 08;

pub fn input() -> String {
    crate::aoc::input_raw(YEAR, DAY)
}

pub fn part1(inputs: &str) -> u32 {
    let mut lines = inputs.lines();
    let instr = lines.next().unwrap().chars().collect::<Vec<_>>();
    let _whitespace = lines.next();

    let mut graph = HashMap::new();
    for line in lines {
        let line = line.trim();
        let node = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];
        graph.insert(node, (left, right));
    }

    let mut node = graph.get("AAA").unwrap();

    for i in 0.. {
        let step = i % instr.len();
        let next = if instr[step] == 'L' { node.0 } else { node.1 };
        if next == "ZZZ" {
            return (i + 1) as u32;
        }
        node = graph.get(next).unwrap();
    }
    0
}

type Node = (char, char, char);

enum Target { Zend, Pin(Node)}

fn find_zs(
    root: &Node,
    graph: &HashMap<Node, (Node, Node)>,
    instr: &Vec<char>,
    target: Target
) -> u64 {
    let mut node = graph.get(root).unwrap();
    let found = 0;
    for i in 0.. {
        let step = i % instr.len();
        let next = if instr[step] == 'L' { node.0 } else { node.1 };
        if match target { Target::Zend => next.2 == 'Z', Target::Pin(n) => next == n } {
            return (i+1) as u64;
        }
        node = graph.get(&next).unwrap();
    }
    0
}

fn factor(mut num: u64) -> Vec<u64> {
    let mut ret = Vec::new();
    for i in 2..(num as f64).sqrt() as u64 {
        while num % i == 0 {
            ret.push(i);
            num /= i;
        }
    }
    ret.push(num);
    ret
}

/// Performance: Naive solution not possible.
/// Fair assumption: Every root only hits one child with Z (tested up to 10M steps)
pub fn part2(inputs: &str) -> u64 {
    let mut lines = inputs.lines();
    let instr = lines.next().unwrap().chars().collect::<Vec<_>>();
    let _whitespace = lines.next();

    let mut graph = HashMap::new();
    let mut nodes = Vec::new();
    for line in lines {
        let line = line.trim().chars().collect::<Vec<_>>();
        let node = (line[0], line[1], line[2]);
        let left = (line[7], line[8], line[9]);
        let right = (line[12], line[13], line[14]);
        graph.insert(node, (left, right));
        if node.2 == 'A' {
            nodes.push(node);
        }
    }

    let mut steps = Vec::new();

    for root in nodes {
        // println!("{:?}", root);
        let s = find_zs(&root, &graph, &instr, Target::Zend);
        steps.push(factor(s));
        // println!("{} {:?}", s, factor(s));
    }
    // TODO: CRT
    let lcm = steps[0][1];
    steps.into_iter().fold(lcm, |acc, n| {println!("{}", acc); acc*n[0]})
}

#[test]
fn test_day8_part1() {
    let inputs = "RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)";
    assert_eq!(2, part1(inputs));
    let inputs = "LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)";
    assert_eq!(6, part1(inputs));
}

#[test]
fn test_day8_part2() {
    let inputs = "LR

    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)";
    assert_eq!(6, part2(inputs));
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

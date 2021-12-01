
extern crate regex;
use std::collections::{HashSet, HashMap, VecDeque};
use regex::Regex;

type BagMap = HashMap<String, HashSet<(String, usize)>>;

#[allow(unused)]
pub fn input() -> (BagMap, BagMap) {
    let mut containers1 = HashMap::new();
    let mut containers2 = HashMap::new();
    let ifile = crate::aoc::input_raw(20, 7);
    for line in ifile.lines() {
        parse_line(&mut containers1, &mut containers2, line);
    }
    (containers1, containers2)
}

#[allow(unused)]
fn parse_line(map1: &mut BagMap, map2: &mut BagMap, line: &str) {

    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<num>[0-9]) (?P<bag>[a-z]+ [a-z]+)").unwrap();
    }
    let mut split = line.split(" bags contain ");
    let key = split.next().unwrap().to_string();

    // Add key if it does not exist (avoid keyerror in search)
    if !map1.contains_key(&key) { map1.insert(key.clone(), HashSet::new()); }
    let container2 = map2.entry(key.clone()).or_insert(HashSet::new());

    let line = split.next().unwrap();
    if line != "no other bags." {
        for e in line.split(",") {
            let cap = RE.captures(e).unwrap();
            let container1 = map1.entry(cap["bag"].to_string()).or_insert(HashSet::new());
            container1.insert((key.clone(), cap["num"].parse::<usize>().unwrap()));
            container2.insert((cap["bag"].to_string(), cap["num"].parse::<usize>().unwrap()));
        }
    }
}

#[allow(unused)]
pub fn part1(inputs: &BagMap) -> usize {
    // Visited nodes when parsing the tree from "shiny gold" bags
    let mut visited: HashSet<String> = HashSet::with_capacity(inputs.len());
    let mut queue = VecDeque::new();

    queue.push_back(&inputs["shiny gold"]);
    while !queue.is_empty() {
        for e in queue.pop_front().unwrap() {
            if visited.insert(e.0.clone()) {
                queue.push_back(&inputs[&e.0]);
            }
        }
    }
    visited.len()
}

#[allow(unused)]
pub fn part2(inputs: &BagMap) -> usize {
    // Visited nodes when parsing the tree from "shiny gold" bags
    let mut visited: Vec<(String, usize)> = Vec::new();
    let mut queue = VecDeque::new();

    queue.push_back((&inputs["shiny gold"], 1));
    while !queue.is_empty() {
        let (children, size) = queue.pop_front().unwrap();
        for e in children {
            visited.push((e.0.clone(), e.1*size));
            queue.push_back((&inputs[&e.0], e.1*size));
        }
    }
    visited.iter()
        .map(|e| e.1)
        .sum()
}

#[test]
fn test_day7_part1() {
    let inputs = String::from("light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
        bright white bags contain 1 shiny gold bag.\n\
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
        faded blue bags contain no other bags.\n\
        dotted black bags contain no other bags.");
    let mut containers: BagMap = HashMap::new();
    let mut dummy: BagMap = HashMap::new();
    for line in inputs.lines() {
        parse_line(&mut containers, &mut dummy, line);
    }
    assert_eq!(4, part1(&containers));
}

#[test]
fn test_day7_part2() {
    let inputs = String::from("light red bags contain 1 bright white bag, 2 muted yellow bags.\n\
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.\n\
        bright white bags contain 1 shiny gold bag.\n\
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.\n\
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.\n\
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.\n\
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.\n\
        faded blue bags contain no other bags.\n\
        dotted black bags contain no other bags.");
    let mut containers: BagMap = HashMap::new();
    let mut dummy: BagMap = HashMap::new();
    for line in inputs.lines() {
        parse_line(&mut dummy, &mut containers, line);
    }
    assert_eq!(32, part2(&containers));
    let inputs = String::from("shiny gold bags contain 2 dark red bags.\n\
    dark red bags contain 2 dark orange bags.\n\
    dark orange bags contain 2 dark yellow bags.\n\
    dark yellow bags contain 2 dark green bags.\n\
    dark green bags contain 2 dark blue bags.\n\
    dark blue bags contain 2 dark violet bags.\n\
    dark violet bags contain no other bags.");
    let mut containers: BagMap = HashMap::new();
    let mut dummy: BagMap = HashMap::new();
    for line in inputs.lines() {
        parse_line(&mut dummy, &mut containers, line);
    }
    assert_eq!(126, part2(&containers));
}

#[test]
fn run_day7() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 7 part 1: ");
    println!("{} - in {:?}", part1(&inputs.0), pt_start.elapsed().unwrap());
    print!("Day 7 part 2: ");
    let pt_start = SystemTime::now();
    println!("{} - in {:?}", part2(&inputs.1), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}

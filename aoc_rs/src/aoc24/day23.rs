use std::collections::{HashMap, HashSet};

/// Keywords: Graph, Cycles
use super::YEAR;
static DAY: usize = 23;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Node {
    id: [char; 2],
}

fn parse_input(inputs: &str) -> HashMap<Node, Vec<Node>> {
    let mut edges = HashMap::new();
    inputs.lines().for_each(|line| {
        let (a, b) = line.trim().split_once("-").unwrap();
        let a = Node {
            id: a.chars().take(2).collect::<Vec<_>>().try_into().unwrap(),
        };
        let b = Node {
            id: b.chars().take(2).collect::<Vec<_>>().try_into().unwrap(),
        };
        edges.entry(a).or_insert(Vec::new()).push(b);
        edges.entry(b).or_insert(Vec::new()).push(a);
    });
    edges
}

fn input() -> HashMap<Node, Vec<Node>> {
    parse_input(&crate::aoc::input_raw(YEAR, DAY))
}

/// Return number of cycles of length `max_length` from `source`.
fn find_cycles(
    source: Node,
    current: Node,
    edges: &HashMap<Node, Vec<Node>>,
    depth: usize,
    max_depth: usize,
    visited: &mut HashSet<Node>,
    mut path: Vec<Node>
) -> usize {
    path.push(current);
    // println!("{}{:?}", " ".repeat(depth), current);
    if depth == max_depth {
        if current == source {
            println!("    {:?}", path);
        }
        (current == source) as usize
    } else {
        if visited.insert(current) {
            edges[&current]
                .iter()
                .map(|&n| {
                    find_cycles(source, n, edges, depth + 1, max_depth, visited, path.clone())
                })
                .sum()
        } else {
            0
        }
    }
}

/// For each node:
///     For each neighbour:
fn part1(inputs: &HashMap<Node, Vec<Node>>) -> usize {
    inputs
        .keys()
        .filter(|k| k.id[0] == 't')
        .map(|&k| find_cycles(k, k, inputs, 0, 3, &mut HashSet::new(), Vec::new()))
        .sum()
}

fn part2(inputs: &HashMap<Node, Vec<Node>>) -> u32 {
    todo!();
}


lazy_static::lazy_static! {
    static ref TEST_INPUTS: HashMap<Node, Vec<Node>> = parse_input("kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn");
}

#[test]
fn test_2024_day23_part1() {
    assert_eq!(part1(&TEST_INPUTS), 7);
}

#[test]
fn test_2024_day23_part2() {
    // TODO
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
    print!("{} Day {} part 1: ", YEAR, DAY);
    println!("{}", res);
    println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    let res = part2(&inputs);
    print!("{} Day {} part 2: ", YEAR, DAY);
    println!("{}", res);
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}

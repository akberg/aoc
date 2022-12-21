use std::collections::HashMap;

static DAY: usize = 21;

pub fn input() -> String {
    crate::aoc::input_raw(21)
}

fn compute_tree(tree: &HashMap<&str, &str>, root: &str) -> i64 {
    let op = tree.get(root).unwrap();
    if let Ok(x) = op.parse() {
        return x;
    }
    let mut op = op.split_ascii_whitespace();
    let lhs = compute_tree(tree, op.next().unwrap());
    let operator = op.next().unwrap();
    let rhs = compute_tree(tree, op.next().unwrap());
    match operator {
        "+" => lhs + rhs,
        "-" => lhs - rhs,
        "*" => lhs * rhs,
        "/" => lhs / rhs,
        _ => unreachable!()
    }
}
#[derive(Debug, Clone)]
enum Tree {
    Num(i64),
    X,
    Add(Box<Tree>, Box<Tree>),
    Sub(Box<Tree>, Box<Tree>),
    Mul(Box<Tree>, Box<Tree>),
    Div(Box<Tree>, Box<Tree>),
}
fn compute_eq(tree: &HashMap<&str, &str>, root: &str) -> Tree {
    if root == "humn" {
        return Tree::X;
    }
    let op = tree.get(root).unwrap();
    if let Ok(x) = op.parse() {
        return Tree::Num(x);
    }
    let mut op = op.split_ascii_whitespace();
    let lhs = compute_eq(tree, op.next().unwrap());
    let operator = op.next().unwrap();
    let rhs = compute_eq(tree, op.next().unwrap());
    match operator {
        "+" => if let (&Tree::Num(l),&Tree::Num(r))=(&lhs,&rhs) { Tree::Num(l + r) } else { Tree::Add(Box::new(lhs),Box::new(rhs))},
        "-" => if let (&Tree::Num(l),&Tree::Num(r))=(&lhs,&rhs) { Tree::Num(l - r) } else { Tree::Sub(Box::new(lhs),Box::new(rhs))},
        "*" => if let (&Tree::Num(l),&Tree::Num(r))=(&lhs,&rhs) { Tree::Num(l * r) } else { Tree::Mul(Box::new(lhs),Box::new(rhs))},
        "/" => if let (&Tree::Num(l),&Tree::Num(r))=(&lhs,&rhs) { Tree::Num(l / r) } else { Tree::Div(Box::new(lhs),Box::new(rhs))},
        _ => unreachable!()
    }
}

pub fn part1(inputs: &str) -> i64 {
    let mut monkeys = HashMap::new();
    inputs.lines().for_each(|line| {
        let (m, op) = line.trim().split_once(": ").unwrap();
        monkeys.insert(m, op);
    });
    compute_tree(&monkeys, "root")
}

pub fn part2(inputs: &str) -> i64 {
    let mut monkeys = HashMap::new();
    inputs.lines().for_each(|line| {
        let (m, op) = line.trim().split_once(": ").unwrap();
        monkeys.insert(m, op);
    });
    let mut root = monkeys.get("root").unwrap().split_ascii_whitespace();
    // Build equation tree for both sides
    let lhs = compute_eq(&monkeys, root.next().unwrap());
    let rhs = compute_eq(&monkeys, root.skip(1).next().unwrap());
    let (mut val, mut unknown) = if let Tree::Num(x) = lhs { (x, rhs) } else if let Tree::Num(x) = rhs { (x, lhs) } else { unreachable!() };

    // Until X is reached, unwrap operations from uknown side and apply inverse to known side
    loop {
        // println!("{:?} = {:?}", val, unknown);
        match unknown.clone() {
            Tree::Add(lhs, rhs) => {
                if let Tree::Num(x) = *lhs {
                    val -= x;
                    unknown = *rhs;
                }
                else if let Tree::Num(x) = *rhs {
                    val -= x;
                    unknown = *lhs;
                }
            },
            Tree::Sub(lhs, rhs) => {
                if let Tree::Num(x) = *lhs {
                    val = x - val;
                    unknown = *rhs;
                }
                else if let Tree::Num(x) = *rhs {
                    val += x;
                    unknown = *lhs;
                }
            },
            Tree::Mul(lhs, rhs) => {
                if let Tree::Num(x) = *lhs {
                    val /= x;
                    unknown = *rhs;
                }
                else if let Tree::Num(x) = *rhs {
                    val /= x;
                    unknown = *lhs;
                }
            },
            Tree::Div(lhs, rhs) => {
                if let Tree::Num(x) = *lhs {
                    val = x / val;
                    unknown = *rhs;
                }
                else if let Tree::Num(x) = *rhs {
                    val = x * val;
                    unknown = *lhs;
                }
            },
            Tree::X => {
                return val
            },
            Tree::Num(_) => unreachable!()
        }
    }
}

#[test]
fn test_day21_part1() {
    let inputs = "root: pppw + sjmn
    dbpl: 5
    cczh: sllz + lgvd
    zczc: 2
    ptdq: humn - dvpt
    dvpt: 3
    lfqf: 4
    humn: 5
    ljgn: 2
    sjmn: drzm * dbpl
    sllz: 4
    pppw: cczh / lfqf
    lgvd: ljgn * ptdq
    drzm: hmdt - zczc
    hmdt: 32";
    assert_eq!(part1(inputs), 152);
}

#[test]
fn test_day21_part2() {
    let inputs = "root: pppw + sjmn
    dbpl: 5
    cczh: sllz + lgvd
    zczc: 2
    ptdq: humn - dvpt
    dvpt: 3
    lfqf: 4
    humn: 5
    ljgn: 2
    sjmn: drzm * dbpl
    sllz: 4
    pppw: cczh / lfqf
    lgvd: ljgn * ptdq
    drzm: hmdt - zczc
    hmdt: 32";
    assert_eq!(part2(inputs), 301);
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



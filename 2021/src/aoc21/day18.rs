static DAY: i32 = 18;

#[derive(Clone, Debug, PartialEq)]
pub enum ParseTree { Node(Box<ParseTree>, Box<ParseTree>), Leaf(i32) }

#[allow(unused)]
fn add(op1: &ParseTree, op2: &ParseTree) -> ParseTree {
    ParseTree::Node(Box::new(op1.clone()), Box::new(op2.clone()))
}

fn serialize(tree: &ParseTree, vec: &mut Vec<(i32, i32)>, depth: i32) {
    use ParseTree::*;
    match tree {
        Node(l, r) => {
            serialize(l, vec, depth+1);
            serialize(r, vec, depth+1);
        },
        Leaf(x) => {
            vec.push((depth, *x));
        }
    }
}
fn deserialize(vec: &mut Vec<(i32, i32)>, depth: i32) -> ParseTree {
    if depth < vec[0].0 {
        ParseTree::Node(
            Box::new(deserialize(vec, depth+1)),
            Box::new(deserialize(vec, depth+1))
        )
    } else {
        ParseTree::Leaf(vec.remove(0).1)
    }
}

fn add_serial(op1: &Vec<(i32, i32)>, op2: &Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut ret = op1.clone().iter().map(|(d,n)| (d+1,*n)).collect::<Vec<(i32,i32)>>();
    ret.extend(op2.iter().map(|(d,n)| (d+1,*n)));
    ret
}

fn reduce_serial(num: &mut Vec<(i32, i32)>) {
    /* explode */
    let mut changed = true;
    while changed {
        changed = false;
        for i in 0..num.len() {
            let d = num[i].0;
            if d > 4 {
                let (l, r) = (num[i].1, num[i+1].1);
                if i > 0 {
                    num[i-1].1 += l;
                }
                if i+2 < num.len() {
                    num[i+2].1 += r;
                }
                num[i+1] = (d-1, 0);
                changed = true;
                num.remove(i);
                break;
            }
        }
    }

    /* split */
    for i in 0..num.len() {
        if num[i].1 > 9 {
            let (d, n) = num[i];
            let (l,r) = if n % 2 == 0 {
                (n / 2, n / 2)      // Even
            } else {
                (n / 2, n / 2 + 1)  // Odd, integer division to round down
            };
            num[i] = (d+1, r);
            num.insert(i, (d+1, l));
            changed = true;
            break;
        }
    }
    
    if changed {
        reduce_serial(num);
    }
}

fn magnitude(num: &ParseTree) -> i32 {
    if let ParseTree::Node(l, r) = num {
        3 * match **l {
            ParseTree::Leaf(x) => x,
            ParseTree::Node(_,_) => magnitude(l),
        }
        + 
        2 * match **r {
            ParseTree::Leaf(x) => x,
            ParseTree::Node(_,_) => magnitude(r),
        }
    } else { panic!() }
}

fn parse_line(line: &str) -> ParseTree {
    let mut stack = Vec::new();
    let mut sfstack = Vec::new();
    for c in line.chars() {
        match c {
            '[' => stack.push(c),
            ']' => {
                let (r, l) = (sfstack.pop().unwrap(), sfstack.pop().unwrap());
                sfstack.push(ParseTree::Node(Box::new(l), Box::new(r)));
            },
            ',' => (),
            _ => sfstack.push(ParseTree::Leaf(c.to_digit(10).unwrap() as i32)), 
        };
    }
    sfstack[0].clone()
}

pub fn input() -> Vec<ParseTree> {
    crate::aoc::input(DAY).iter().map(|line| parse_line(line)).collect::<_>()
}

/// Sum snailfish numbers and return magnitude
pub fn part1(inputs: &Vec<ParseTree>) -> i32 {
    /* Serialize all inputs */
    let inputs = inputs.iter().map(|tree| {
        let mut serial = Vec::new();
        serialize(tree, &mut serial, 0);
        serial
    }).collect::<Vec<_>>();
    /* Accumulate sum, reducing every time */
    let mut acc = inputs[0].clone();
    for line in inputs.iter().skip(1) {
        acc = add_serial(&acc, &line);
        reduce_serial(&mut acc);
    }
    magnitude(&deserialize(&mut acc, 0))
}

/// Find the highest magnitude from adding two of the snailfish numbers
pub fn part2(inputs: &Vec<ParseTree>) -> i32 {
    /* Serialize all inputs */
    let inputs = inputs.iter().map(|tree| {
        let mut serial = Vec::new();
        serialize(tree, &mut serial, 0);
        serial
    }).collect::<Vec<_>>();

    let mut mag_max = 0;
    for i in 0..inputs.len() {
        for j in 0..inputs.len() {
            if i != j {
                let mut a = add_serial(&inputs[i], &inputs[j]);
                reduce_serial(&mut a);
                let mag = magnitude(&deserialize(&mut a, 0));
                mag_max = i32::max(mag, mag_max);

                let mut a = add_serial(&inputs[j], &inputs[i]);
                reduce_serial(&mut a);
                let mag = magnitude(&deserialize(&mut a, 0));
                mag_max = i32::max(mag, mag_max);
            }
        }
    }
    mag_max
}

/* TESTS */
#[allow(unused)]
static TEST_INPUT0: &'static [&str] = &[
    "[[1,2],[[3,4],5]]", // Pair ( Pair ( 1,2 ) , )
    "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]",
    "[[[[1,1],[2,2]],[3,3]],[4,4]]",
    "[[[[3,0],[5,3]],[4,4]],[5,5]]",
    "[[[[5,0],[7,4]],[5,5]],[6,6]]",
    "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]",
];
#[allow(unused)]
static TEST_INPUT1: &'static [&str] = &[
    "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]",
    "[[[5,[2,8]],4],[5,[[9,9],0]]]",
    "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]",
    "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]",
    "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]",
    "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]",
    "[[[[5,4],[7,7]],8],[[8,3],8]]",
    "[[9,3],[[9,9],[6,[4,9]]]]",
    "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]",
    "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]",
];
#[allow(unused)]
static TEST_INPUT2: &'static [&str] = &[
    "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]",
    "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]",
    "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]",
    "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]",
    "[7,[5,[[3,8],[1,4]]]]",
    "[[2,[2,2]],[8,[8,1]]]",
    "[2,9]",
    "[1,[[[9,3],9],[[9,0],[0,7]]]]",
    "[[[5,[7,4]],7],1]",
    "[[[[4,2],2],6],[8,7]]",
];

#[test]
fn test_day18_reduce_serial() {
    let mut before = vec![(5,9), (5,8), (4,1), (3,2), (2,3), (1,4)];
    let      after = vec![(4,0), (4,9), (3,2), (2,3), (1,4)];
    reduce_serial(&mut before);
    assert_eq!(before, after);
}

#[test]
fn test_day18_serialize() {
    let mut ser = Vec::new();
    serialize(&parse_line(TEST_INPUT0[0]), &mut ser, 0);
    assert_eq!(ser, vec![(2,1), (2,2), (3,3), (3,4), (2,5)]);
    let mut ser = Vec::new();
    serialize(&parse_line(TEST_INPUT0[1]), &mut ser, 0);
    assert_eq!(ser, vec![(4,0), (4,7), (3,4), (4,7), (4,8), (4,6), (4,0), (2,8), (2,1)]);
}

#[test]
fn test_day18_deserialize() {
    let mut ser = Vec::new();
    let tree = parse_line(TEST_INPUT0[0]);
    serialize(&tree, &mut ser, 0);
    assert_eq!(tree, deserialize(&mut ser, 0));
    let mut ser = Vec::new();
    let tree = parse_line(TEST_INPUT0[1]);
    serialize(&tree, &mut ser, 0);
    assert_eq!(tree, deserialize(&mut ser, 0));
}

#[test]
fn test_day18_magnitude() {
    assert_eq!(magnitude(&parse_line(TEST_INPUT0[0])), 143);
    assert_eq!(magnitude(&parse_line(TEST_INPUT0[1])), 1384);
    assert_eq!(magnitude(&parse_line(TEST_INPUT0[2])), 445);
    assert_eq!(magnitude(&parse_line(TEST_INPUT0[3])), 791);
    assert_eq!(magnitude(&parse_line(TEST_INPUT0[4])), 1137);
    assert_eq!(magnitude(&parse_line(TEST_INPUT0[5])), 3488);
}

#[test]
fn test_day18_part1() {
    let inputs = TEST_INPUT1.iter().map(|line| parse_line(line)).collect::<Vec<_>>();
    assert_eq!(part1(&inputs), 4140);
}

#[test]
fn test_day18_part2() {
    let inputs = TEST_INPUT1.iter().map(|line| parse_line(line)).collect::<Vec<_>>();
    assert_eq!(part2(&inputs), 3993);
}


#[allow(unused)]
pub fn run() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . .");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day {} part 1: ", DAY);
    println!("{}", part1(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day {} part 2: ", DAY);
    println!("{}", part2(&inputs));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}

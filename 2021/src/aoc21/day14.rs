const DAY: i32 = 14;

use std::collections::{HashMap};

pub fn input() -> (Vec<char>, HashMap<(char, char), char>) {
    use std::io::{prelude::*, BufReader};
    let f = crate::aoc::input_file(DAY).unwrap();
    let mut f = BufReader::new(f).lines();

    let init_string = f.next().unwrap().unwrap().chars().collect::<Vec<_>>();
    let _ = f.next();

    let mut pairs = HashMap::new();
    f.for_each(|line| {
        let s = line.unwrap().chars().collect::<Vec<_>>();
        pairs.insert((s[0], s[1]), s[6]);   // "%c%c -> %c"
    });

    (init_string, pairs)
}

/// Recursively insert elements in vector until step limit is reached
/// at most once.
fn insert_elements(pairs: &HashMap<(char, char), char>, node: (char, char), step: i32, lim: i32) -> Vec<char> {
    eprintln!("[insert_elements] step: {}", step);
    if step == lim {
        return vec![node.0, node.1]
    }
    let ins = pairs[&node];
    let mut left = insert_elements(pairs, (node.0, ins), step+1, lim);
    let    right = insert_elements(pairs, (ins, node.1), step+1, lim);
    left.extend(right.iter().skip(1));
    left
}

/// Stepwise insert_elements
/// - pairs: map of char pairs to new char
/// - count: count of each pair on current step
/// - freq: character frequency
fn insert_elements_step(
    pairs: &HashMap<(char,char), char>,
    count: &HashMap<(char,char),usize>,
    freq: &mut HashMap<char, usize>
) -> HashMap<(char,char),usize> {
    // ret replaces count
    let mut ret = HashMap::new();

    for (k, v) in count {
        eprintln!("[insert_elements_step] {:?}: {}", k, v);
        // k: evaluated pair
        // c: character to be added
        // v: number of times it will be added
        let c = pairs[k];
        *freq.entry(c).or_default() += v;
        *ret.entry((k.0, c)).or_default() += v;
        *ret.entry((c, k.1)).or_default() += v;
    }
    ret
}

/// Countall paths where all small caves are visited at most once
/// BFS, but not marking large caves as visited. Needs to be recursive
pub fn part1(init: &Vec<char>, pairs: &HashMap<(char, char), char>) -> usize {
    let mut line = insert_elements(pairs, (init[0], init[1]), 0, 10);
    init[1..init.len()].windows(2).for_each(|w| {
        line.extend(insert_elements(pairs, (w[0], w[1]), 0, 10).iter().skip(1));
    });
    /* Finding least and most common element */
    let freq = line.iter().fold(HashMap::<char, usize>::new(), |mut m, &c| {
        *m.entry(c).or_default() += 1;
        m
    });
    let mut freq = freq.values().collect::<Vec<_>>();
    freq.sort();
    *freq.last().unwrap() - freq[0]
}


/// Repeat with 40 steps. Too slow, should try adding meoization
pub fn part2(init: &Vec<char>, pairs: &HashMap<(char, char), char>) -> usize {
    let mut freq = init.iter().fold(HashMap::<char,usize>::new(),
    |mut m, &c| {
        *m.entry(c).or_default() += 1;
        m
    });
    let mut count = init.windows(2).fold(HashMap::<(char,char),usize>::new(),
    |mut m, w| {
        *m.entry((w[0], w[1])).or_default() += 1;
        m
    });

    for i in 0..40 {
        eprintln!("step {}, count size: {}", i, count.len());
        count = insert_elements_step(pairs, &count, &mut freq);
        eprintln!("{:?}", freq);
    }

    let mut freq = freq.values().collect::<Vec<_>>();
    freq.sort();
    *freq.last().unwrap() - freq[0]
}

/* TESTS */
#[allow(unused)]
static TEST_INPUT_LINE: &str = "NNCB";
#[allow(unused)]
static TEST_INPUT_PAIRS: &'static [((char,char),char)] = &[
    (('C', 'H'), 'B'),
    (('H', 'H'), 'N'),
    (('C', 'B'), 'H'),
    (('N', 'H'), 'C'),
    (('H', 'B'), 'C'),
    (('H', 'C'), 'B'),
    (('H', 'N'), 'C'),
    (('N', 'N'), 'C'),
    (('B', 'H'), 'H'),
    (('N', 'C'), 'B'),
    (('N', 'B'), 'B'),
    (('B', 'N'), 'B'),
    (('B', 'B'), 'N'),
    (('B', 'C'), 'B'),
    (('C', 'C'), 'N'),
    (('C', 'N'), 'C'),
];

#[test]
fn test_day14_insert_elements() {
    let init = TEST_INPUT_LINE.chars().collect::<Vec<_>>();
    let mut pairs = HashMap::new();
    TEST_INPUT_PAIRS.iter().for_each(|&(k,v)| { pairs.insert(k, v); });
    let mut line = insert_elements(&pairs, (init[0], init[1]), 0, 1);
    init[1..init.len()].windows(2).for_each(|w| {
        line.extend(insert_elements(&pairs, (w[0], w[1]), 0, 1).iter().skip(1));
    });

    assert_eq!(line, "NCNBCHB".chars().collect::<Vec<_>>());
}

#[test]
fn test_day14_insert_elements_step() {
    let init = TEST_INPUT_LINE.chars().collect::<Vec<_>>();
    let mut pairs = HashMap::new();
    TEST_INPUT_PAIRS.iter().for_each(|&(k,v)| { pairs.insert(k, v); });
    let mut freq = init.iter().fold(HashMap::<char,usize>::new(), |mut m, &c| {
        *m.entry(c).or_default() += 1;
        m
    });
    let mut count = init.windows(2).fold(HashMap::<(char,char),usize>::new(),|mut m, w| {
        *m.entry((w[0], w[1])).or_default() += 1;
        m
    });

    for i in 0..12 {
        eprintln!("{} steps", i);
        let mut line = insert_elements(&pairs, (init[0], init[1]), 0, i as i32+1);
        init[1..init.len()].windows(2).for_each(|w| {
            line.extend(insert_elements(&pairs, (w[0], w[1]), 0, i as i32+1).iter().skip(1));
        });

        count = insert_elements_step(&pairs, &count, &mut freq);

        let test_freq = line.iter().fold(HashMap::<char, usize>::new(), |mut m, &c| {
            *m.entry(c).or_default() += 1;
            m
        });
        eprintln!("stepwise: {:?}", freq);
        eprintln!("recursive: {:?}", test_freq);
        assert_eq!(freq, test_freq);
    }
}

#[test]
fn test_day14_part1() {
    let init = TEST_INPUT_LINE.chars().collect::<Vec<_>>();
    let mut pairs = HashMap::new();
    TEST_INPUT_PAIRS.iter().for_each(|&(k,v)| { pairs.insert(k, v); });
    assert_eq!(part1(&init, &pairs), 1588);
}

#[test]
fn test_day14_part2() {
    let init = TEST_INPUT_LINE.chars().collect::<Vec<_>>();
    let mut pairs = HashMap::new();
    TEST_INPUT_PAIRS.iter().for_each(|&(k,v)| { pairs.insert(k, v); });
    assert_eq!(part2(&init, &pairs), 2188189693529);
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
    println!("{}", part1(&inputs.0, &inputs.1));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day {} part 2: ", DAY);
    part2(&inputs.0, &inputs.1);
    println!("{}", part2(&inputs.0, &inputs.1));
    println!("Took {:?}", pt_start.elapsed().unwrap());
    println!("Total time: {:?}", start.elapsed().unwrap());
}

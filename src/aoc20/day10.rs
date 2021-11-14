
use std::collections::HashMap;
use std::cmp::min;

#[allow(unused)]
pub fn input() -> Vec<i64> {
    let mut ret = crate::aoc::input_raw(20, 10)
        .lines()
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    ret.sort();
    ret
}


#[allow(unused)]
pub fn part1(inputs: &[i64]) -> i64 {
    let (o, t, _) = inputs.iter()
        .fold((0, 1, 0), |(ones, threes, prev), x| match x-prev {
            1 => (ones+1, threes, *x),
            3 => (ones, threes+1, *x),
            _ => (ones, threes, *x)
        });
    o * t
}


#[allow(unused)]
pub fn part2(inputs: &[i64]) -> i64 {
    let mut pathcount: HashMap::<i64, i64> = HashMap::new();
    pathcount.insert(0, 1);
    // First case, set adapters accessible from 0 to at least 1
    for i in 0..inputs.len()-1 {
        if inputs[i] > 3 { break; }
        pathcount.insert(inputs[i], 1);
    }
    for (i, a) in inputs.iter().enumerate() {
        let c = pathcount[a];
        for j in i+1..min(i+4, inputs.len()) {
            if inputs[j] - a > 3 { break; } // break inner loop
            let e = pathcount.entry(inputs[j]).or_insert(0);
            *e += c;
        }
    }
    pathcount[inputs.last().unwrap()]
}


#[test]
fn test_day10_part1() {
    let mut inputs = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    inputs.sort();
    assert_eq!(5*7, part1(&inputs));
    let mut inputs = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
    inputs.sort();
    assert_eq!(22*10, part1(&inputs));
}

#[test]
fn test_day10_part2() {
    let mut inputs = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
    inputs.sort();
    assert_eq!(8, part2(&inputs));
    let mut inputs = vec![28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35, 8, 17, 7, 9, 4, 2, 34, 10, 3];
    inputs.sort();
    assert_eq!(19208, part2(&inputs));
}

#[test]
fn run_day10() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 10 part 1: ");
    println!("{} - in {:?}", part1(&inputs), pt_start.elapsed().unwrap());
    print!("Day 10 part 2: ");
    let pt_start = SystemTime::now();
    println!("{} - in {:?}", part2(&inputs), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}

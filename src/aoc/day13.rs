
#[allow(unused)]
pub fn input() -> (i64, Vec<i64>) {
    let inputs = crate::aoc::input(13);
    (
        inputs[0].parse::<i64>().unwrap(), 
        inputs[1].split(",").map(|c|c.parse::<i64>().unwrap_or(0)).collect::<Vec<_>>()
    )
}

#[allow(unused)]
pub fn part1(inputs: &(i64, Vec<i64>)) -> i64 { // arr + t - arr % t
    let (i, t) = inputs.1
        .iter()
        .filter(|i| **i != 0)
        .map(|t| (t - inputs.0 % t, t))
        .fold((std::i64::MAX, 0), |(i, t), (j, u)| if j < i { (j, *u) } else { (i, t) });
    i*t
}

// Chinese remainder theorem (all bus numbers are prime numbers)
#[allow(unused)]
pub fn part2(inputs: &(i64, Vec<i64>)) -> i64 {
    let a = inputs.1
        .iter()
        .enumerate()
        .filter_map(|(i, v)| match v { 0=>None, _=>Some((i as i64 % *v, *v))})
        .collect::<Vec<(i64, i64)>>();
    println!("{:?}", a);
    let start: (i64, i64) = (0, inputs.0);
    a.iter()
    //.skip(1)
    .fold(start, |(a1, n1), (a2, n2)| {
        let (m1, m2) = extended_gcd(&n1, n2);
        let n = n1 * n2;
        let a12 = (((a2 * n1 * m1 + a1 * n2 * m2)%n) + n)%n;
        println!("{} % {} id={}, {} % {} id={} n1*n2={} a1,2={}", a1, n1, m1, a2, n2, m2, n, a12);
        (a12, n)
    }).0
    //.for_each(|w| println!("{} {}: {:?}", w[0].1, w[1].1, extended_gcd(w[0].1, w[1].1)));
    
}

fn extended_gcd(a: &i64, b: &i64) -> (i64, i64) {
    let (mut old_r, mut r) = (*a, *b);
    let (mut old_s, mut s) = (1, 0);
    let (mut old_t, mut t) = (0, 1);
    while r != 0 {
        let q = old_r / r;
        let mut b = r;
        r = old_r - q*r;
        old_r = b;
        b = s;
        s = old_s - q*s;
        old_s = b;
        b = t;
        t = old_t - q*t;
        old_t = b;
    }
    (old_s, old_t)
}

#[test]
fn test_day13_part1() {
    let inputs = ["939", "7,13,x,x,59,x,31,19"];
    let inputs = (
        inputs[0].parse::<i64>().unwrap(), 
        inputs[1].split(",").map(|c|c.parse::<i64>().unwrap_or(0)).filter(|i| *i != 0).collect::<Vec<_>>()
    );
    assert_eq!(295, part1(&inputs));
}

#[test]
fn test_day13_part2_0() {
    let inputs = (1, vec![2,5]);
    assert_eq!(4, part2(&inputs));
}

#[test]
fn test_day13_part2_1() {
    let inputs = (939, vec![7, 0,13,19]);
    assert_eq!(3417, part2(&inputs));
}

#[test]
fn test_day13_part2_2() {
    let inputs = (939, vec![7,13,0,0,59,0,31,19]);
    assert_eq!(1068781, part2(&inputs));
}

#[test]
fn test_day13_part2_3() {
    let inputs = (939, vec![67, 7, 59, 61]);
    assert_eq!(754018, part2(&inputs));
}

#[test]
fn test_day13_part2_4() {
    let inputs = (939, vec![67, 0, 7, 59, 61]);
    assert_eq!(779210, part2(&inputs));
}

#[test]
fn test_day13_part2_5() {
    println!("{:?} {:?}", extended_gcd(&3, &4), extended_gcd(&12, &5));
    let inputs = (1, vec![3, 0, 0, 4, 5]);
    assert_eq!(39, part2(&inputs));
}

#[test]
fn run_day13() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 13 part 1: ");
    println!("{} - in {:?}", part1(&inputs), pt_start.elapsed().unwrap());
    print!("Day 13 part 2: ");
    let pt_start = SystemTime::now();
    println!("{} - in {:?}", part2(&inputs), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}
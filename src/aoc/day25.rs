extern crate num;


#[allow(unused)]
pub fn input() -> (usize, usize) {
    let i = crate::aoc::input(25);
    (
        i[0].parse().unwrap(),
        i[1].parse().unwrap()
    )
}

// subj^lsize % 20201227
// subj = 7, lsize = secret
fn transform(subj: usize, lsize: usize) -> usize {
    //pow(subj, lsize) % 20201227
    (0..lsize).fold(1, |acc,_| (acc*subj) % 20201227)
}



fn brute(pkey: usize) -> usize {
    let mut acc = 1;
    for i in 1.. {
        acc = (acc * 7) % 20201227;
        if pkey == acc { return i }
    }
    acc
    //(2..).find(|i|{let x = transform(7, *i); println!("{}: {}", i, x); x==pkey}).unwrap()
    //iter::split(2..20201227, split_range1).
    //(2..202012usize).into_par_iter().find_any(|i|{let x = transform(7, *i); println!("{}: {}", i, x); x==pkey}).unwrap()
}

fn brute_encryption_key(pkeyd: usize, pkeyc: usize) -> usize {
    let ld = brute(pkeyd);
    println!("Found key: {}", ld);
    transform(pkeyc, ld)
}

#[allow(unused)]
pub fn part1(inputs: (usize, usize)) -> usize {
    brute_encryption_key(inputs.1, inputs.0)
}


#[test]
fn test_day25_brute_1() {
    let pkey = 5764801;
    assert_eq!(8, brute(pkey));
}

#[test]
fn test_day25_brute_2() {
    let pkey = 17807724;
    assert_eq!(11, brute(pkey));
}

#[test]
fn test_day25_part1() {
    let pkeyd = 17807724;
    let pkeyc = 5764801;
    let ld = brute(pkeyd);
    assert_eq!(14897079, transform(pkeyc, ld));
    let lc = brute(pkeyc);
    assert_eq!(14897079, transform(pkeyd, lc));
    assert_eq!(14897079, part1((pkeyd, pkeyc)));
}


#[test]
fn run_day25() {
    use std::time::SystemTime;
    let start = SystemTime::now();
    print!("Parsing input . . . ");
    let inputs = input();
    println!("{:?}", start.elapsed().unwrap());
    let pt_start = SystemTime::now();
    print!("Day 25 part 1: ");
    println!("{:?} - in {:?}", part1(inputs), pt_start.elapsed().unwrap());
    // print!("Day 25 part 2: ");
    // let pt_start = SystemTime::now();
    // println!("{} - in {:?}", part2(&inputs), pt_start.elapsed().unwrap());
    println!("Total duration: {:?}", start.elapsed().unwrap())
}
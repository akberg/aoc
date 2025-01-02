use super::YEAR;
static DAY: usize = 22;

fn input() -> Vec<usize> {
    crate::aoc::input_raw(YEAR, DAY)
        .lines()
        .map(|ls| ls.trim().parse::<_>().unwrap())
        .collect()
}


fn mix(a: usize, b: usize) -> usize {
    a ^ b
}
fn prune(a: usize) -> usize {
    a.rem_euclid(16777216)
}

fn rand_next(secret: usize) -> usize {
    let secret = prune(mix(secret, secret * 64));
    let secret = prune(mix(secret, secret / 32));
    let secret = prune(mix(secret, secret * 2048));
    secret
}
fn evolve(secret: usize, n: usize) -> usize {
    (0..n).fold(secret, |s, _i| rand_next(s))
}

fn part1(inputs: &Vec<usize>) -> usize {
    inputs.iter().map(|&s| evolve(s, 2000)).sum()
}

fn part2(inputs: &Vec<usize>) -> u32 {
    todo!();
}

#[test]
fn test_2024_day22_part1() {
    let test_inputs = vec![
        1,
        10,
        100,
        2024,
    ];
    assert_eq!(mix(42, 15), 37);
    assert_eq!(prune(100000000), 16113920);
    assert_eq!(rand_next(123), 15887950);
    assert_eq!(rand_next(15887950), 16495136);

    assert_eq!(evolve(test_inputs[0], 2000), 8685429);
    assert_eq!(evolve(test_inputs[1], 2000), 4700978);
    assert_eq!(evolve(test_inputs[2], 2000), 15273692);
    assert_eq!(evolve(test_inputs[3], 2000), 8667524);
    assert_eq!(part1(&test_inputs), 37327623);
}

#[test]
fn test_2024_day22_part2() {
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



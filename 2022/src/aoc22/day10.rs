static DAY: usize = 10;

pub fn input() -> String {
    crate::aoc::input_raw(1)
        //.lines()
        //.map(|ls| ls.parse::<_>().unwrap())
        //.collect()
}

pub fn part1(inputs: &str) -> u32 {
    todo!();
    0
}

pub fn part2(inputs: &str) -> u32 {
    todo!();
    0
}

#[test]
fn test_day10_part1() {
    // TODO
}

#[test]
fn test_day10_part2() {
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



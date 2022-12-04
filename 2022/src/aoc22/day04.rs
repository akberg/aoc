static DAY: usize = 04;

pub fn input() -> String {
    crate::aoc::input_raw(4)
        // .lines()
        // .map(|ls| ls.parse::<_>().unwrap())
        // .collect()
}

pub fn part1(inputs: &str) -> u32 {
    inputs.lines()
    .map(|line| sscanf::sscanf!(line, "{}-{},{}-{}", usize,usize,usize,usize).unwrap())
    .filter(|(a0,a1,b0,b1)| (a0<=b0&&b1<=a1) || (b0<=a0&&a1<=b1))
    .count() as u32
}

pub fn part2(inputs: &str) -> u32 {
    inputs.lines()
    .map(|line| sscanf::sscanf!(line, "{}-{},{}-{}", usize,usize,usize,usize).unwrap())
    .filter(|(a0,a1,b0,b1)|
        (a0..=a1).contains(&b0) ||
        (a0..=a1).contains(&b1) ||
        (b0..=b1).contains(&a0) ||
        (b0..=b1).contains(&a1)
    )
    .count() as u32
}

#[test]
fn test_day4_part1() {
    let inputs = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    assert_eq!(part1(inputs), 2);
}

#[test]
fn test_day4_part2() {
    let inputs = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";
    assert_eq!(part2(inputs), 4);
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



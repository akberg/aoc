static DAY: i32 = 16;

pub fn input() -> (std::ops::Range<isize>, std::ops::Range<isize>) {
    let (rx, ry) = (34..67, -215..-186);
    rx.contains(&40);
    (rx, ry)
}

/// Find the trajectory with the highest max y that hits the target
pub fn part1(inputs: &str) -> u64 {
    // x(t) = x'(t) * t
    // x'(t) = x'(0) + t * if x'(0) > 0 { -1 } else { 1 }
    // y(t) 
    0
}


pub fn part2(inputs: &str) -> u64 {
    0
}

/* TESTS */
#[allow(unused)]
static TEST_INPUT: &'static [&str] = &[
    "D2FE28",
];

#[test]
fn test_day16_part1() {
    assert_eq!(part1(&parse_line(TEST_INPUT[2])), 16);
}

#[test]
fn test_day16_part2() {
    assert_eq!(part2(&parse_line(TEST_INPUT[6])), 3);
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

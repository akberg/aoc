use super::YEAR;
static DAY: usize = 01;
// Keywords: Modulo

fn parse_input(input: &str) -> i32 {
    let mut a = input.trim().chars();
    let dir = a.next().unwrap();
    let n = a.as_str().parse::<i32>().unwrap();
    return match dir { 'R' => n, 'L' => -n, _ => unreachable!() }
}

fn input() -> Vec<i32> {
    crate::aoc::input_raw(YEAR, DAY)
        .lines()
        .map(parse_input)
        .collect()
}

fn part1(inputs: &Vec<i32>) -> i32 {
    let mut i = 50i32;
    let mut res = 0;
    for n in inputs {
        i = (((i + n) % 100) + 100) % 100;
        if i == 0 { res += 1; }
    }
    return res
}

// Goddamn modulo never being implemented as one would expect
fn part2(inputs: &Vec<i32>) -> i32 {
    let mut i = 50i32;
    let mut res = 0;
    println!("start");
    for &n in inputs {
        let sign = n.signum(); // Sign, sets direction
        let mut rem = n;
        // print!("{}, Rot {}, {} -> ", i, n, res);
        while rem.abs() > 100 {
            rem -= 100 * sign;
            res += 1;
        }
        if i != 0 && (i + rem >= 100 || i + rem <= 0) {
            res += 1;
        }
        // println!("{} ({} || {})", res, i != 0 && i + rem >= 100, i != 0 && i + rem <= 0);
        i = (((i + n) % 100) + 100) % 100;
    }
    return res
}
// 6649 too high
// 5398 too low
// 6103 too low

#[test]
fn test_2025_day1_part1() {
    let input = "L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82".lines().map(parse_input).collect::<Vec<_>>();
    assert_eq!(part1(&input), 3);
}

#[test]
fn test_2025_day1_part2() {
    let input = "L68
    L30
    R48
    L5
    R60
    L55
    L1
    L99
    R14
    L82".lines().map(parse_input).collect::<Vec<_>>();
    assert_eq!(part2(&input), 6);
    assert_eq!(part2(&vec![1000]), 10);
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



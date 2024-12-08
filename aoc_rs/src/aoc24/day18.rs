use super::YEAR;
static DAY: usize = 18;

fn input() -> String {
    crate::aoc::input_raw(YEAR, DAY)
        //.lines()
        //.map(|ls| ls.parse::<_>().unwrap())
        //.collect()
}

fn part1(_inputs: &str) -> u32 {
    todo!();
}

fn part2(_inputs: &str) -> u32 {
    todo!();
}

#[test]
fn test_2024_day18_part1() {
    // TODO
}

#[test]
fn test_2024_day18_part2() {
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



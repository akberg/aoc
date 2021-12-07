static DAY: i32 = 7;

pub fn input() -> Vec<i64> {
    use std::io::{prelude::*, BufReader};
    let f = crate::aoc::input_file(DAY).unwrap();
    let f = BufReader::new(f);
    f.lines().next().unwrap().unwrap()
        .split(",")
        .map(|n| n.parse::<_>().unwrap())
        .collect::<_>()

}


/**Calculate cost of aligning to median */
pub fn part1(inputs: &Vec<i64>) -> i64 {
    let mut inputs = inputs.clone();
    inputs.sort();
    let m = inputs[inputs.len()/2];

    inputs.iter().map(|p| (p - m).abs()).sum::<_>()
}

/**Calculate cost of alignment to average. Puzzle input gives a float
 * imprecission causing rounding to give the wrong value. Problem in
 * my code or in Rust?
 */
pub fn part2(inputs: &Vec<i64>) -> i64 {
    let avg = (inputs.iter().sum::<i64>() as f64 / inputs.len() as f64).round() as i64;
    println!("{} ({})", avg, inputs.iter().sum::<i64>() as f64/ inputs.len() as f64);
    let avg = inputs.iter().sum::<i64>() / inputs.len() as i64;
    println!("{}", avg);

    inputs.iter().map(|p| (0..=(p - avg).abs() as usize).sum::<usize>() as i64).sum::<i64>()
}


/* TESTS */
#[allow(unused)]
static TEST_NUMBERS: &'static [i64] = &[16,1,2,0,4,2,7,1,2,14];

#[test]
fn test_day07_part1() {
    let inputs = Vec::from(TEST_NUMBERS);
    assert_eq!(part1(&inputs), 37);
}

#[test]
fn test_day07_part2() {
    let inputs = Vec::from(TEST_NUMBERS);
    assert_eq!(part2(&inputs), 168);
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
